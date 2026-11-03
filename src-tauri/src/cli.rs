use crate::{library, model::Settings, plasma, system};

const HELP: &str = "\
AniWall — KDE 动态壁纸管理器

用法:
  aniwall                       启动图形界面（最小化到托盘）
  aniwall --minimized           启动时不显示窗口（开机自启用）
  aniwall --check               打印环境诊断 JSON（用于 CI / 排障）
  aniwall --list                列出壁纸库
  aniwall --set <id|文件路径>   直接应用某个壁纸（id 来自 --list）
  aniwall --random              随机换一张壁纸
  aniwall --next                切换到下一张壁纸
  aniwall --restore             恢复静态壁纸
  aniwall --settings            打印当前播放设置 JSON
  aniwall --rename <id> <标题>  修改壁纸标题
  aniwall --selftest            后端自检（设置读写、壁纸库、桌面集成）
  aniwall --autostart on|off|status   开机自启开关
  aniwall --version             显示版本
  aniwall --help                显示帮助
";

fn print_library() {
    let items = library::load();
    if items.is_empty() {
        println!("壁纸库为空。");
        return;
    }
    for item in items {
        println!(
            "{}\t{}\t{}x{}\t{:.0}s\t{}",
            item.id, item.title, item.width, item.height, item.duration_secs, item.path
        );
    }
}

fn apply(target: &str) -> Result<(), String> {
    let settings: Settings = library::read_settings();
    let items = library::load();
    let found = items
        .iter()
        .find(|wallpaper| wallpaper.id == target || wallpaper.path == target);
    let (path, title) = match found {
        Some(item) => (std::path::PathBuf::from(&item.path), item.title.clone()),
        None => {
            let path = std::path::PathBuf::from(target);
            if !path.exists() {
                return Err(format!("找不到壁纸 id 或文件：{target}"));
            }
            (path, target.to_string())
        }
    };
    plasma::set_video(&path, &settings)?;
    println!("已应用：{title}");
    Ok(())
}

fn selftest() -> i32 {
    let mut failures = 0;
    let mut check = |name: &str, ok: bool, detail: &str| {
        println!("{} {name} — {detail}", if ok { "✓" } else { "✗" });
        if !ok {
            failures += 1;
        }
    };

    // 1. 数据目录
    let dirs_ok = crate::paths::ensure_dirs().is_ok();
    check(
        "数据目录",
        dirs_ok,
        &crate::paths::data_dir().display().to_string(),
    );

    // 2. 设置读写
    let settings = library::read_settings();
    let wrote = library::write_settings(&settings).is_ok();
    let again = library::read_settings();
    let same = serde_json::to_string(&settings).unwrap_or_default()
        == serde_json::to_string(&again).unwrap_or_default();
    check("播放设置读写", wrote && same, "round-trip 一致");

    // 3. 壁纸库
    let items = library::load();
    let missing: Vec<&str> = items
        .iter()
        .filter(|w| !std::path::Path::new(&w.path).exists())
        .map(|w| w.path.as_str())
        .collect();
    check(
        "壁纸库索引",
        missing.is_empty(),
        &format!("{} 个壁纸，缺失文件 {} 个", items.len(), missing.len()),
    );

    // 4. 播放器插件
    let plugin = system::plugin_version();
    check(
        "Smart Video Wallpaper Reborn",
        plugin.is_some(),
        &plugin.unwrap_or_else(|| "未安装".into()),
    );

    // 5. Plasma 会话可访问
    let current = plasma::current();
    let plasma_ok = current
        .as_array()
        .map(|a| !a.is_empty())
        .unwrap_or(false);
    check(
        "PlasmaShell 连通",
        plasma_ok,
        &format!("当前桌面数 {}", current.as_array().map(|a| a.len()).unwrap_or(0)),
    );

    // 6. 桌面集成
    let desktop = dirs::config_dir()
        .map(|p| p.join("autostart/aniwall.desktop"))
        .unwrap_or_default();
    check("开机自启项", true, if desktop.exists() { "已开启" } else { "未开启" });

    // 7. 图标
    let icon = dirs::data_dir()
        .map(|p| p.join("icons/hicolor/256x256/apps/aniwall.png"))
        .unwrap_or_default();
    check(
        "应用图标",
        icon.exists(),
        &icon.display().to_string(),
    );

    if failures == 0 {
        println!("
全部通过 ✅");
        0
    } else {
        println!("
{failures} 项失败 ❌");
        1
    }
}

fn report(result: Result<String, String>) -> i32 {
    match result {
        Ok(message) => {
            println!("{message}");
            0
        }
        Err(error) => {
            eprintln!("{error}");
            1
        }
    }
}

fn autostart_command(args: &[String]) -> i32 {
    match args.get(2).map(|value| value.as_str()) {
        Some("on") | Some("enable") => report(crate::set_autostart_impl(true)),
        Some("off") | Some("disable") => report(crate::set_autostart_impl(false)),
        Some("status") | None => {
            println!(
                "开机自启：{}",
                if crate::autostart_enabled() {
                    "已开启"
                } else {
                    "已关闭"
                }
            );
            0
        }
        Some(other) => {
            eprintln!("未知参数：{other}（可用 on / off / status）");
            2
        }
    }
}

/// 如果是 CLI 调用则执行并返回退出码；否则返回 None 交给 GUI。
pub fn maybe_run() -> Option<i32> {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1)?.as_str();
    match cmd {
        "--check" | "check" => {
            let status = system::env_status();
            match serde_json::to_string_pretty(&status) {
                Ok(json) => {
                    println!("{json}");
                    Some(0)
                }
                Err(e) => {
                    eprintln!("序列化失败: {e}");
                    Some(1)
                }
            }
        }
        "--list" | "list" => {
            print_library();
            Some(0)
        }
        "--set" | "set" => {
            let Some(target) = args.get(2) else {
                eprintln!("用法: aniwall --set <id|文件路径>");
                return Some(2);
            };
            Some(match apply(target) {
                Ok(()) => 0,
                Err(e) => {
                    eprintln!("应用失败: {e}");
                    1
                }
            })
        }
        "--random" | "random" => Some(report(crate::random_wallpaper_impl())),
        "--next" | "next" => Some(report(crate::next_wallpaper_impl())),
        "--restore" | "restore" => Some(report(crate::restore_impl())),
        "--autostart" | "autostart" => Some(autostart_command(&args)),
        "--settings" | "settings" => {
            match serde_json::to_string_pretty(&library::read_settings()) {
                Ok(json) => {
                    println!("{json}");
                    Some(0)
                }
                Err(error) => {
                    eprintln!("{error}");
                    Some(1)
                }
            }
        }
        "--rename" | "rename" => {
            let (Some(id), Some(title)) = (args.get(2), args.get(3)) else {
                eprintln!("用法: aniwall --rename <id> <新标题>");
                return Some(2);
            };
            let mut items = library::load();
            match items.iter_mut().find(|w| &w.id == id) {
                Some(item) => {
                    item.title = title.clone();
                    match library::save(&items) {
                        Ok(()) => {
                            println!("已重命名：{title}");
                            Some(0)
                        }
                        Err(error) => {
                            eprintln!("保存失败：{error}");
                            Some(1)
                        }
                    }
                }
                None => {
                    eprintln!("找不到壁纸：{id}");
                    Some(1)
                }
            }
        }
        "--selftest" | "selftest" => Some(selftest()),
        "--version" | "-V" => {
            println!("aniwall {}", env!("CARGO_PKG_VERSION"));
            Some(0)
        }
        "--help" | "-h" => {
            print!("{HELP}");
            Some(0)
        }
        _ => None,
    }
}
