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


//! # Example
//!
//! Does nothing.
//!
//! ## Overview
//!
//! (Should be high-level details that are relevant to the most broad audience.)
//!
//! (The audience here is potentially non-coders who just want to know what this pallet does, not how it does it.)
//!
//! (potentially a few paragraphs, focus on what external folks should know about the pallet.)
//!
//! ### Example
//!
//! (Your pallet must have a few tests that cover important user journeys. Use <https://crates.io/crates/docify> to reuse these as examples).
//!
//! ## Pallet API
//!
//! (Reminder: inside the [`pallet`] module, a template that leads the reader to the relevant items is auto-generated. There is no need to repeat things like "See Config trait for ...", which are generated inside [`pallet`] here anyways. You can use the below line as-is:)
//!
//! See the [`pallet`] module for more information about the interfaces this pallet exposes, including its configuration trait, dispatchables, storage items, events and errors.
//!
//! (The audience of this is those who want to know how this pallet works, to the extent of being able to build something on top of it, like a DApp or another pallet.)
//!
//! This section can most often be left as-is.
//!
//! ## Low Level / Implementation Details
//!
//! (The format of this section is up to you, but we suggest the Design-oriented approach that follows.)
//!
//! (The audience of this would be your future self, or anyone who wants to gain a deep understanding of how the pallet works so that they can eventually propose optimizations to it.)
//!
//! ### Design Goals (optional)
//!
//! (Describe your goals with the pallet design.)
//!
//! ### Design (optional)
//!
//! (Describe how you've reached those goals. This should describe the storage layout of your pallet and what was your approach in designing it that way.)
//!
//! ### Terminology (optional)
//!
//! (Optionally, explain any non-obvious terminology here. You can link to it if you want to use the terminology further up.)
//! # License
//! 
//! Apache-2.0
//! 
//! # References
//!
//! This crate was auto generated by FRAMY CLI <https://crates.io/crates/framy>.  
//! Please report bugs to <https://github.com/ggwpez/framy/issues>.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

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
		/// The overarching event type of the runtime.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Weight information for all calls of this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	pub type DummyValue<T> = StorageValue<_, u32>;
	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The value was changed to the new value.
		Changed {
			/// The new value.
			value: u32,
		}
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The value was too large.
		TooLarge,
	}

	#[pallet::call(weight(<T as Config>::WeightInfo))]
	impl<T: Config> Pallet<T> {
		/// Change the value stored in storage.
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
			Self::deposit_event(Event::Changed { value });

			// Error if `value` is too large and revert the storage changes plus event emission.
			ensure!(value < 10, Error::<T>::TooLarge);
			
			Ok(())
		}
	}
}
