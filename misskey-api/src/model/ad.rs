use crate::model::id::Id;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ad {
    pub id: Id<Ad>,
    #[serde(default)]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub expires_at: Option<DateTime<Utc>>,
    pub place: String,
    #[serde(default)]
    pub priority: Option<String>,
    pub url: String,
    pub image_url: String,
    #[serde(default)]
    pub memo: Option<String>,
}

impl_entity!(Ad);
