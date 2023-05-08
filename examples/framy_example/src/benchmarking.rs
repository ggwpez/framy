// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benches {
	use super::*;

	/// Benchmarks the slowest path of `change_value`.
	#[benchmark]
	fn change_value() -> Result<(), BenchmarkError> {
		let caller: T::AccountId = whitelisted_caller();

		// You can mock the storage here:
		DummyValue::<T>::put(1);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), 9);

		Ok(())
	}

	// Implements a test for each benchmark. Execute with:
	// `cargo test -p pallet-framyExample --features runtime-benchmarks`.
	impl_benchmark_test_suite!(
		Framyexample,
		crate::mock::new_test_ext(),
		crate::mock::Test
	);
}
