use crate::model::Settings;
use crate::system;
use std::path::Path;

pub const PLUGIN_ID: &str = system::PLUGIN_ID;

/// 通过 PlasmaShell 的脚本接口执行一段 QML 引擎脚本（同步等待结果）。
pub fn evaluate(script: &str) -> Result<String, String> {
    if !system::cmd_exists("qdbus6") {
        return Err("找不到 qdbus6（KDE Plasma 环境才能切换壁纸）".into());
    }
    system::run(
        "qdbus6",
        &[
            "org.kde.plasmashell",
            "/PlasmaShell",
            "org.kde.PlasmaShell.evaluateScript",
            script,
        ],
    )
}

/// 把本地视频路径编码成 file:// URI（非 ASCII 字符做 percent-encode）。
pub fn file_uri(path: &Path) -> String {
    let raw = path.to_string_lossy();
    let mut out = String::from("file://");
    for b in raw.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' | b'/' => {
                out.push(*b as char)
            }
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

fn escape_js_single(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\'', "\\'")
}

/// 应用某个视频为当前桌面壁纸（所有桌面/屏幕）。
pub fn set_video(path: &Path, s: &Settings) -> Result<String, String> {
    let uri = file_uri(path);
    let videos = serde_json::json!([{
        "filename": uri,
        "enabled": true,
        "duration": 0,
        "customDuration": 0,
        "playbackRate": 0.0,
        "alternativePlaybackRate": 0.0,
        "loop": s.loop_playback,
    }])
    .to_string();
    let videos_js = escape_js_single(&videos);

    let script = format!(
        r#"
var videos = '{videos_js}';
var ds = desktops();
for (var i = 0; i < ds.length; i++) {{
  var d = ds[i];
  d.wallpaperPlugin = '{plugin}';
  d.currentConfigGroup = ['Wallpaper', '{plugin}', 'General'];
  d.writeConfig('VideoUrls', videos);
  d.writeConfig('FillMode', {fill});
  d.writeConfig('MuteMode', {mute});
  d.writeConfig('PauseMode', {pause});
  d.writeConfig('BatteryPausesVideo', {battery});
  d.writeConfig('PauseBatteryLevel', {level});
  d.writeConfig('ResumeLastVideo', {resume});
}}
print('ok:' + ds.length);
"#,
        videos_js = videos_js,
        plugin = PLUGIN_ID,
        fill = s.fill_mode,
        mute = s.mute_mode,
        pause = s.pause_mode,
        battery = if s.battery_pauses { "true" } else { "false" },
        level = s.battery_level,
        resume = if s.resume_last { "true" } else { "false" },
    );

    let out = evaluate(&script)?;
    if !out.contains("ok:") {
        return Err(format!("Plasma 返回异常：{out}"));
    }
    Ok(out)
}

/// 读取当前每个桌面的壁纸插件与视频配置。
pub fn current() -> serde_json::Value {
    let script = r#"
var ds = desktops();
var out = [];
for (var i = 0; i < ds.length; i++) {
  var d = ds[i];
  var videos = '';
  try {
    d.currentConfigGroup = ['Wallpaper', d.wallpaperPlugin, 'General'];
    videos = String(d.readConfig('VideoUrls', ''));
  } catch (e) { videos = ''; }
  out.push({ plugin: String(d.wallpaperPlugin), videos: videos });
}
print(JSON.stringify(out));
"#;
    match evaluate(script) {
        Ok(text) => {
            // qdbus 的输出可能带额外日志行，取第一段合法 JSON
            for line in text.lines() {
                let line = line.trim();
                if line.starts_with('[') {
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
                        return v;
                    }
                }
            }
            serde_json::json!([])
        }
        Err(_) => serde_json::json!([]),
    }
}
