// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for referendum
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./../target/release/joystream-node
// benchmark
// pallet
// --base-path=/mnt/disks/local-ssd/
// --pallet=referendum
// --extrinsic=*
// --chain=dev
// --steps=50
// --repeat=20
// --execution=wasm
// --template=./../devops/joystream-pallet-weight-template.hbs
// --output=./../runtime-modules/referendum/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for referendum.
pub trait WeightInfo {
	fn on_initialize_revealing(_i: u32, ) -> Weight;
	fn on_initialize_voting() -> Weight;
	fn vote() -> Weight;
	fn reveal_vote_space_for_new_winner(_i: u32, ) -> Weight;
	fn reveal_vote_space_not_in_winners(_i: u32, ) -> Weight;
	fn reveal_vote_space_replace_last_winner(_i: u32, ) -> Weight;
	fn reveal_vote_already_existing(_i: u32, ) -> Weight;
	fn release_vote_stake() -> Weight;
	fn opt_out_of_voting() -> Weight;
}

/// Weights for referendum using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Instance1Referendum Stage (r:1 w:1)
	// Storage: Council AnnouncementPeriodNr (r:1 w:1)
	// Storage: Council Stage (r:0 w:1)
	fn on_initialize_revealing(i: u32, ) -> Weight {
		(32_918_000 as Weight)
			// Standard Error: 86_000
			.saturating_add((1_424_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Referendum Stage (r:1 w:1)
	fn on_initialize_voting() -> Weight {
		(18_880_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance1Referendum AccountsOptedOut (r:1 w:0)
	// Storage: Instance1Referendum Stage (r:1 w:0)
	// Storage: Instance1Referendum Votes (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn vote() -> Weight {
		(43_910_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Referendum Stage (r:1 w:1)
	// Storage: Instance1Referendum Votes (r:1 w:1)
	// Storage: Council Candidates (r:1 w:1)
	// Storage: Council AnnouncementPeriodNr (r:1 w:0)
	fn reveal_vote_space_for_new_winner(i: u32, ) -> Weight {
		(45_258_000 as Weight)
			// Standard Error: 144_000
			.saturating_add((2_127_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Referendum Stage (r:1 w:1)
	// Storage: Instance1Referendum Votes (r:1 w:1)
	// Storage: Council Candidates (r:1 w:1)
	// Storage: Council AnnouncementPeriodNr (r:1 w:0)
	fn reveal_vote_space_not_in_winners(i: u32, ) -> Weight {
		(47_590_000 as Weight)
			// Standard Error: 58_000
			.saturating_add((1_263_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Referendum Stage (r:1 w:1)
	// Storage: Instance1Referendum Votes (r:1 w:1)
	// Storage: Council Candidates (r:1 w:1)
	// Storage: Council AnnouncementPeriodNr (r:1 w:0)
	fn reveal_vote_space_replace_last_winner(i: u32, ) -> Weight {
		(47_356_000 as Weight)
			// Standard Error: 72_000
			.saturating_add((1_157_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Referendum Stage (r:1 w:1)
	// Storage: Instance1Referendum Votes (r:1 w:1)
	// Storage: Council Candidates (r:1 w:1)
	// Storage: Council AnnouncementPeriodNr (r:1 w:0)
	fn reveal_vote_already_existing(i: u32, ) -> Weight {
		(46_890_000 as Weight)
			// Standard Error: 74_000
			.saturating_add((1_400_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Referendum Votes (r:1 w:1)
	// Storage: Council AnnouncementPeriodNr (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn release_vote_stake() -> Weight {
		(46_810_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Referendum AccountsOptedOut (r:0 w:1)
	fn opt_out_of_voting() -> Weight {
		(16_920_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn on_initialize_revealing(i: u32, ) -> Weight {
		0
	}
	fn on_initialize_voting() -> Weight {
		0
	}
	fn vote() -> Weight {
		0
	}
	fn reveal_vote_space_for_new_winner(i: u32, ) -> Weight {
		0
	}
	fn reveal_vote_space_not_in_winners(i: u32, ) -> Weight {
		0
	}
	fn reveal_vote_space_replace_last_winner(i: u32, ) -> Weight {
		0
	}
	fn reveal_vote_already_existing(i: u32, ) -> Weight {
		0
	}
	fn release_vote_stake() -> Weight {
		0
	}
	fn opt_out_of_voting() -> Weight {
		0
	}
}
