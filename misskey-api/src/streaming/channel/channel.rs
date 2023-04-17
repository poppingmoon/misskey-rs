#[cfg(feature = "12-71-0")]
use crate::model::user::User;
use crate::model::{channel::Channel, id::Id, note::Note};
use crate::streaming::channel::NoOutgoing;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum ChannelEvent {
    Note(Note),
    #[cfg(feature = "12-71-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-71-0")))]
    Typers(Vec<User>),
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub channel_id: Id<Channel>,
}

impl misskey_core::streaming::ConnectChannelRequest for Request {
    type Incoming = ChannelEvent;
    type Outgoing = NoOutgoing;

    const NAME: &'static str = "channel";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        let client = TestClient::new().await;
        let channel = client
            .test(
                crate::endpoint::channels::create::Request::builder()
                    .name("test")
                    .build(),
            )
            .await;

        let mut stream = client
            .channel(Request {
                channel_id: channel.id,
            })
            .await
            .unwrap();
        stream.disconnect().await.unwrap();
    }

    #[tokio::test]
    async fn stream() {
        let client = TestClient::new().await;
        let channel = client
            .test(
                crate::endpoint::channels::create::Request::builder()
                    .name("test")
                    .build(),
            )
            .await;

        let mut stream = client
            .channel(Request {
                channel_id: channel.id.clone(),
            })
            .await
            .unwrap();

        future::join(
            client.test(
                crate::endpoint::notes::create::Request::builder()
                    .text("some text")
                    .channel_id(channel.id)
                    .build(),
            ),
            async { stream.next().await.unwrap().unwrap() },
        )
        .await;
    }
}
