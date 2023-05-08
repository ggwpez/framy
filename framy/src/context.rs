use convert_case::{Case, Casing};
use serde::Serialize;
use typesafe_builders::prelude::*;

#[derive(Serialize, Builder)]
pub struct Context {
	pub pallet: Pallet,
	pub cargo: Cargo,
}

#[derive(Serialize, Builder)]
pub struct Pallet {
	pub license_header: String,
	pub name: String,
	pub storage: Storage,
	pub event: Event,
	pub call: Call,
	pub error: Error,
}

pub fn folder_name(name: &str) -> String {
	let canon = name.to_case(Case::Snake);
	canon.strip_prefix("pallet_").unwrap_or(&canon).to_string()
}

#[derive(Serialize)]
pub struct Storage {
	pub dummy: bool,
}

#[derive(Serialize)]
pub struct Event {
	pub dummy: bool,
}

#[derive(Serialize)]
pub struct Call {
	pub dummy: bool,
}

#[derive(Serialize)]
pub struct Error {
	pub dummy: bool,
}

#[derive(Serialize, Builder)]
pub struct Cargo {
	pub module: String,
	pub version: String,
	pub author: String,
	pub description: String,
	pub license: String,
	pub repository: String,
	pub edition: String,
	pub homepage: String,
}
