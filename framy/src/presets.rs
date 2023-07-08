use crate::context::*;

/// Known good version of Substrate that should be used to fix dependencies.
const SUBSTRATE_GOOD_REV: &str = "70c0547f40daa8b550ef1a849514d07c385f963e";

/// Basic is used by every other preset.
pub mod basic {
	use super::*;

	pub fn cargo() -> GenericCargoBuilder<false, true, false, false, false, false, true, false, true>
	{
		Cargo::builder()
			.version("0.1.0".to_string())
			.edition("2021".to_string())
			.substrate_rev(SUBSTRATE_GOOD_REV.into())
	}

	pub fn pallet() -> GenericPalletBuilder<false, false, true, true, true, true> {
		Pallet::builder()
			.storage(Storage { dummy: true })
			.event(Event { dummy: true })
			.call(Call { dummy: true })
			.error(Error { dummy: true })
	}

	pub fn context() -> GenericContextBuilder<false, false> {
		Context::builder()
	}
}

/// Parity presets are heavily opinionated.
pub mod substrate {
	use super::*;

	pub fn cargo() -> GenericCargoBuilder<false, true, true, false, true, true, true, true, true> {
		basic::cargo()
			.author("Parity Technologies <admin@parity.io>".to_string())
			.repository("https://github.com/paritytech/substrate".to_string())
			.homepage("https://substrate.io".to_string())
			.license("Apache-2.0".to_string())
	}

	pub fn pallet() -> GenericPalletBuilder<true, false, true, true, true, true> {
		basic::pallet().license_header(include_str!("../templates/HEADER-SUBSTRATE").to_string())
	}
}
