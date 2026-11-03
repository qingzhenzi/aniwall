use crate::system;

const HELP: &str = "\
AniWall — KDE 动态壁纸管理器

用法:
  aniwall            启动图形界面
  aniwall --check    打印环境诊断 JSON（用于 CI / 排障）
  aniwall --version  显示版本
  aniwall --help     显示帮助
";

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
