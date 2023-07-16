use anyhow::Result;

#[derive(sqlx::Type)]
#[sqlx(transparent)]
struct Id(i32);

pub fn add_photo() -> Result<()> {
    Ok(())
}
