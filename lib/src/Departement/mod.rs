pub mod Departement {
	use bson::doc;
	use bson::oid::ObjectId;
	use mongodb::results::InsertOneResult;
	use serde::{Deserialize, Serialize};
	use crate::Structure::Structure::{StrucDept, Structure};

	#[derive(Serialize, Deserialize, Debug)]
	pub struct Departement {
		#[serde(rename="_id")]
		pub id : ObjectId,
		pub nom : String,
	}

}