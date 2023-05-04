use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    /// [ 1 .. ] characters
    #[builder(setter(into))]
    pub url: String,
    #[builder(default, setter(into))]
    pub memo: String,
    #[builder(default, setter(into))]
    pub place: String,
    #[builder(default, setter(into))]
    pub priority: String,
    #[cfg(feature = "12-81-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-81-0")))]
    #[builder(default, setter(into))]
    pub ratio: u64,
    #[serde(with = "ts_milliseconds")]
    #[builder(default, setter(into))]
    pub expires_at: DateTime<Utc>,
    /// [ 1 .. ] characters
    #[builder(setter(into))]
    pub image_url: String,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/ad/create";
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
            .test(Request {
                url: url.to_string(),
                memo: "memo".to_string(),
                place: "square".to_string(),
                priority: "middle".to_string(),
                #[cfg(feature = "12-81-0")]
                ratio: 1,
                image_url: url.to_string(),
                expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
            })
            .await;
    }
}
