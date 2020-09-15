#[cfg(feature = "12-9-0")]
use crate::model::drive::DriveFileId;
use crate::model::emoji::EmojiId;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[cfg(any(docsrs, not(feature = "12-9-0")))]
use url::Url;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[cfg(feature = "12-9-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-9-0")))]
    pub file_id: DriveFileId,
    #[cfg(any(docsrs, not(feature = "12-9-0")))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-9-0"))))]
    #[builder(setter(into))]
    pub name: String,
    #[cfg(any(docsrs, not(feature = "12-9-0")))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-9-0"))))]
    pub url: Url,
    #[cfg(any(docsrs, not(feature = "12-9-0")))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-9-0"))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub category: Option<String>,
    #[cfg(any(docsrs, not(feature = "12-9-0")))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-9-0"))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub aliases: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: EmojiId,
}

impl misskey_core::Request for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "admin/emoji/add";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    #[cfg(feature = "12-9-0")]
    async fn request() {
        let client = TestClient::new();
        let image_url = client.avatar_url().await;
        let file = client
            .test(crate::endpoint::drive::files::upload_from_url::Request {
                url: image_url,
                folder_id: None,
                is_sensitive: None,
                force: None,
            })
            .await;

        client.admin.test(Request { file_id: file.id }).await;
    }

    #[tokio::test]
    #[cfg(not(feature = "12-9-0"))]
    async fn request() {
        let client = TestClient::new();
        let image_url = client.avatar_url().await;
        let name = uuid::Uuid::new_v4().to_simple().to_string();

        client
            .admin
            .test(Request {
                name,
                url: image_url,
                category: None,
                aliases: None,
            })
            .await;
    }

    #[tokio::test]
    #[cfg(not(feature = "12-9-0"))]
    async fn request_with_options() {
        let client = TestClient::new();
        let image_url = client.avatar_url().await;
        let name = uuid::Uuid::new_v4().to_simple().to_string();

        client
            .admin
            .test(Request {
                name,
                url: image_url,
                category: Some("nice".to_string()),
                aliases: Some(vec!["test2".to_string()]),
            })
            .await;
    }
}
