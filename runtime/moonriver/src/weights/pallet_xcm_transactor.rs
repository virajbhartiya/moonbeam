// Copyright 2024 Moonbeam foundation
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

//! Autogenerated weights for `pallet_xcm_transactor`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-11-12, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-0-176`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("moonriver-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/moonbeam
// benchmark
// pallet
// --chain=moonriver-dev
// --steps=50
// --repeat=20
// --pallet=pallet_xcm_transactor
// --extrinsic=*
// --wasm-execution=compiled
// --header=./file_header.txt
// --template=./benchmarking/frame-weight-template.hbs
// --output=./runtime/moonriver/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for `pallet_xcm_transactor`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_xcm_transactor::WeightInfo for WeightInfo<T> {
	/// Storage: `XcmTransactor::IndexToAccount` (r:1 w:1)
	/// Proof: `XcmTransactor::IndexToAccount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `3541`
		// Minimum execution time: 8_580_000 picoseconds.
		Weight::from_parts(8_944_000, 3541)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `XcmTransactor::IndexToAccount` (r:0 w:1)
	/// Proof: `XcmTransactor::IndexToAccount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn deregister() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_720_000 picoseconds.
		Weight::from_parts(5_947_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `XcmTransactor::TransactInfoWithWeightLimit` (r:0 w:1)
	/// Proof: `XcmTransactor::TransactInfoWithWeightLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_transact_info() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_734_000 picoseconds.
		Weight::from_parts(6_944_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `XcmTransactor::TransactInfoWithWeightLimit` (r:0 w:1)
	/// Proof: `XcmTransactor::TransactInfoWithWeightLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_transact_info() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_266_000 picoseconds.
		Weight::from_parts(6_496_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `XcmTransactor::DestinationAssetFeePerSecond` (r:0 w:1)
	/// Proof: `XcmTransactor::DestinationAssetFeePerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_fee_per_second() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_319_000 picoseconds.
		Weight::from_parts(6_639_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `AssetManager::AssetIdType` (r:1 w:0)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::IndexToAccount` (r:1 w:0)
	/// Proof: `XcmTransactor::IndexToAccount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::RelayIndices` (r:1 w:0)
	/// Proof: `XcmTransactor::RelayIndices` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::TransactInfoWithWeightLimit` (r:1 w:0)
	/// Proof: `XcmTransactor::TransactInfoWithWeightLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::DestinationAssetFeePerSecond` (r:1 w:0)
	/// Proof: `XcmTransactor::DestinationAssetFeePerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeId` (r:1 w:0)
	/// Proof: `AssetManager::AssetTypeId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	fn transact_through_derivative() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `451`
		//  Estimated: `3916`
		// Minimum execution time: 29_010_000 picoseconds.
		Weight::from_parts(29_506_000, 3916)
			.saturating_add(T::DbWeight::get().reads(7_u64))
	}
	/// Storage: `AssetManager::AssetIdType` (r:1 w:0)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::TransactInfoWithWeightLimit` (r:1 w:0)
	/// Proof: `XcmTransactor::TransactInfoWithWeightLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::DestinationAssetFeePerSecond` (r:1 w:0)
	/// Proof: `XcmTransactor::DestinationAssetFeePerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeId` (r:1 w:0)
	/// Proof: `AssetManager::AssetTypeId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	fn transact_through_sovereign() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `389`
		//  Estimated: `3854`
		// Minimum execution time: 24_301_000 picoseconds.
		Weight::from_parts(25_355_000, 3854)
			.saturating_add(T::DbWeight::get().reads(5_u64))
	}
	/// Storage: `AssetManager::AssetIdType` (r:1 w:0)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::TransactInfoWithWeightLimit` (r:1 w:0)
	/// Proof: `XcmTransactor::TransactInfoWithWeightLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::DestinationAssetFeePerSecond` (r:1 w:0)
	/// Proof: `XcmTransactor::DestinationAssetFeePerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn transact_through_signed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `433`
		//  Estimated: `3898`
		// Minimum execution time: 39_427_000 picoseconds.
		Weight::from_parts(40_130_000, 3898)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `XcmTransactor::RelayIndices` (r:1 w:0)
	/// Proof: `XcmTransactor::RelayIndices` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetIdType` (r:1 w:0)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::TransactInfoWithWeightLimit` (r:1 w:0)
	/// Proof: `XcmTransactor::TransactInfoWithWeightLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::DestinationAssetFeePerSecond` (r:1 w:0)
	/// Proof: `XcmTransactor::DestinationAssetFeePerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn hrmp_manage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `433`
		//  Estimated: `3898`
		// Minimum execution time: 41_652_000 picoseconds.
		Weight::from_parts(42_393_000, 3898)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}
