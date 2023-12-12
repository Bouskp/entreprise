use bson::{Document, to_document,doc};
use mongodb::{Client, Collection, Database};
use mongodb::results::{DeleteResult, InsertOneResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Structure {
	 db : Database,
	 column : Collection<Worker>,
	 client : Client
}

impl Structure {
	pub async fn new(db : &str, column : &str) -> Result<Structure, mongodb::error::Error> {
		let client = Client::with_uri_str("mongodb://localhost:27017").await?;
		let database = client.database(db);
		let collection = database.collection::<Worker>(column);
		Ok(Structure {
			db : database,
			column : collection,
			client
		})
	}

	pub fn get_client(&self)-> Client {
		self.client.clone()
	}

	pub fn get_db(&self) -> Database {
		self.db.clone()
	}

	pub fn get_column(&self) -> Collection<Worker> { self.column.clone()}
}





#[derive(Debug, Serialize, Deserialize)]
pub struct Worker {
	 pub nom : String,
	 pub prenom : String,
	 pub age : u64,
	 pub salaire : u64,
}




 pub async fn create(structure: &Structure)->Result<InsertOneResult, mongodb::error::Error> {
	 let worker = Worker {
		 nom : "Kouakou".to_string(),
		 prenom: "Manien Hugues-Derek".to_string(),
		 age: 24,
		 salaire: 450000,
	 };

	let result = structure.get_column().insert_one(worker, None).await?;
	 Ok(result)
}

pub fn read() {

}

pub fn update() {

}

pub async  fn delete(structure: &Structure, nom : &str)->Result<DeleteResult, mongodb::error::Error> {
	let filter = doc! {"nom" : nom};
	let result = structure.get_column().delete_one(filter, None).await?;
	Ok(result)

}


#[cfg(test)]
mod Tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(1+2, 2);
	}
}