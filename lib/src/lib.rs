
use std::io;
use mongodb::Client;
use mongodb::options::ClientOptions;
use serde::{Deserialize, Serialize};

pub struct Strucuture<'a> {
	 db : &'a str,
	 column : &'a str,
}

impl Strucuture<'_> {
	pub fn new<'a>(str_connection : &'a str, db : &'a str, column : &'a str) -> Result<Strucuture<'a>, mongodb::error::Error> {


		Ok(Strucuture {
			db,
			column
		})
	}


	pub fn get_column<'a>(&'a self) -> &'a str {
		self.column
	}

	pub fn get_db<'a>(&'a self) -> &'a str { self.db}
}



pub fn connection()-> Result<Client, mongodb::error::Error> {
	let client_options = ClientOptions::parse("mongodb://localhost:27017")?;
	let client = Client::with_options(client_options)?;
	Ok(client)
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Worker {
	nom : String,
	prenom : String,
	age : u32,
	salaire : u64,
}

pub fn create() {

}

pub fn read() {

}

pub fn update() {

}

pub fn delete() {

}


#[cfg(test)]
mod Tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(1+2, 2);
	}
}