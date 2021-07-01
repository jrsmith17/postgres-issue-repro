use postgres::*;
use postgres_types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
#[postgres(name = "Placeholder")]
pub struct Placeholder {
    pub app_id: String,
    pub url: String
}

fn main() {
    println!("Hello, world!");
}
