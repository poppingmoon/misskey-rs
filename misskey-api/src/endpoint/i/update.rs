use crate::model::{
    drive::DriveFile,
    id::Id,
    page::Page,
    user::{User, UserField},
};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub lang: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub location: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub birthday: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub avatar_id: Option<Option<Id<DriveFile>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub banner_id: Option<Option<Id<DriveFile>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub fields: Option<Vec<UserField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub is_locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub careful_bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub auto_accept_followed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub is_bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub is_cat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub auto_watch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub inject_featured_note: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub always_mark_nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub pinned_page_id: Option<Option<Id<Page>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub muted_words: Option<Vec<Vec<String>>>,
}

impl misskey_core::Request for Request {
    type Response = User;
    const ENDPOINT: &'static str = "i/update";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_options() {
        use crate::model::user::UserField;

        let client = TestClient::new();
        client
            .test(Request {
                name: Some(Some("test".to_string())),
                description: Some(Some("test description".to_string())),
                lang: Some(Some("ja-JP".to_string())),
                location: Some(Some("somewhere".to_string())),
                birthday: None,
                avatar_id: None,
                banner_id: None,
                fields: Some(vec![UserField {
                    name: "key".to_string(),
                    value: "value".to_string(),
                }]),
                is_locked: Some(true),
                careful_bot: Some(true),
                auto_accept_followed: Some(true),
                is_bot: Some(true),
                is_cat: Some(true),
                auto_watch: Some(true),
                inject_featured_note: Some(true),
                always_mark_nsfw: Some(true),
                pinned_page_id: None,
                muted_words: Some(vec![
                    vec!["mute1".to_string(), "mute2".to_string()],
                    vec!["mute3".to_string()],
                ]),
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_null_options() {
        let client = TestClient::new();
        client
            .test(Request {
                name: Some(None),
                description: Some(None),
                lang: Some(None),
                location: Some(None),
                birthday: Some(None),
                avatar_id: Some(None),
                banner_id: Some(None),
                fields: None,
                is_locked: None,
                careful_bot: None,
                auto_accept_followed: None,
                is_bot: None,
                is_cat: None,
                auto_watch: None,
                inject_featured_note: None,
                always_mark_nsfw: None,
                pinned_page_id: Some(None),
                muted_words: None,
            })
            .await;
    }
}
