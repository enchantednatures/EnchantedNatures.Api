use crate::database::PhotoRepository;


pub struct AppState {
    pub repo: PhotoRepository,
    pub client: aws_sdk_s3::Client,
}

impl AppState {
    fn new(repo: PhotoRepository, client: aws_sdk_s3::Client) -> Self {
        Self { repo, client }
    }
}
