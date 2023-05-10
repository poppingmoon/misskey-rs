use crate::model::{emoji::Emoji, id::Id};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub ids: Vec<Id<Emoji>>,
    pub license: Option<String>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/emoji/set-license-bulk";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let id = client.admin.get_emoji_id().await;

        client
            .admin
            .test(Request {
                ids: vec![id],
                license: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_license() {
        let client = TestClient::new();
        let id = client.admin.get_emoji_id().await;

        client
            .admin
            .test(Request {
                ids: vec![id],
                license: Some("license".to_string()),
            })
            .await;
    }
}
