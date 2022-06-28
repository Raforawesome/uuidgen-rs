mod gen;
pub use gen::{ gen_uuid, gen_id };

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
	fn test_id() {
		let id: String = gen_id(false);
		let id2: String = gen_id(true);
		println!("Non-dashed: {id}\nDashed: {id2}")
	}

	#[test]
	fn test_uuid() {
		let id: String = gen_uuid(false);
		let id2: String = gen_uuid(true);
		println!("Non-dashed: {id}\nDashed: {id2}")
	}
}
