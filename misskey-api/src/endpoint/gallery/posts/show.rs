use crate::model::{gallery::GalleryPost, id::Id};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub post_id: Id<GalleryPost>,
}

impl misskey_core::Request for Request {
    type Response = GalleryPost;
    const ENDPOINT: &'static str = "gallery/posts/show";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let file = client.get_drive_file().await;

        let post = client
            .test(crate::endpoint::gallery::posts::create::Request {
                title: "gallery post".to_string(),
                description: None,
                file_ids: vec![file.id],
                is_sensitive: None,
            })
            .await;

        client.test(Request { post_id: post.id }).await;
    }
}
