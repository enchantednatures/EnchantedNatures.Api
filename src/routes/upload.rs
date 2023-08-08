use std::io;
use tokio::io::AsyncReadExt;

use axum::extract::{BodyStream, Path, State};
use futures::TryStreamExt;
use tokio_util::io::StreamReader;

use crate::App;

#[utoipa::path(
    post,
    path = "/api/v0/upload/{file_name}",
    params(
        ("file_name"= String, Path, description = "Filename")
    ),
    request_body(content = [u8], description = "File contents", content_type = "image/jpeg")
)]
pub async fn save_request_body(
    State(app): State<App>,
    Path(file_name): Path<String>,
    body: BodyStream,
) {
    let body_with_io_error = body.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
    let mut body_reader = StreamReader::new(body_with_io_error);

    let mut buffer = Vec::new();
    body_reader
        .read_to_end(&mut buffer)
        .await
        .expect("Failed to read body");

    // let mut body = body.into_stream().into_inner();
    // while let Some(chunk) = body.next().await {
    //     let chunk = chunk.expect("Body chunk must be okay");
    //     bytes.extend_from_slice(&chunk);
    // }
    println!("{:?}", buffer);
    match app.upload_photo(buffer, &file_name).await {
        Ok(output) => println!("File uploaded successfully: {:?}", output),
        Err(err) => eprintln!("Failed to upload file: {:?}", err), // Handle error as needed
    }
}
