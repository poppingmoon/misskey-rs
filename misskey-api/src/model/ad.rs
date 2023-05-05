use crate::model::id::Id;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ad {
    pub id: Id<Ad>,
    #[serde(default)]
    pub created_at: Option<DateTime<Utc>>,
    #[cfg(feature = "13-7-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-7-0")))]
    #[serde(default)]
    pub starts_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub expires_at: Option<DateTime<Utc>>,
    pub place: String,
    #[serde(default)]
    pub priority: Option<String>,
    #[cfg(feature = "12-81-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-81-0")))]
    pub ratio: u64,
    pub url: String,
    pub image_url: String,
    #[serde(default)]
    pub memo: Option<String>,
}

impl_entity!(Ad);
