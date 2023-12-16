
pub mod Structure {
use mongodb::{Database, Client, Collection};
	use crate::Departement::Departement::Departement;
	use crate::Worker::Worker::Worker;

	#[derive(Debug, Clone)]
	pub struct Structure {
		db: Database,
		column: Collection<Worker>,
		client: Client
	}

	#[derive(Debug, Clone)]
	pub struct StrucDept {
		db: Database,
		column: Collection<Departement>,
		client: Client
	}

	impl Structure {
		pub async fn new(db: &str, column: &str) -> Result<Structure, mongodb::error::Error> {
			let client = Client::with_uri_str("mongodb://localhost:27017").await?;
			let database = client.database(db);
			let collection = database.collection::<Worker>(column);
			Ok(Structure {
				db: database,
				column: collection,
				client
			})
		}

		pub fn get_client(&self) -> Client {
			self.client.clone()
		}

		pub fn get_db(&self) -> Database {
			self.db.clone()
		}

		pub fn get_column(&self) -> Collection<Worker> { self.column.clone() }
	}

	impl StrucDept {
		pub async fn new(db : &str, column : &str)-> Result<StrucDept, mongodb::error::Error> {
			let client = Client::with_uri_str("mongodb://localhost:27017").await?;
			let database = client.database(db);
			let collection = database.collection::<Departement>(column);
			Ok(StrucDept {
				db: database,
				column: collection,
				client
			})
		}

		pub fn get_client(&self) -> Client {
			self.client.clone()
		}

		pub fn get_db(&self) -> Database { self.db.clone()}

		pub fn get_column(&self) -> Collection<Departement> {
			self.column.clone()
		}
	}
}

