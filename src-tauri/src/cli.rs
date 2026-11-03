use crate::{library, model::Settings, plasma, system};

const HELP: &str = "\
AniWall — KDE 动态壁纸管理器

用法:
  aniwall                       启动图形界面
  aniwall --check               打印环境诊断 JSON（用于 CI / 排障）
  aniwall --list                列出壁纸库
  aniwall --set <id|文件路径>   直接应用某个壁纸（id 来自 --list）
  aniwall --restore             恢复静态壁纸
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
            match apply(target) {
                Ok(()) => Some(0),
                Err(e) => {
                    eprintln!("应用失败: {e}");
                    Some(1)
                }
            }
        }
        "--restore" | "restore" => match system::restore_static_wallpaper() {
            Ok(message) => {
                println!("{message}");
                Some(0)
            }
            Err(e) => {
                eprintln!("恢复失败: {e}");
                Some(1)
            }
        },
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
