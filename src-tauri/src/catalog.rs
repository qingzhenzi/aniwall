use crate::model::CatalogItem;

/// 内置壁纸清单（随二进制内嵌，来源见 README）
const CATALOG_JSON: &str = include_str!("../../catalog.json");

pub fn items() -> Vec<CatalogItem> {
    serde_json::from_str(CATALOG_JSON).unwrap_or_default()
}

/// 把 BVID / 裸 URL 规范化成 B 站视频地址
pub fn normalize_url(input: &str) -> String {
    let input = input.trim();
    if input.starts_with("http://") || input.starts_with("https://") {
        return input.to_string();
    }
    if input.starts_with("BV") {
        return format!("https://www.bilibili.com/video/{input}");
    }
    input.to_string()
}
