use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HomepageHero {
    pub title: String,
    pub subtitle: String,
    pub cta_text: String,
    pub cta_link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomepageHeroBody {
    pub title: String,
    pub subtitle: String,
    pub cta_text: String,
    pub cta_link: String,
}

/// pure-api1 compatibility: generic homepage section row
#[derive(Debug, Serialize, Deserialize)]
pub struct HomepageSectionRow {
    pub section_name: String,
    pub content: String,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

/// pure-api1 compatibility: PUT /api/homepage/:section body
#[derive(Debug, Deserialize)]
pub struct UpsertSectionBody {
    pub content: String,
}
