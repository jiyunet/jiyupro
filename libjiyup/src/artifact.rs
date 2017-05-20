use chain::Address;

trait Artifact {
	fn get_type(&self) -> u16;
	fn get_version(&self) -> u16;
	fn get_author(&self) -> &Address;
	fn get_timestamp(&self) -> u64;
}
