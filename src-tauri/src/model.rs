use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallpaper {
    pub id: String,
    pub title: String,
    pub path: String,
    #[serde(default)]
    pub poster: Option<String>,
    #[serde(default)]
    pub size_bytes: u64,
    #[serde(default)]
    pub duration_secs: f64,
    #[serde(default)]
    pub width: u32,
    #[serde(default)]
    pub height: u32,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub added_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    /// 视频填充模式：0=适应 1=拉伸 2=裁剪填充
    #[serde(default = "d_fill")]
    pub fill_mode: i32,
    /// 静音模式：3=从不静音 5=总是静音
    #[serde(default = "d_mute")]
    pub mute_mode: i32,
    /// 暂停模式：0=有最大化/全屏窗口时暂停, 1=当前窗口时暂停, 2=有窗口时暂停, 3=从不
    #[serde(default = "d_pause")]
    pub pause_mode: i32,
    #[serde(default = "d_true")]
    pub battery_pauses: bool,
    #[serde(default = "d_battery")]
    pub battery_level: i32,
    #[serde(default = "d_true")]
    pub resume_last: bool,
    /// 可选：yt-dlp cookies 文件（B 站部分视频需要登录）
    #[serde(default)]
    pub cookie_file: String,
    /// 当前生效的壁纸 id
    #[serde(default)]
    pub active_id: String,
    #[serde(default = "d_true")]
    pub loop_playback: bool,
}

fn d_fill() -> i32 {
    2
}
fn d_mute() -> i32 {
    5
}
fn d_pause() -> i32 {
    0
}
fn d_battery() -> i32 {
    20
}
fn d_true() -> bool {
    true
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            fill_mode: d_fill(),
            mute_mode: d_mute(),
            pause_mode: d_pause(),
            battery_pauses: true,
            battery_level: d_battery(),
            resume_last: true,
            cookie_file: String::new(),
            active_id: String::new(),
            loop_playback: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogItem {
    pub id: String,
    pub bvid: String,
    pub title: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub poster: String,
    #[serde(default)]
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ToolInfo {
    pub name: String,
    pub ok: bool,
    pub version: String,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct EnvStatus {
    pub distro_id: String,
    pub distro_name: String,
    pub kernel: String,
    pub session_type: String,
    pub desktop: String,
    pub plasma_version: String,
    pub plugin_version: String,
    pub plugin_ok: bool,
    pub session_ok: bool,
    pub tested_distro: bool,
    pub tools: Vec<ToolInfo>,
    pub library_count: usize,
    pub wallpaper_dir: String,
}
