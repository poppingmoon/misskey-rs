#[cfg(feature = "13-13-0")]
use crate::model::{drive::DriveFile, role::Role};
use crate::model::{emoji::Emoji, id::Id};

use serde::Serialize;
use typed_builder::TypedBuilder;
#[cfg(any(docsrs, not(feature = "12-9-0")))]
use url::Url;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    pub id: Id<Emoji>,
    #[builder(setter(into))]
    pub name: String,
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub file_id: Option<Id<DriveFile>>,
    #[builder(default, setter(strip_option, into))]
    pub category: Option<String>,
    #[builder(default)]
    pub aliases: Vec<String>,
    #[cfg(any(docsrs, not(feature = "12-9-0")))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-9-0"))))]
    pub url: Url,
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub license: Option<String>,
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub is_sensitive: Option<bool>,
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub local_only: Option<bool>,
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub role_ids_that_can_be_used_this_emoji_as_reaction: Option<Vec<Id<Role>>>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/emoji/update";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        #[cfg(not(feature = "12-9-0"))]
        let image_url = client.avatar_url().await;
        let id = client.admin.get_emoji_id().await;
        let name = ulid_crate::Ulid::new().to_string();

        client
            .admin
            .test(Request {
                id,
                name,
                #[cfg(feature = "13-13-0")]
                file_id: None,
                category: Some("great".to_string()),
                aliases: vec!["namename".to_string()],
                #[cfg(not(feature = "12-9-0"))]
                url: image_url,
                #[cfg(feature = "13-10-0")]
                license: None,
                #[cfg(feature = "13-13-0")]
                is_sensitive: None,
                #[cfg(feature = "13-13-0")]
                local_only: None,
                #[cfg(feature = "13-13-0")]
                role_ids_that_can_be_used_this_emoji_as_reaction: None,
            })
            .await;
    }

    #[cfg(feature = "13-10-0")]
    #[tokio::test]
    async fn request_with_options() {
        let client = TestClient::new();
        let id = client.admin.get_emoji_id().await;
        let name = ulid_crate::Ulid::new().to_string();
        let file = client.admin.get_drive_file().await;
        let role = client
            .admin
            .test(crate::endpoint::admin::roles::create::Request::default())
            .await;

        client
            .admin
            .test(Request {
                id,
                name,
                #[cfg(feature = "13-13-0")]
                file_id: Some(file.id),
                category: Some("great".to_string()),
                aliases: vec!["namename".to_string()],
                license: Some("license".to_string()),
                #[cfg(feature = "13-13-0")]
                is_sensitive: Some(true),
                #[cfg(feature = "13-13-0")]
                local_only: Some(true),
                #[cfg(feature = "13-13-0")]
                role_ids_that_can_be_used_this_emoji_as_reaction: Some(vec![role.id]),
            })
            .await;
    }
}
