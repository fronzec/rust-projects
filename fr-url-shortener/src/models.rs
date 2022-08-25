use chrono::NaiveDateTime;

// Using #[derive(Queryable)] assumes that the order of fields
#[derive(Queryable)]
pub struct Url {
    pub id: i32,
    pub key: String,
    pub secret_key: String,
    pub target_url: String,
    pub is_active: bool,
    pub clicks: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}