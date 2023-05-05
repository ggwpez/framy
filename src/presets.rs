use crate::context::*;

/// Basic is used by every preset.
pub mod basic {
	use super::*;

	pub fn cargo() -> GenericCargoBuilder<false, true, false, false, false, false, true, false> {
		Cargo::builder()
			.version("0.1.0".to_string())
			.edition("2021".to_string())
	}

	pub fn pallet() -> GenericPalletBuilder<false, false, true, true, true, true> {
		Pallet::builder()
			.storage(Storage { dummy: true })
			.event(Event { dummy: true })
			.call(Call { dummy: true })
			.error(Error { dummy: true })
	}

	pub fn context() -> GenericContextBuilder<false, false, true, true, true, true> {
		Context::builder()
			.test(Test { dummy: true })
			.mock(Mock { dummy: true })
			.benchmarking(Benchmarking { dummy: true })
			.weights(Weights { dummy: true })
	}
}

/// DotSama is slightly opinionated...
pub mod dotsama {
	use super::*;

	pub fn cargo() -> GenericCargoBuilder<false, true, false, false, true, false, true, false> {
		basic::cargo()
			.license("Apache-2.0".to_string())
	}

	pub fn pallet() -> GenericPalletBuilder<false, false, true, true, true, true> {
		basic::pallet()
	}
}

/// Parity presets are heavily opinionated.
pub mod substrate {
	use super::*;

	pub fn cargo() -> GenericCargoBuilder<false, true, true, false, true, true, true, true> {
		dotsama::cargo()
			.author("Parity Technologies <admin@parity.io>".to_string())
			.repository("https://github.com/paritytech/substrate".to_string())
			.homepage("https://substrate.io".to_string())
	}

	pub fn pallet() -> GenericPalletBuilder<true, false, true, true, true, true> {
		dotsama::pallet()
			.license_header(include_str!("../templates/HEADER-SUBSTRATE").to_string())
	}
}
