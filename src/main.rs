extern crate lib;
use std::error::Error;
use std::process::exit;
use std::result;
use lib::{create, delete, Structure};

#[tokio::main]
async fn main() {
    let result = run().await;
    if let Ok(_value) = result {

    } else if let Err(err) = result {
        println!("Une erreur applicative : {}", err);
        exit(1);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let structure = Structure::new("workers_rust", "worker").await?;
    let result = delete(&structure, "Kouamé").await?;
    println!("{:?}", result.deleted_count);
    Ok(())
}