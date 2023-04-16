use crate::model::{emoji::Emoji, id::Id};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub ids: Vec<Id<Emoji>>,
    pub category: Option<String>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/emoji/set-category-bulk";
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
                category: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_category() {
        let client = TestClient::new();
        let id = client.admin.get_emoji_id().await;

        client
            .admin
            .test(Request {
                ids: vec![id],
                category: Some("cat".to_string()),
            })
            .await;
    }
}
