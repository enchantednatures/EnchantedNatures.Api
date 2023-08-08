use aws_sdk_s3::{
    error::SdkError,
    operation::put_object::{PutObjectError, PutObjectOutput},
    primitives::ByteStream,
};

use crate::database::PhotoRepository;

pub struct AppState {
    pub repo: PhotoRepository,
    pub client: aws_sdk_s3::Client,
    bucket_name: String,
}

impl AppState {
    pub(crate) fn new(repo: PhotoRepository, client: aws_sdk_s3::Client) -> Self {
        Self {
            repo,
            client,
            bucket_name: "photos".into(),
        }
    }

    pub async fn upload_photo(
        &self,
        body: Vec<u8>,
        key: &str,
    ) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .body(ByteStream::from(body))
            .send()
            .await
    }
}