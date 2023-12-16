pub mod Worker {
	use bson::oid::ObjectId;
	use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
	pub struct Worker {
		#[serde(rename = "_id")]
		pub id : ObjectId,
		pub nom : String,
		pub prenom : String,
		pub age : u64,
		pub salaire : u64,
		pub depart_id : ObjectId,
	}


}