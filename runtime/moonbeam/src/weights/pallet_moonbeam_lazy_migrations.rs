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

//! Autogenerated weights for `pallet_moonbeam_lazy_migrations`
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
// --pallet=pallet_moonbeam_lazy_migrations
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

/// Weights for `pallet_moonbeam_lazy_migrations`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_moonbeam_lazy_migrations::WeightInfo for WeightInfo<T> {
	/// Storage: `EVM::Suicided` (r:100 w:0)
	/// Proof: `EVM::Suicided` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `EVM::AccountCodes` (r:100 w:0)
	/// Proof: `EVM::AccountCodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `EVM::AccountStorages` (r:1000 w:900)
	/// Proof: `EVM::AccountStorages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `MoonbeamLazyMigrations::SuicidedContractsRemoved` (r:1 w:1)
	/// Proof: `MoonbeamLazyMigrations::SuicidedContractsRemoved` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[1, 100]`.
	/// The range of component `l` is `[1, 1000]`.
	fn clear_suicided_storage(a: u32, l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4251 + a * (12 ±0) + l * (84 ±0)`
		//  Estimated: `37759 + a * (2229 ±19) + l * (2536 ±1)`
		// Minimum execution time: 46_475_000 picoseconds.
		Weight::from_parts(47_461_000, 37759)
			// Standard Error: 2_615_231
			.saturating_add(Weight::from_parts(42_043_431, 0).saturating_mul(a.into()))
			// Standard Error: 261_244
			.saturating_add(Weight::from_parts(27_225_099, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(l.into())))
			.saturating_add(T::DbWeight::get().writes(41_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(l.into())))
			.saturating_add(Weight::from_parts(0, 2229).saturating_mul(a.into()))
			.saturating_add(Weight::from_parts(0, 2536).saturating_mul(l.into()))
	}
}
