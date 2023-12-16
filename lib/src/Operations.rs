pub mod Operations {
	use bson::oid::ObjectId;
	use mongodb::bson::{doc};
	use mongodb::results::{InsertOneResult, UpdateResult, DeleteResult};
	use crate::Departement::Departement::Departement;
	use crate::Structure::Structure::{Structure, StrucDept};
	use crate::Worker::Worker::Worker;

	pub async fn create(structure: &Structure,struc_dept: &StrucDept, nom : &str, prenom : &str, age : u64, salaire : u64, departement: &str)->Result<InsertOneResult, mongodb::error::Error> {
		let dept_id = get_id_by_nom(struc_dept, departement).await?;

		let worker = Worker {
		id : ObjectId::new(),
		nom : nom.to_string(),
		prenom: prenom.to_string(),
		age: age,
		salaire: salaire,
		depart_id : dept_id,
	};

	let result = structure.get_column().insert_one(worker, None).await?;
	Ok(result)
}

pub async  fn read(structure: &Structure, nom: &str) -> Result<Worker, mongodb::error::Error> {
	let filter = doc! {
		"nom" : nom,
	};
	let result = structure.get_column().find_one(filter, None).await?.unwrap();
	Ok(result)

}

pub async  fn update(structure: &Structure, nom : &str) -> Result<UpdateResult, mongodb::error::Error> {
	let filter = doc! {"nom" : nom};
	let update = doc ! {"$set" : {"age" : 25, "prenom" : "Bouskp225"}};
	let result = structure.get_column().update_one(filter, update, None).await?;
	Ok(result)

}

pub async  fn delete(structure: &Structure, nom : &str)->Result<DeleteResult, mongodb::error::Error> {
	let filter = doc! {"nom" : nom};
	let result = structure.get_column().delete_one(filter, None).await?;
	Ok(result)

}

pub async fn delete_many(structure : &Structure, nom : &str) -> Result<DeleteResult, mongodb::error::Error> {
	let filter = doc! {"nom" : nom};
	let result = structure.get_column().delete_many(filter, None).await?;
	Ok(result)
}

pub async fn ajouter_dept(structure : &StrucDept, nom : &str) -> Result<InsertOneResult, mongodb::error::Error> {

	let depart = Departement {
		nom : nom.to_string(),
		id : ObjectId::new()
	};
	let result = structure.get_column().insert_one(depart, None).await?;
	Ok(result)

}

pub async fn supp_dept(struc_dept: &StrucDept, nom : &str) -> Result<DeleteResult, mongodb::error::Error> {
	let nom = nom.to_lowercase();
	let filter = doc! {"nom" : nom};
	let result = struc_dept.get_column().delete_one(filter, None).await?;
	Ok(result)
}

pub async fn modif_dept(struc_dept: &StrucDept, nom : &str, nouveau : &str) -> Result<UpdateResult, mongodb::error::Error> {
	let nom = nom.to_lowercase();
	let filter = doc! {"nom" : nom};
	let update = doc! {"$set" : {"nom" : nouveau}};
	let result = struc_dept.get_column().update_one(filter, update, None).await?;
	Ok(result)
}

pub async fn cherhcher_dept(struc_dept: &StrucDept, nom : &str) -> Result<Departement, mongodb::error::Error> {
	let filter = doc! {"nom" : nom};
	let resultat = struc_dept.get_column().find_one(filter,None ).await?.unwrap();
	Ok(resultat)

}

pub async fn get_id_by_nom(struc_dept: &StrucDept, nom : &str) ->Result<ObjectId, mongodb::error::Error> {
	let filter = doc! {"nom" : nom};
	let resultat = struc_dept.get_column().find_one(filter,None ).await?.unwrap().id;
	Ok(resultat)
}

}
