use crate::model::Wallpaper;
use crate::paths;
use crate::system;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Default, Serialize, Deserialize)]
struct LibraryFile {
    #[serde(default)]
    version: u32,
    #[serde(default)]
    items: Vec<Wallpaper>,
}

pub fn load() -> Vec<Wallpaper> {
    let text = std::fs::read_to_string(paths::library_file()).unwrap_or_default();
    let mut lib: LibraryFile = serde_json::from_str(&text).unwrap_or_default();
    lib.items.retain(|w| Path::new(&w.path).exists());
    lib.items.sort_by(|a, b| b.added_at.cmp(&a.added_at));
    lib.items
}

pub fn save(items: &[Wallpaper]) -> Result<(), String> {
    let lib = LibraryFile {
        version: 1,
        items: items.to_vec(),
    };
    let text = serde_json::to_string_pretty(&lib).map_err(|e| e.to_string())?;
    std::fs::write(paths::library_file(), text).map_err(|e| e.to_string())
}

fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

fn slug(title: &str) -> String {
    let mut out = String::new();
    for ch in title.chars() {
        if ch.is_alphanumeric() {
            out.push(ch.to_lowercase().next().unwrap_or(ch));
        } else if ch == ' ' || ch == '-' || ch == '_' {
            if !out.ends_with('-') {
                out.push('-');
            }
        }
        if out.chars().count() >= 48 {
            break;
        }
    }
    let trimmed = out.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "wallpaper".to_string()
    } else {
        trimmed
    }
}

fn poster_path_for(id: &str) -> std::path::PathBuf {
    paths::posters_dir().join(format!("{id}.jpg"))
}

/// 用 ffprobe 读取视频时长与分辨率。
pub fn probe(path: &Path) -> (f64, u32, u32) {
    let args = [
        "-v",
        "error",
        "-select_streams",
        "v:0",
        "-show_entries",
        "stream=width,height",
        "-show_entries",
        "format=duration",
        "-of",
        "json",
        &path.display().to_string(),
    ];
    let Ok(text) = system::run("ffprobe", &args) else {
        return (0.0, 0, 0);
    };
    let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else {
        return (0.0, 0, 0);
    };
    let duration = json
        .pointer("/format/duration")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);
    let width = json
        .pointer("/streams/0/width")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;
    let height = json
        .pointer("/streams/0/height")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;
    (duration, width, height)
}

/// 生成封面缩略图（失败返回 false，不影响主流程）。
pub fn make_poster(video: &Path, id: &str) -> Option<String> {
    let poster = poster_path_for(id);
    for seek in ["2", "0"] {
        let ok = system::run(
            "ffmpeg",
            &[
                "-v",
                "error",
                "-y",
                "-ss",
                seek,
                "-i",
                &video.display().to_string(),
                "-frames:v",
                "1",
                "-vf",
                "scale=640:-2",
                "-q:v",
                "3",
                &poster.display().to_string(),
            ],
        )
        .is_ok();
        if ok && poster.exists() {
            return Some(poster.display().to_string());
        }
    }
    None
}

/// 把本地视频导入壁纸库（已在库里则直接返回）。
pub fn add_from_file(src: &Path, source: &str) -> Result<Wallpaper, String> {
    if !src.exists() {
        return Err(format!("文件不存在：{}", src.display()));
    }
    paths::ensure_dirs().map_err(|e| e.to_string())?;

    let wall_dir = paths::wallpapers_dir();
    let dest = if src.parent() == Some(wall_dir.as_path()) {
        src.to_path_buf()
    } else {
        let name = src
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "wallpaper.mp4".into());
        let dest = wall_dir.join(name);
        if src != dest {
            std::fs::copy(src, &dest).map_err(|e| format!("复制文件失败：{e}"))?;
        }
        dest
    };

    let title = dest
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "未命名壁纸".into());
    let id = format!("{}-{:x}", slug(&title), now_millis());
    let (duration, width, height) = probe(&dest);
    let size_bytes = std::fs::metadata(&dest).map(|m| m.len()).unwrap_or(0);
    let poster = make_poster(&dest, &id);

    let item = Wallpaper {
        id,
        title,
        path: dest.display().to_string(),
        poster,
        size_bytes,
        duration_secs: duration,
        width,
        height,
        source: source.to_string(),
        added_at: now_millis(),
    };

    let mut items = load();
    items.push(item.clone());
    save(&items)?;
    Ok(item)
}

pub fn delete(id: &str) -> Result<(), String> {
    let mut items = load();
    if let Some(pos) = items.iter().position(|w| w.id == id) {
        let item = items.remove(pos);
        let _ = std::fs::remove_file(&item.path);
        if let Some(poster) = &item.poster {
            let _ = std::fs::remove_file(poster);
        }
        save(&items)?;
    }
    Ok(())
}

/// 读取设置（不存在时返回默认值）
pub fn read_settings() -> crate::model::Settings {
    let text = std::fs::read_to_string(paths::settings_file()).unwrap_or_default();
    serde_json::from_str(&text).unwrap_or_default()
}

pub fn write_settings(settings: &crate::model::Settings) -> Result<(), String> {
    paths::ensure_dirs().map_err(|e| e.to_string())?;
    let text = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    std::fs::write(paths::settings_file(), text).map_err(|e| e.to_string())
}

/// 扫描壁纸目录，把不在库里的视频补录进去（手动往目录里丢文件也能识别）。
pub fn scan_import() -> Vec<Wallpaper> {
    let dir = paths::wallpapers_dir();
    let known: Vec<String> = load().into_iter().map(|w| w.path).collect();
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut added = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());
        let known_ext = matches!(ext.as_deref(), Some("mp4" | "webm" | "mkv" | "mov"));
        if known_ext && !known.contains(&path.display().to_string()) {
            if let Ok(item) = add_from_file(&path, "手动放入") {
                added.push(item);
            }
        }
    }
    added
}
