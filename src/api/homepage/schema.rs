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
