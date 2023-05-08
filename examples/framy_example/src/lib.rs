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

#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

pub use pallet::*;

mod mock;
mod tests;
mod benchmarking;

pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Weight information for all calls of this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	pub type DummyValue<T> = StorageValue<_, u32>;
	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Changed {
			// You can use named fields here.
		}
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		TooLarge, // Errors cannot have fields.
	}

	#[pallet::call(weight(<T as Config>::WeightInfo))]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		pub fn change_value(origin: OriginFor<T>, value: u32) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			
			// The actual logic is in a separate function to ease testing and implementing traits.
			Self::do_change_value(value).map_err(Into::into)
		}
	}

	impl<T: Config> Pallet<T> {
		/// Logic for call `Self::change_value`.
		pub(crate) fn do_change_value(value: u32) -> Result<(), Error<T>> {
			// Update storage.
			DummyValue::<T>::put(value);

			// Emit an event.
			Self::deposit_event(Event::Changed { });

			// Error if `value` is too large and revert the storage changes.
			ensure!(value < 10, Error::<T>::TooLarge);
			
			Ok(())
		}
	}
}
