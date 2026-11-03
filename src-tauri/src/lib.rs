mod catalog;
pub mod cli;
mod download;
mod library;
mod model;
mod paths;
mod plasma;
mod system;

use model::{CatalogItem, EnvStatus, Settings, Wallpaper};
use tauri::Emitter;

fn cookie_path(settings: &Settings) -> Option<std::path::PathBuf> {
    let p = settings.cookie_file.trim();
    if p.is_empty() {
        None
    } else {
        Some(std::path::PathBuf::from(p))
    }
}

#[tauri::command]
fn ui_log(message: String) -> Result<(), String> {
    let _ = crate::paths::ensure_dirs();
    let path = crate::paths::data_dir().join("ui.log");
    let line = format!("[{}] {}\n", std::process::id(), message);
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    file.write_all(line.as_bytes()).map_err(|e| e.to_string())
}

#[tauri::command]
fn ui_ready() -> Result<String, String> {
    // 前端初始化完成后写一个心跳文件，便于排障（也用于 CI / 冒烟测试）
    let _ = crate::paths::ensure_dirs();
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let path = crate::paths::data_dir().join("last_ui_ready");
    std::fs::write(&path, stamp.to_string()).map_err(|e| e.to_string())?;
    Ok("ok".into())
}

#[tauri::command]
fn env_status() -> EnvStatus {
    system::env_status()
}

#[tauri::command]
fn list_wallpapers() -> Vec<Wallpaper> {
    let _ = library::scan_import();
    library::load()
}

#[tauri::command]
fn current_wallpaper() -> serde_json::Value {
    plasma::current()
}

#[tauri::command]
fn set_wallpaper(id: String) -> Result<String, String> {
    let items = library::load();
    let item = items
        .into_iter()
        .find(|w| w.id == id)
        .ok_or_else(|| "壁纸不存在（可能已被删除）".to_string())?;
    let mut settings = library::read_settings();
    let msg = plasma::set_video(std::path::Path::new(&item.path), &settings)?;
    settings.active_id = item.id.clone();
    library::write_settings(&settings)?;
    Ok(format!("已应用《{}》 · {msg}", item.title))
}

#[tauri::command]
fn restore_wallpaper() -> Result<String, String> {
    let mut settings = library::read_settings();
    let msg = system::restore_static_wallpaper()?;
    settings.active_id.clear();
    let _ = library::write_settings(&settings);
    Ok(msg)
}

#[tauri::command]
fn import_files(paths: Vec<String>) -> Result<Vec<Wallpaper>, String> {
    let mut added = Vec::new();
    for p in paths {
        let path = std::path::PathBuf::from(&p);
        match library::add_from_file(&path, &p) {
            Ok(item) => added.push(item),
            Err(e) => eprintln!("导入 {p} 失败：{e}"),
        }
    }
    Ok(added)
}

#[tauri::command]
fn delete_wallpaper(id: String) -> Result<(), String> {
    library::delete(&id)
}

#[tauri::command]
fn read_settings() -> Settings {
    library::read_settings()
}

#[tauri::command]
fn save_settings(settings: Settings) -> Result<String, String> {
    library::write_settings(&settings)?;
    if !settings.active_id.is_empty() {
        if let Some(item) = library::load().into_iter().find(|w| w.id == settings.active_id) {
            let _ = plasma::set_video(std::path::Path::new(&item.path), &settings);
        }
    }
    Ok("设置已保存".into())
}

#[tauri::command]
fn install_plugin() -> Result<String, String> {
    system::install_plugin()
}

#[tauri::command]
fn install_deps() -> Result<String, String> {
    system::install_deps()
}

#[tauri::command]
fn install_shortcut() -> Result<String, String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    system::install_desktop_entry(&exe)
}

#[tauri::command]
fn catalog() -> Vec<CatalogItem> {
    catalog::items()
}

#[tauri::command]
fn open_folder() -> Result<String, String> {
    let dir = paths::wallpapers_dir();
    let _ = system::run("xdg-open", &[dir.to_string_lossy().as_ref()]);
    Ok(dir.display().to_string())
}

#[tauri::command]
async fn download_urls(app: tauri::AppHandle, urls: Vec<String>) -> Result<Vec<Wallpaper>, String> {
    if urls.is_empty() {
        return Err("没有需要下载的地址".into());
    }
    let settings = library::read_settings();
    let cookie = cookie_path(&settings);
    let total = urls.len();
    let app2 = app.clone();

    let handle = tauri::async_runtime::spawn_blocking(move || {
        let mut added: Vec<Wallpaper> = Vec::new();
        let dir = paths::wallpapers_dir();
        for (index, raw) in urls.iter().enumerate() {
            let url = catalog::normalize_url(raw);
            let _ = app2.emit(
                "download-progress",
                serde_json::json!({
                    "index": index, "total": total, "url": url,
                    "percent": 0.0, "status": "start", "message": "开始解析"
                }),
            );

            let app3 = app2.clone();
            let progress_url = url.clone();
            let mut on_progress = move |percent: f64, message: String| {
                let _ = app3.emit(
                    "download-progress",
                    serde_json::json!({
                        "index": index, "total": total, "url": progress_url,
                        "percent": percent, "status": "downloading", "message": message
                    }),
                );
            };

            match download::download(&url, &dir, cookie.as_deref(), &mut on_progress) {
                Ok(file) => match library::add_from_file(&file, &url) {
                    Ok(item) => {
                        let _ = app2.emit(
                            "download-progress",
                            serde_json::json!({
                                "index": index, "total": total, "url": url,
                                "percent": 100.0, "status": "done", "message": item.title
                            }),
                        );
                        added.push(item);
                    }
                    Err(e) => {
                        let _ = app2.emit(
                            "download-progress",
                            serde_json::json!({
                                "index": index, "total": total, "url": url,
                                "percent": 0.0, "status": "error", "message": format!("入库失败：{e}")
                            }),
                        );
                    }
                },
                Err(e) => {
                    let _ = app2.emit(
                        "download-progress",
                        serde_json::json!({
                            "index": index, "total": total, "url": url,
                            "percent": 0.0, "status": "error", "message": e
                        }),
                    );
                }
            }
        }
        added
    });

    let added = handle.await.map_err(|e| format!("下载线程异常：{e}"))?;
    let _ = app.emit("library-updated", serde_json::json!({}));
    Ok(added)
}

pub fn run() {
    let _ = paths::ensure_dirs();
    tauri::Builder::default()
        .on_page_load(|_webview, payload| {
            eprintln!("[aniwall] page load: {:?} {}", payload.event(), payload.url());
        })
        .setup(|_app| {
            eprintln!("[aniwall] setup complete");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            env_status,
            ui_ready,
            ui_log,
            list_wallpapers,
            current_wallpaper,
            set_wallpaper,
            restore_wallpaper,
            import_files,
            delete_wallpaper,
            read_settings,
            save_settings,
            install_plugin,
            install_deps,
            install_shortcut,
            catalog,
            open_folder,
            download_urls,
        ])
        .run(tauri::generate_context!())
        .expect("AniWall 启动失败");
}
