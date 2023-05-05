use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::model::{ad::Ad, id::Id};

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    pub id: Id<Ad>,
    #[builder(default, setter(into))]
    pub memo: String,
    /// [ 1 .. ] characters
    #[builder(setter(into))]
    pub url: String,
    /// [ 1 .. ] characters
    #[builder(setter(into))]
    pub image_url: String,
    #[builder(default, setter(into))]
    pub place: String,
    #[builder(default, setter(into))]
    pub priority: String,
    #[cfg(feature = "12-81-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-81-0")))]
    #[builder(default, setter(into))]
    pub ratio: u64,
    #[cfg(feature = "13-7-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-7-0")))]
    #[serde(with = "ts_milliseconds")]
    #[builder(default, setter(into))]
    pub starts_at: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    #[builder(default, setter(into))]
    pub expires_at: DateTime<Utc>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/ad/update";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let url = client.avatar_url().await;

        client
            .admin
            .test(
                crate::endpoint::admin::ad::create::Request::builder()
                    .url(url.clone())
                    .image_url(url.clone())
                    .expires_at(chrono::Utc::now() + chrono::Duration::hours(1))
                    .build(),
            )
            .await;

        let ads = client
            .admin
            .test(crate::endpoint::admin::ad::list::Request::default())
            .await;

        client
            .admin
            .test(Request {
                id: ads[0].id,
                url: url.to_string(),
                memo: "memo".to_string(),
                place: "horizontal".to_string(),
                priority: "high".to_string(),
                #[cfg(feature = "12-81-0")]
                ratio: 2,
                image_url: url.to_string(),
                #[cfg(feature = "13-7-0")]
                starts_at: chrono::Utc::now(),
                expires_at: chrono::Utc::now() + chrono::Duration::hours(2),
            })
            .await;
    }
}
