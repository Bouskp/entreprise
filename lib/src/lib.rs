pub mod Operations;
pub mod Structure;
pub mod Worker;
pub mod Departement;


#[cfg(test)]
mod Tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(1+2, 2);
	}
}