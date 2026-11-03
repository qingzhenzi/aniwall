use std::path::PathBuf;

/// 所有数据放在 XDG 目录下：
///   `~/.local/share/aniwall/`  -> 壁纸库、封面、library.json
///   `~/.config/aniwall/`       -> settings.json
pub fn data_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("aniwall")
}

pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("aniwall")
}

pub fn wallpapers_dir() -> PathBuf {
    data_dir().join("wallpapers")
}

pub fn posters_dir() -> PathBuf {
    data_dir().join("posters")
}

pub fn library_file() -> PathBuf {
    data_dir().join("library.json")
}

pub fn settings_file() -> PathBuf {
    config_dir().join("settings.json")
}

/// Smart Video Wallpaper Reborn 插件安装目录（用户级）
pub fn plugin_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("plasma/wallpapers/luisbocanegra.smart.video.wallpaper.reborn")
}

pub fn system_plugin_dir() -> PathBuf {
    PathBuf::from("/usr/share/plasma/wallpapers/luisbocanegra.smart.video.wallpaper.reborn")
}

pub fn ensure_dirs() -> std::io::Result<()> {
    std::fs::create_dir_all(wallpapers_dir())?;
    std::fs::create_dir_all(posters_dir())?;
    std::fs::create_dir_all(config_dir())?;
    Ok(())
}
