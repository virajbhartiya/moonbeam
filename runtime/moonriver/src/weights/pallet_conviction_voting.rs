// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_conviction_voting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-07-25, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-0-176`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("moonbase-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/moonbeam
// benchmark
// pallet
// --chain=moonbase-dev
// --steps=50
// --repeat=20
// --pallet=pallet_conviction_voting
// --extrinsic=*
// --wasm-execution=compiled
// --header=./file_header.txt
// --template=./benchmarking/frame-weight-template.hbs
// --output=./runtime/common/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for `pallet_conviction_voting`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_conviction_voting::WeightInfo for WeightInfo<T> {
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `ConvictionVoting::VotingFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(1152), added: 3627, mode: `MaxEncodedLen`)
	/// Storage: `ConvictionVoting::ClassLocksFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::ClassLocksFor` (`max_values`: None, `max_size`: Some(137), added: 2612, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(37), added: 2512, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn vote_new() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1962`
		//  Estimated: `42428`
		// Minimum execution time: 63_183_000 picoseconds.
		Weight::from_parts(64_601_000, 42428)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `ConvictionVoting::VotingFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(1152), added: 3627, mode: `MaxEncodedLen`)
	/// Storage: `ConvictionVoting::ClassLocksFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::ClassLocksFor` (`max_values`: None, `max_size`: Some(137), added: 2612, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(37), added: 2512, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn vote_existing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2263`
		//  Estimated: `83866`
		// Minimum execution time: 84_800_000 picoseconds.
		Weight::from_parts(85_964_000, 83866)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: `ConvictionVoting::VotingFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(1152), added: 3627, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn remove_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1841`
		//  Estimated: `83866`
		// Minimum execution time: 57_451_000 picoseconds.
		Weight::from_parts(58_541_000, 83866)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `ConvictionVoting::VotingFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(1152), added: 3627, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	fn remove_other_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1385`
		//  Estimated: `4617`
		// Minimum execution time: 19_644_000 picoseconds.
		Weight::from_parts(20_527_000, 4617)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ConvictionVoting::VotingFor` (r:2 w:2)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(1152), added: 3627, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumInfoFor` (r:20 w:20)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// Storage: `ConvictionVoting::ClassLocksFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::ClassLocksFor` (`max_values`: None, `max_size`: Some(137), added: 2612, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(37), added: 2512, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:20)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 20]`.
	fn delegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1621 + r * (248 ±0)`
		//  Estimated: `83866 + r * (3387 ±0)`
		// Minimum execution time: 42_633_000 picoseconds.
		Weight::from_parts(38_091_643, 83866)
			// Standard Error: 77_374
			.saturating_add(Weight::from_parts(26_302_549, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(6_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 3387).saturating_mul(r.into()))
	}
	/// Storage: `ConvictionVoting::VotingFor` (r:2 w:2)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(1152), added: 3627, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumInfoFor` (r:20 w:20)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:20)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 20]`.
	fn undelegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1340 + r * (248 ±0)`
		//  Estimated: `83866 + r * (3387 ±0)`
		// Minimum execution time: 19_458_000 picoseconds.
		Weight::from_parts(11_738_092, 83866)
			// Standard Error: 78_434
			.saturating_add(Weight::from_parts(25_800_751, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 3387).saturating_mul(r.into()))
	}
	/// Storage: `ConvictionVoting::VotingFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(1152), added: 3627, mode: `MaxEncodedLen`)
	/// Storage: `ConvictionVoting::ClassLocksFor` (r:1 w:1)
	/// Proof: `ConvictionVoting::ClassLocksFor` (`max_values`: None, `max_size`: Some(137), added: 2612, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(37), added: 2512, mode: `MaxEncodedLen`)
	fn unlock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1228`
		//  Estimated: `4752`
		// Minimum execution time: 46_940_000 picoseconds.
		Weight::from_parts(48_064_000, 4752)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}