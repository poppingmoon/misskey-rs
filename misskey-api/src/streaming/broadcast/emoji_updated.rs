use crate::model::emoji::EmojiSimple;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmojiUpdatedEvent {
    pub emojis: Vec<EmojiSimple>,
}

impl misskey_core::streaming::BroadcastEvent for EmojiUpdatedEvent {
    const TYPE: &'static str = "emojiUpdated";
}

#[cfg(test)]
mod tests {
    use super::EmojiUpdatedEvent;
    use crate::test::{http::TestClient as HttpTestClient, websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};
    use ulid_crate::Ulid;

    #[tokio::test]
    async fn broadcast() {
        let http_client = HttpTestClient::new();
        let client = TestClient::new().await;
        let emoji_id = http_client.admin.get_emoji_id().await;
        let ulid = Ulid::new().to_string();
        http_client
            .admin
            .test(
                crate::endpoint::admin::emoji::update::Request::builder()
                    .id(emoji_id)
                    .name(ulid.clone())
                    .build(),
            )
            .await;

        let mut stream = client.broadcast::<EmojiUpdatedEvent>().await.unwrap();

        future::join(
            http_client.admin.test(
                crate::endpoint::admin::emoji::update::Request::builder()
                    .id(emoji_id)
                    .name(ulid.clone())
                    .build(),
            ),
            async { stream.next().await.unwrap().unwrap() },
        )
        .await;
    }
}
