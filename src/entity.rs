use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoticeSent {
    pub id: i32,                // DB autoincrement
    pub main_category: String,  // Training | ClassNotice | StudentAffairs | Tuition
    pub external_id: String,    // hash(title+date) or source id
    pub published_date: String, // YYYY-MM-DD or None
    pub title: String,          // must be >= 3 characters
    pub body: String,           // optional, non-empty if present
    pub sent_at: String,        // ISO timestamp
}

impl NoticeSent {
    pub fn new(date: String, title: String, content: String) -> Self {
        // external_id = hash(date + title)
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(date.as_bytes());
        hasher.update(title.as_bytes());
        let hash = format!("{:x}", hasher.finalize());

        // sent_at = current UTC timestamp
        let sent_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        NoticeSent {
            id: 0, // let DB autoincrement
            main_category: "ClassNotice".to_string(),
            external_id: hash,
            published_date: date,
            title,
            body: content,
            sent_at,
        }
    }
}

impl std::fmt::Display for NoticeSent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}\n{}", self.published_date, self.title, self.body)
    }
}

#[allow(dead_code)]
impl NoticeSent {
    pub fn fmt_id(&self) {
        println!(
            "[{}] {}\n{}",
            self.published_date, self.title, self.external_id
        )
    }
}
