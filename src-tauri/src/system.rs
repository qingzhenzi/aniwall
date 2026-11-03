use crate::model::{EnvStatus, ToolInfo};
use crate::paths;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const PLUGIN_ID: &str = "luisbocanegra.smart.video.wallpaper.reborn";
pub const PLUGIN_VERSION: &str = "2.13.0";
pub const PLUGIN_URL: &str =
    "https://github.com/luisbocanegra/plasma-smart-video-wallpaper-reborn/archive/refs/tags/v2.13.0.tar.gz";

/// 执行命令，成功返回 stdout（去掉尾部空白），失败返回错误信息。
pub fn run(cmd: &str, args: &[&str]) -> Result<String, String> {
    let out = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| format!("无法执行 `{cmd}`：{e}"))?;
    let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if out.status.success() {
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
        Err(if stderr.is_empty() {
            format!("`{cmd}` 退出码 {:?}", out.status.code())
        } else {
            stderr
        })
    }
}

pub fn cmd_exists(cmd: &str) -> bool {
    if cmd.contains('/') {
        return Path::new(cmd).exists();
    }
    std::env::var_os("PATH")
        .map(|paths| std::env::split_paths(&paths).any(|dir| dir.join(cmd).exists()))
        .unwrap_or(false)
}

pub fn version_of(cmd: &str, args: &[&str]) -> Option<String> {
    run(cmd, args).ok().and_then(|s| {
        s.lines()
            .next()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
    })
}

pub fn os_release() -> (String, String) {
    let text = std::fs::read_to_string("/etc/os-release").unwrap_or_default();
    let mut id = String::new();
    let mut name = String::new();
    for line in text.lines() {
        if let Some(v) = line.strip_prefix("ID=") {
            id = v.trim_matches('"').to_string();
        }
        if let Some(v) = line.strip_prefix("PRETTY_NAME=") {
            name = v.trim_matches('"').to_string();
        }
    }
    if name.is_empty() {
        name = "Unknown Linux".to_string();
    }
    (id, name)
}

pub fn kernel_version() -> String {
    std::fs::read_to_string("/proc/sys/kernel/osrelease")
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

pub fn plasma_version() -> String {
    version_of("plasmashell", &["--version"]).unwrap_or_default()
}

pub fn plugin_dir() -> PathBuf {
    let user = paths::plugin_dir();
    if user.join("metadata.json").exists() {
        user
    } else {
        paths::system_plugin_dir()
    }
}

pub fn plugin_version() -> Option<String> {
    let meta = plugin_dir().join("metadata.json");
    let text = std::fs::read_to_string(meta).ok()?;
    let json: serde_json::Value = serde_json::from_str(&text).ok()?;
    json.pointer("/KPlugin/Version")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

pub fn tools() -> Vec<ToolInfo> {
    let list: [(&str, &[&str], &str); 8] = [
        ("yt-dlp", &["--version"], "下载 B 站等站点的视频"),
        ("ffmpeg", &["-version"], "生成封面 / 转码"),
        ("ffprobe", &["-version"], "读取分辨率、时长"),
        ("qdbus6", &["--version"], "调用 PlasmaShell 切换壁纸"),
        ("kpackagetool6", &["--version"], "安装 Plasma 插件（可选）"),
        ("curl", &["--version"], "下载插件包"),
        ("bsdtar", &["--version"], "解压插件包"),
        ("pkexec", &["--version"], "以管理员权限安装依赖"),
    ];
    list.iter()
        .map(|(name, args, purpose)| {
            let ok = cmd_exists(name);
            ToolInfo {
                name: name.to_string(),
                ok,
                version: if ok {
                    version_of(name, args).unwrap_or_default()
                } else {
                    String::new()
                },
                purpose: purpose.to_string(),
            }
        })
        .collect()
}

pub fn env_status() -> EnvStatus {
    let (distro_id, distro_name) = os_release();
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "unknown".into());
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "unknown".into());
    let pill = tools();
    let plugin_version = plugin_version();
    let library_count = crate::library::load().len();
    EnvStatus {
        distro_id: distro_id.clone(),
        distro_name,
        kernel: kernel_version(),
        session_type: session_type.clone(),
        desktop: desktop.clone(),
        plasma_version: plasma_version(),
        plugin_ok: plugin_version.is_some(),
        plugin_version: plugin_version.unwrap_or_else(|| "未安装".into()),
        session_ok: desktop.to_lowercase().contains("kde") && session_type == "wayland",
        tested_distro: distro_id == "cachyos",
        tools: pill,
        library_count,
        wallpaper_dir: paths::wallpapers_dir().display().to_string(),
    }
}

/// 下载并安装 Smart Video Wallpaper Reborn 到用户级插件目录。
pub fn install_plugin() -> Result<String, String> {
    if plugin_version().is_some() {
        return Ok(format!(
            "Smart Video Wallpaper Reborn 已安装（{}）",
            plugin_version().unwrap_or_default()
        ));
    }
    if !cmd_exists("curl") || !cmd_exists("bsdtar") {
        return Err("缺少 curl 或 bsdtar，请先安装依赖".into());
    }
    let dest = paths::plugin_dir();
    let tmp = std::env::temp_dir().join(format!("aniwall-plugin-{}.tar.gz", std::process::id()));
    if let Some(parent) = tmp.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    run(
        "curl",
        &["-L", "--fail", "--retry", "2", "-o", &tmp.display().to_string(), PLUGIN_URL],
    )
    .map_err(|e| format!("下载插件失败：{e}"))?;

    let unpack = std::env::temp_dir().join(format!("aniwall-plugin-src-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&unpack);
    std::fs::create_dir_all(&unpack).map_err(|e| e.to_string())?;
    run(
        "bsdtar",
        &[
            "-xzf",
            &tmp.display().to_string(),
            "-C",
            &unpack.display().to_string(),
            "--strip-components=1",
        ],
    )
    .map_err(|e| format!("解压插件失败：{e}"))?;

    if !unpack.join("metadata.json").exists() {
        return Err(format!(
            "插件包结构异常：{} 下没有 metadata.json",
            unpack.display()
        ));
    }
    if dest.exists() {
        let _ = std::fs::remove_dir_all(&dest);
    }
    std::fs::create_dir_all(&dest).map_err(|e| e.to_string())?;
    copy_dir_all(&unpack, &dest).map_err(|e| format!("复制插件失败：{e}"))?;
    let _ = std::fs::remove_dir_all(&unpack);
    let _ = std::fs::remove_file(&tmp);

    // 让 Plasma 重新扫描插件（失败也不影响）
    let _ = run("kbuildsycoca6", &[]);
    Ok(format!(
        "已安装 Smart Video Wallpaper Reborn v{} 到 {}（若系统设置里看不到，可重启 plasmashell）",
        PLUGIN_VERSION,
        dest.display()
    ))
}

/// 用 pkexec + pacman 安装缺失的运行时依赖。
pub fn install_deps() -> Result<String, String> {
    let mut missing: Vec<&str> = Vec::new();
    if !cmd_exists("yt-dlp") {
        missing.push("yt-dlp");
    }
    if !cmd_exists("ffmpeg") || !cmd_exists("ffprobe") {
        missing.push("ffmpeg");
    }
    if !cmd_exists("curl") {
        missing.push("curl");
    }
    if !cmd_exists("bsdtar") {
        missing.push("libarchive");
    }
    if missing.is_empty() {
        return Ok("运行时依赖已齐全（yt-dlp / ffmpeg / curl / libarchive）".into());
    }
    if !cmd_exists("pacman") {
        return Err("当前系统没有 pacman，请手动安装：".to_string() + &missing.join(" "));
    }
    let mut args: Vec<String> = vec!["pacman".into(), "-S".into(), "--needed".into(), "--noconfirm".into()];
    args.extend(missing.iter().map(|s| s.to_string()));
    let out = Command::new("pkexec")
        .args(&args)
        .output()
        .map_err(|e| format!("无法调用 pkexec：{e}"))?;
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    if out.status.success() {
        Ok(format!("已安装：{}\n{}", missing.join(" "), text.trim()))
    } else {
        Err(format!("安装失败（可能需要输入密码）：{}", text.trim()))
    }
}

/// 生成并安装桌面快捷方式（.desktop + 图标）。
pub fn install_desktop_entry(exe: &Path) -> Result<String, String> {
    let apps = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("applications");
    std::fs::create_dir_all(&apps).map_err(|e| e.to_string())?;
    let entry = apps.join("aniwall.desktop");
    let content = format!(
        "[Desktop Entry]\nType=Application\nName=AniWall\nName[zh_CN]=AniWall 动态壁纸\nComment=KDE 动态壁纸管理器（下载 / 管理 / 一键切换）\nComment[zh_CN]=KDE 动态壁纸管理器（下载 / 管理 / 一键切换）\nExec={}\nIcon=aniwall\nTerminal=false\nCategories=Utility;Settings;\nStartupWMClass=aniwall\n",
        exe.display()
    );
    std::fs::write(&entry, content).map_err(|e| e.to_string())?;

    let icon_dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("icons/hicolor/256x256/apps");
    let _ = std::fs::create_dir_all(&icon_dir);
    let _ = std::fs::write(icon_dir.join("aniwall.png"), ICON_PNG);

    let _ = run("update-desktop-database", &[apps.to_string_lossy().as_ref()]);
    Ok(format!("桌面快捷方式已写入 {}", entry.display()))
}

/// 取消动态壁纸，恢复成 CachyOS 默认静态图（找不到则退回 KDE 默认色）。
pub fn restore_static_wallpaper() -> Result<String, String> {
    let candidates = [
        "/usr/share/wallpapers/cachyos-wallpapers/Cachy depths 5K.png",
        "/usr/share/wallpapers/cachyos-wallpapers/north.png",
        "/usr/share/wallpapers/Next/contents/images/2560x1440.png",
    ];
    let image = candidates
        .iter()
        .find(|p| Path::new(p).exists())
        .map(|s| s.to_string())
        .unwrap_or_default();
    let script = if image.is_empty() {
        "var ds = desktops(); for (var i = 0; i < ds.length; i++) { ds[i].wallpaperPlugin = 'org.kde.image'; } print('ok');"
            .to_string()
    } else {
        format!(
            "var ds = desktops(); for (var i = 0; i < ds.length; i++) {{ var d = ds[i]; d.wallpaperPlugin = 'org.kde.image'; d.currentConfigGroup = ['Wallpaper','org.kde.image','General']; d.writeConfig('Image', 'file://{}'); }} print('ok');",
            image
        )
    };
    crate::plasma::evaluate(&script).map(|_| "已恢复静态壁纸".to_string())
}

pub fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let target = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &target)?;
        } else {
            std::fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}

/// 128x128 的应用图标（由 `cargo tauri icon` 生成的同名文件内嵌）
const ICON_PNG: &[u8] = include_bytes!("../icons/128x128@2x.png");
