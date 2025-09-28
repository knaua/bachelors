
use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::Connection;
use crate::Db;

async fn read(mut db: Box<dyn Connection<Database=Db, Options=()>>) {
    let x = sqlx::query("SELECT * FROM devices WHERE available = 1")
        .fetch_all(&mut *db).await;

    let y = x.unwrap();

    println!("{:?}", y)

}

fn write() {

}