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


//! Autogenerated weights for pallet_crowdloan_rewards
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-0-36`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// /home/ubuntu/moonbeam
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// *
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --json-file
// raw.json
// --output
// weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_crowdloan_rewards.
pub trait WeightInfo {
	fn initialize_reward_vec(x: u32, ) -> Weight;
	fn complete_initialization() -> Weight;
	fn claim() -> Weight;
	fn update_reward_address() -> Weight;
	fn associate_native_identity() -> Weight;
	fn change_association_with_relay_keys(x: u32, ) -> Weight;
}

/// Weights for pallet_crowdloan_rewards using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: CrowdloanRewards Initialized (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards Initialized (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards InitializedRewardAmount (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards InitializedRewardAmount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards TotalContributors (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards TotalContributors (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:501 w:501)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: CrowdloanRewards ClaimedRelayChainIds (r:500 w:500)
	/// Proof Skipped: CrowdloanRewards ClaimedRelayChainIds (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards UnassociatedContributions (r:500 w:0)
	/// Proof Skipped: CrowdloanRewards UnassociatedContributions (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards AccountsPayable (r:500 w:500)
	/// Proof Skipped: CrowdloanRewards AccountsPayable (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[1, 500]`.
	fn initialize_reward_vec(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `75944 + x * (659 ±0)`
		//  Estimated: `398492 + x * (14132 ±0)`
		// Minimum execution time: 157_343_000 picoseconds.
		Weight::from_parts(112_720_307, 398492)
			// Standard Error: 31_246
			.saturating_add(Weight::from_parts(62_816_545, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(x.into())))
			.saturating_add(Weight::from_parts(0, 14132).saturating_mul(x.into()))
	}
	/// Storage: CrowdloanRewards Initialized (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards Initialized (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards InitRelayBlock (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards InitRelayBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards InitializedRewardAmount (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards InitializedRewardAmount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: CrowdloanRewards TotalContributors (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards TotalContributors (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards EndRelayBlock (r:0 w:1)
	/// Proof Skipped: CrowdloanRewards EndRelayBlock (max_values: Some(1), max_size: None, mode: Measured)
	fn complete_initialization() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `513`
		//  Estimated: `12086`
		// Minimum execution time: 26_631_000 picoseconds.
		Weight::from_parts(27_496_000, 12086)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: CrowdloanRewards Initialized (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards Initialized (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards AccountsPayable (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards AccountsPayable (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainSystem ValidationData (r:1 w:0)
	/// Proof Skipped: ParachainSystem ValidationData (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards InitRelayBlock (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards InitRelayBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards EndRelayBlock (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards EndRelayBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	fn claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `935`
		//  Estimated: `20252`
		// Minimum execution time: 67_906_000 picoseconds.
		Weight::from_parts(70_803_000, 20252)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: CrowdloanRewards AccountsPayable (r:2 w:2)
	/// Proof Skipped: CrowdloanRewards AccountsPayable (max_values: None, max_size: None, mode: Measured)
	fn update_reward_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `397`
		//  Estimated: `6337`
		// Minimum execution time: 30_064_000 picoseconds.
		Weight::from_parts(30_825_000, 6337)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: CrowdloanRewards UnassociatedContributions (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards UnassociatedContributions (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards ClaimedRelayChainIds (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards ClaimedRelayChainIds (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards AccountsPayable (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards AccountsPayable (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	fn associate_native_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `769`
		//  Estimated: `18874`
		// Minimum execution time: 130_134_000 picoseconds.
		Weight::from_parts(131_976_000, 18874)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: CrowdloanRewards AccountsPayable (r:2 w:2)
	/// Proof Skipped: CrowdloanRewards AccountsPayable (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[1, 500]`.
	fn change_association_with_relay_keys(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `366 + x * (32 ±0)`
		//  Estimated: `6306 + x * (32 ±0)`
		// Minimum execution time: 92_053_000 picoseconds.
		Weight::from_parts(92_960_000, 6306)
			// Standard Error: 9_521
			.saturating_add(Weight::from_parts(55_220_012, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(x.into()))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: CrowdloanRewards Initialized (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards Initialized (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards InitializedRewardAmount (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards InitializedRewardAmount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards TotalContributors (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards TotalContributors (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:501 w:501)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: CrowdloanRewards ClaimedRelayChainIds (r:500 w:500)
	/// Proof Skipped: CrowdloanRewards ClaimedRelayChainIds (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards UnassociatedContributions (r:500 w:0)
	/// Proof Skipped: CrowdloanRewards UnassociatedContributions (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards AccountsPayable (r:500 w:500)
	/// Proof Skipped: CrowdloanRewards AccountsPayable (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[1, 500]`.
	fn initialize_reward_vec(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `75944 + x * (659 ±0)`
		//  Estimated: `398492 + x * (14132 ±0)`
		// Minimum execution time: 157_343_000 picoseconds.
		Weight::from_parts(112_720_307, 398492)
			// Standard Error: 31_246
			.saturating_add(Weight::from_parts(62_816_545, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().reads((4_u64).saturating_mul(x.into())))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(x.into())))
			.saturating_add(Weight::from_parts(0, 14132).saturating_mul(x.into()))
	}
	/// Storage: CrowdloanRewards Initialized (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards Initialized (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards InitRelayBlock (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards InitRelayBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards InitializedRewardAmount (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards InitializedRewardAmount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: CrowdloanRewards TotalContributors (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards TotalContributors (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards EndRelayBlock (r:0 w:1)
	/// Proof Skipped: CrowdloanRewards EndRelayBlock (max_values: Some(1), max_size: None, mode: Measured)
	fn complete_initialization() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `513`
		//  Estimated: `12086`
		// Minimum execution time: 26_631_000 picoseconds.
		Weight::from_parts(27_496_000, 12086)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: CrowdloanRewards Initialized (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards Initialized (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards AccountsPayable (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards AccountsPayable (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainSystem ValidationData (r:1 w:0)
	/// Proof Skipped: ParachainSystem ValidationData (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards InitRelayBlock (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards InitRelayBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards EndRelayBlock (r:1 w:0)
	/// Proof Skipped: CrowdloanRewards EndRelayBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	fn claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `935`
		//  Estimated: `20252`
		// Minimum execution time: 67_906_000 picoseconds.
		Weight::from_parts(70_803_000, 20252)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: CrowdloanRewards AccountsPayable (r:2 w:2)
	/// Proof Skipped: CrowdloanRewards AccountsPayable (max_values: None, max_size: None, mode: Measured)
	fn update_reward_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `397`
		//  Estimated: `6337`
		// Minimum execution time: 30_064_000 picoseconds.
		Weight::from_parts(30_825_000, 6337)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: CrowdloanRewards UnassociatedContributions (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards UnassociatedContributions (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards ClaimedRelayChainIds (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards ClaimedRelayChainIds (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrowdloanRewards AccountsPayable (r:1 w:1)
	/// Proof Skipped: CrowdloanRewards AccountsPayable (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	fn associate_native_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `769`
		//  Estimated: `18874`
		// Minimum execution time: 130_134_000 picoseconds.
		Weight::from_parts(131_976_000, 18874)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: CrowdloanRewards AccountsPayable (r:2 w:2)
	/// Proof Skipped: CrowdloanRewards AccountsPayable (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[1, 500]`.
	fn change_association_with_relay_keys(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `366 + x * (32 ±0)`
		//  Estimated: `6306 + x * (32 ±0)`
		// Minimum execution time: 92_053_000 picoseconds.
		Weight::from_parts(92_960_000, 6306)
			// Standard Error: 9_521
			.saturating_add(Weight::from_parts(55_220_012, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(x.into()))
	}
}