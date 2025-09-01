use chrono::{NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Categories allowed in DB
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Category {
    Training,
    ClassNotice,
    StudentAffairs,
    Tuition,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Category::Training => "Training",
            Category::ClassNotice => "ClassNotice",
            Category::StudentAffairs => "StudentAffairs",
            Category::Tuition => "Tuition",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Category {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Training" => Ok(Category::Training),
            "ClassNotice" => Ok(Category::ClassNotice),
            "StudentAffairs" => Ok(Category::StudentAffairs),
            "Tuition" => Ok(Category::Tuition),
            _ => Err(format!("Invalid category: {}", s)),
        }
    }
}

mod date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => serializer.serialize_str(&d.format(FORMAT).to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;
        match opt {
            Some(s) => NaiveDate::parse_from_str(&s, FORMAT)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

mod datetime_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(dt: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&dt.format(FORMAT).to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoticeSent {
    pub id: i32,                 // DB autoincrement
    pub main_category: Category, // Training | ClassNotice | StudentAffairs | Tuition
    pub external_id: String,     // hash(title+date) or source id

    #[serde(with = "date_format")]
    pub published_date: Option<NaiveDate>, // DATE

    pub title: String,        // must be >= 3 characters
    pub body: Option<String>, // optional, non-empty if present

    #[serde(with = "datetime_format")]
    pub sent_at: NaiveDateTime, // TIMESTAMP
}

impl NoticeSent {
    pub fn new(
        date: Option<String>,
        title: String,
        content: Option<String>,
        category: Category,
    ) -> Self {
        use sha2::{Digest, Sha256};

        // external_id = hash(date + title)
        let mut hasher = Sha256::new();
        if let Some(d) = &date {
            hasher.update(d.as_bytes());
        }
        hasher.update(title.as_bytes());
        let hash = format!("{:x}", hasher.finalize());

        // sent_at = current UTC timestamp
        let sent_at = Utc::now().naive_utc();

        // parse date if provided
        let published_date =
            date.and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());

        NoticeSent {
            id: 0,
            main_category: category,
            external_id: hash,
            published_date,
            title,
            body: content,
            sent_at,
        }
    }
}

impl fmt::Display for NoticeSent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}\n{}",
            self.published_date
                .map(|d| d.to_string())
                .unwrap_or_else(|| "N/A".to_string()),
            self.title,
            self.body.clone().unwrap_or_default()
        )
    }
}

impl NoticeSent {
    pub fn fmt_id(&self) {
        println!(
            "[{}] {}\n{}",
            self.published_date
                .map(|d| d.to_string())
                .unwrap_or_else(|| "N/A".to_string()),
            self.title,
            self.external_id
        )
    }
}
