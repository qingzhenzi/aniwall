use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn is_video_ext(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());
    matches!(
        ext.as_deref(),
        Some("mp4" | "webm" | "mkv" | "mov" | "flv" | "m4v")
    )
}

fn snapshot(dir: &Path) -> Vec<PathBuf> {
    std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.is_file() && is_video_ext(p))
                .collect()
        })
        .unwrap_or_default()
}

fn parse_percent(line: &str) -> Option<f64> {
    let idx = line.find('%')?;
    let before = &line[..idx];
    let start = before
        .rfind(|c: char| !(c.is_ascii_digit() || c == '.'))
        .map(|i| i + 1)
        .unwrap_or(0);
    before[start..].trim().parse::<f64>().ok()
}

/// 下载一个视频地址到 `dest_dir`。支持：
/// - 直链（.mp4/.webm/.mkv 等）：用 curl 下载
/// - 其它：交给 yt-dlp（Bilibili / YouTube 等）
///
/// `on_progress(percent, message)` 会被频繁调用。
pub fn download(
    url: &str,
    dest_dir: &Path,
    cookie: Option<&Path>,
    on_progress: &mut dyn FnMut(f64, String),
) -> Result<PathBuf, String> {
    std::fs::create_dir_all(dest_dir).map_err(|e| e.to_string())?;
    let url = url.trim();
    if url.is_empty() {
        return Err("下载地址为空".into());
    }

    let direct = {
        let lower = url.to_lowercase();
        lower.contains(".mp4") || lower.contains(".webm") || lower.contains(".mkv")
    };
    if direct {
        return download_direct(url, dest_dir, on_progress);
    }
    download_ytdlp(url, dest_dir, cookie, on_progress)
}

fn download_direct(
    url: &str,
    dest_dir: &Path,
    on_progress: &mut dyn FnMut(f64, String),
) -> Result<PathBuf, String> {
    let name = url
        .split('/')
        .next_back()
        .filter(|s| !s.is_empty())
        .unwrap_or("downloaded.mp4")
        .split('?')
        .next()
        .unwrap_or("downloaded.mp4")
        .to_string();
    let dest = dest_dir.join(name);
    on_progress(0.0, "开始下载直链…".into());
    let out = Command::new("curl")
        .args(["-L", "--fail", "--retry", "2", "-o"])
        .arg(&dest)
        .arg(url)
        .output()
        .map_err(|e| format!("无法执行 curl：{e}"))?;
    if !out.status.success() {
        return Err(format!(
            "curl 下载失败：{}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    on_progress(100.0, "下载完成".into());
    Ok(dest)
}

fn download_ytdlp(
    url: &str,
    dest_dir: &Path,
    cookie: Option<&Path>,
    on_progress: &mut dyn FnMut(f64, String),
) -> Result<PathBuf, String> {
    if !crate::system::cmd_exists("yt-dlp") {
        return Err("缺少 yt-dlp，请先在「初始化」页安装依赖".into());
    }
    let before = snapshot(dest_dir);

    let stderr_path = std::env::temp_dir().join(format!("aniwall-ytdlp-{}.log", std::process::id()));
    let stderr_file = std::fs::File::create(&stderr_path).map_err(|e| e.to_string())?;

    let output_tpl = format!("{}/%(title).60s-%(id)s.%(ext)s", dest_dir.display());
    let mut cmd = Command::new("yt-dlp");
    cmd.args([
        "--no-warnings",
        "--newline",
        "--no-playlist",
        "--no-simulate",
        "-f",
        "bv*[height<=1080][vcodec^=avc1]+ba/bv*[height<=1080]+ba/b[height<=1080]",
        "--merge-output-format",
        "mp4",
        "-o",
        &output_tpl,
    ]);
    if let Some(c) = cookie {
        if c.exists() {
            cmd.arg("--cookies").arg(c);
        }
    }
    cmd.arg("--").arg(url);
    cmd.stdout(Stdio::piped()).stderr(Stdio::from(stderr_file));

    let mut child = cmd.spawn().map_err(|e| format!("无法启动 yt-dlp：{e}"))?;
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines().map_while(Result::ok) {
            let trimmed = line.trim().to_string();
            if trimmed.contains("[download]") {
                if let Some(pct) = parse_percent(&trimmed) {
                    on_progress(pct, trimmed);
                }
            }
        }
    }
    let status = child.wait().map_err(|e| e.to_string())?;
    if !status.success() {
        let log = std::fs::read_to_string(&stderr_path).unwrap_or_default();
        let _ = std::fs::remove_file(&stderr_path);
        return Err(format!(
            "yt-dlp 下载失败：{}",
            log.lines().last().unwrap_or("未知错误")
        ));
    }
    let _ = std::fs::remove_file(&stderr_path);

    let after = snapshot(dest_dir);
    let mut new_files: Vec<PathBuf> = after.into_iter().filter(|p| !before.contains(p)).collect();
    new_files.sort_by_key(|p| std::fs::metadata(p).and_then(|m| m.modified()).ok());
    new_files.pop().ok_or_else(|| "下载完成但没找到视频文件".to_string())
}
