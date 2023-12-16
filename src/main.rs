extern crate lib;
use std::error::Error;
use std::process::exit;
use lib::Operations::Operations::{ajouter_dept, cherhcher_dept, create, delete, delete_many, get_id_by_nom, supp_dept};
use lib::Structure::Structure::{StrucDept, Structure};

#[tokio::main]
async fn main() {
    let result = run().await;
    if let Err(err) = result {
        println!("Une erreur applicative : {}", err);
        exit(1);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let structure = Structure::new("workers_rust", "worker").await?;
    let dept_strucutre = StrucDept::new("workers_rust", "departement").await?;
    let result = create(&structure, &dept_strucutre, "Koffi", "Ange Germaine", 29, 450000, "Finance").await?;

    println!("{:?}", result);
    Ok(())
}