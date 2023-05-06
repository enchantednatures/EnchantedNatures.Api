use crate::view_models::PhotoViewModel;

#[get("/photos")]
fn photos(pool: sqlx::PgPool) -> impl Responder {
    let query = include_str!("../sql/get_photos.sql");
    let photos = sqlx::query_as!(Photo, query)
        .fetch_all(pool.get_ref())
        .await;

    match photos {
        Ok(photos) => HttpResponse::Ok().json(photos.try_into(PhotoViewModel).unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
