use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
pub struct ShortenUrlDto {
    pub target_url: String,
}

#[derive(Serialize)]
pub struct ShortenedUrlDto {
    pub id: u64,
    pub target_url: String,
    pub url: String,
    pub is_active: bool,
    pub clicks_count: u32,
    pub admin_url: String,
}