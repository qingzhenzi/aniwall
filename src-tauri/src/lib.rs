mod catalog;
pub mod cli;
mod download;
mod library;
mod model;
mod paths;
mod plasma;
mod system;

use model::{CatalogItem, EnvStatus, Settings, Wallpaper};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::menu::{CheckMenuItem, Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager};

const APP_NAME: &str = "AniWall";

static TRAY_READY: AtomicBool = AtomicBool::new(false);

fn cookie_path(settings: &Settings) -> Option<PathBuf> {
    let p = settings.cookie_file.trim();
    if p.is_empty() {
        None
    } else {
        Some(PathBuf::from(p))
    }
}

/* ============================ 窗口 / 托盘 ============================ */

fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

fn toggle_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let visible = window.is_visible().unwrap_or(false);
        let focused = window.is_focused().unwrap_or(false);
        if visible && focused {
            let _ = window.hide();
        } else {
            show_main_window(app);
        }
    }
}

fn update_tray_tooltip(app: &AppHandle, title: &str) {
    if let Some(tray) = app.tray_by_id("main") {
        let _ = tray.set_tooltip(Some(format!("{APP_NAME} · {title}")));
    }
}

/* ============================ 开机自启 ============================ */

fn autostart_file() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("autostart/aniwall.desktop")
}

pub fn autostart_enabled() -> bool {
    autostart_file().is_file()
}

pub fn set_autostart_impl(enabled: bool) -> Result<String, String> {
    let path = autostart_file();
    if !enabled {
        if path.exists() {
            std::fs::remove_file(&path).map_err(|e| format!("删除自启动项失败：{e}"))?;
        }
        return Ok("已关闭开机自启".into());
    }
    let exe = std::env::current_exe().map_err(|e| format!("获取程序路径失败：{e}"))?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("创建目录失败：{e}"))?;
    }
    let content = format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name={APP_NAME}\n\
         Comment=KDE 动态壁纸管理器（开机静默启动到托盘）\n\
         Exec=\"{}\" --minimized\n\
         Icon=aniwall\n\
         Terminal=false\n\
         X-GNOME-Autostart-enabled=true\n\
         NoDisplay=true\n",
        exe.display()
    );
    std::fs::write(&path, content).map_err(|e| format!("写入自启动项失败：{e}"))?;
    Ok("已开启开机自启（启动后静默到托盘）".into())
}

/* ============================ 壁纸切换（共享实现） ============================ */

fn apply_item(item: &Wallpaper) -> Result<String, String> {
    let mut settings = library::read_settings();
    let msg = plasma::set_video(std::path::Path::new(&item.path), &settings)?;
    settings.active_id = item.id.clone();
    library::write_settings(&settings)?;
    Ok(format!("已应用《{}》 · {msg}", item.title))
}

pub fn random_wallpaper_impl() -> Result<String, String> {
    let items = library::load();
    if items.is_empty() {
        return Err("壁纸库为空，先下载几张吧".into());
    }
    let current = library::read_settings().active_id;
    let pool: Vec<&Wallpaper> = items.iter().filter(|w| w.id != current).collect();
    let pool = if pool.is_empty() { items.iter().collect() } else { pool };
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as usize)
        .unwrap_or(0);
    let item = pool[nanos % pool.len()];
    apply_item(item)
}

pub fn next_wallpaper_impl() -> Result<String, String> {
    let mut items = library::load();
    items.sort_by(|a, b| a.added_at.cmp(&b.added_at));
    if items.is_empty() {
        return Err("壁纸库为空，先下载几张吧".into());
    }
    let current = library::read_settings().active_id;
    let index = items.iter().position(|w| w.id == current).unwrap_or(items.len() - 1);
    let item = &items[(index + 1) % items.len()];
    apply_item(item)
}

pub fn restore_impl() -> Result<String, String> {
    let mut settings = library::read_settings();
    let msg = system::restore_static_wallpaper()?;
    settings.active_id.clear();
    let _ = library::write_settings(&settings);
    Ok(msg)
}

/* ============================ 托盘菜单 ============================ */

fn build_tray_menu(app: &AppHandle) -> tauri::Result<Menu<tauri::Wry>> {
    let show = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
    let random = MenuItem::with_id(app, "random", "随机换一张", true, None::<&str>)?;
    let next = MenuItem::with_id(app, "next", "下一张", true, None::<&str>)?;
    let restore = MenuItem::with_id(app, "restore", "恢复静态壁纸", true, None::<&str>)?;
    let autostart = CheckMenuItem::with_id(
        app,
        "autostart",
        "开机自启（静默到托盘）",
        true,
        autostart_enabled(),
        None::<&str>,
    )?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    Menu::with_items(
        app,
        &[&show, &random, &next, &restore, &autostart, &quit],
    )
}

fn refresh_tray_menu(app: &AppHandle) {
    if let Some(tray) = app.tray_by_id("main") {
        if let Ok(menu) = build_tray_menu(app) {
            let _ = tray.set_menu(Some(menu));
        }
    }
}

fn init_tray(app: &AppHandle) -> tauri::Result<()> {
    let menu = build_tray_menu(app)?;

    let mut builder = TrayIconBuilder::with_id("main")
        .tooltip(format!("{APP_NAME} · 动态壁纸管理器"))
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show" => show_main_window(app),
            "random" => {
                if let Ok(message) = random_wallpaper_impl() {
                    let title = library::read_settings().active_id;
                    update_tray_tooltip(app, &title);
                    let _ = app.emit("wallpaper-changed", message);
                }
            }
            "next" => {
                if let Ok(message) = next_wallpaper_impl() {
                    let title = library::read_settings().active_id;
                    update_tray_tooltip(app, &title);
                    let _ = app.emit("wallpaper-changed", message);
                }
            }
            "restore" => {
                let _ = restore_impl();
                let _ = app.emit("wallpaper-changed", "已恢复静态壁纸".to_string());
            }
            "autostart" => {
                let next_state = !autostart_enabled();
                let _ = set_autostart_impl(next_state);
                refresh_tray_menu(app);
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                toggle_main_window(tray.app_handle());
            }
        });

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    builder.build(app)?;
    TRAY_READY.store(true, Ordering::SeqCst);
    Ok(())
}

/* ============================ 命令 ============================ */

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
fn set_wallpaper(app: AppHandle, id: String) -> Result<String, String> {
    let items = library::load();
    let item = items
        .into_iter()
        .find(|w| w.id == id)
        .ok_or_else(|| "壁纸不存在（可能已被删除）".to_string())?;
    let message = apply_item(&item)?;
    update_tray_tooltip(&app, &item.title);
    Ok(message)
}

#[tauri::command]
fn random_wallpaper() -> Result<String, String> {
    random_wallpaper_impl()
}

#[tauri::command]
fn next_wallpaper() -> Result<String, String> {
    next_wallpaper_impl()
}

#[tauri::command]
fn restore_wallpaper() -> Result<String, String> {
    restore_impl()
}

#[tauri::command]
fn import_files(paths: Vec<String>) -> Result<Vec<Wallpaper>, String> {
    let mut added = Vec::new();
    for p in paths {
        let path = PathBuf::from(&p);
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
fn get_autostart() -> bool {
    autostart_enabled()
}

#[tauri::command]
fn set_autostart(app: AppHandle, enabled: bool) -> Result<String, String> {
    let message = set_autostart_impl(enabled)?;
    refresh_tray_menu(&app);
    Ok(message)
}

#[tauri::command]
fn hide_window(app: AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
}

#[tauri::command]
fn app_info() -> serde_json::Value {
    serde_json::json!({
        "name": APP_NAME,
        "version": env!("CARGO_PKG_VERSION"),
        "exe": std::env::current_exe().map(|p| p.display().to_string()).unwrap_or_default(),
        "autostart": autostart_enabled(),
        "tray": TRAY_READY.load(Ordering::SeqCst),
        "tested_on": "CachyOS / KDE Plasma 6 / Wayland",
    })
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

/* ============================ 启动 ============================ */

pub fn run() {
    let _ = paths::ensure_dirs();
    let minimized = std::env::args().any(|arg| arg == "--minimized");

    tauri::Builder::default()
        .on_page_load(|_webview, payload| {
            eprintln!("[aniwall] page load: {:?} {}", payload.event(), payload.url());
        })
        .setup(move |app| {
            let handle = app.handle().clone();
            match init_tray(&handle) {
                Ok(()) => eprintln!("[aniwall] tray ready"),
                Err(err) => eprintln!("[aniwall] tray init failed: {err}"),
            }
            if minimized && TRAY_READY.load(Ordering::SeqCst) {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                    eprintln!("[aniwall] started minimized to tray");
                }
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if TRAY_READY.load(Ordering::SeqCst) {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            env_status,
            ui_ready,
            ui_log,
            list_wallpapers,
            current_wallpaper,
            set_wallpaper,
            random_wallpaper,
            next_wallpaper,
            restore_wallpaper,
            import_files,
            delete_wallpaper,
            read_settings,
            save_settings,
            install_plugin,
            install_deps,
            install_shortcut,
            get_autostart,
            set_autostart,
            hide_window,
            app_info,
            catalog,
            open_folder,
            download_urls,
        ])
        .run(tauri::generate_context!())
        .expect("AniWall 启动失败");
}
