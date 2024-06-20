use emberflix::{establish_connection, router::*};
use rocket::*;

#[launch]
async fn rocket() -> _ {
  let db = match establish_connection().await {
    Ok(db) => db,
    Err(e) => panic!("{}", e),
  };

  rocket::build().manage(db).mount("/", routes![index])
}
