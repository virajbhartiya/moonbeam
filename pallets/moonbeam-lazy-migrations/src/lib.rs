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

//! # Lazy Migration Pallet

#![allow(non_camel_case_types)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod benchmarks;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub mod weights;
pub use weights::WeightInfo;

use frame_support::pallet;

pub use pallet::*;

#[pallet]
pub mod pallet {
	use super::*;
	use cumulus_primitives_storage_weight_reclaim::get_proof_size;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_core::H160;
	use sp_std::if_std;

	pub const ARRAY_LIMIT: u32 = 1000;
	pub type GetArrayLimit = ConstU32<ARRAY_LIMIT>;

	/// Pallet for multi block migrations
	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::storage]
	/// The total number of suicided contracts that were removed
	pub(crate) type SuicidedContractsRemoved<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	pub(crate) type StateMigrationStatusValue<T: Config> =
		StorageValue<_, StateMigrationStatus, ValueQuery>;

	pub(crate) type StorageKey = BoundedVec<u8, ConstU32<512>>;

	#[derive(Clone, Encode, Decode, scale_info::TypeInfo, PartialEq, Eq, MaxEncodedLen, Debug)]
	pub enum StateMigrationStatus {
		NotStarted,
		Started(StorageKey),
		Error(BoundedVec<u8, ConstU32<1024>>),
		Complete,
	}

	impl Default for StateMigrationStatus {
		fn default() -> Self {
			return StateMigrationStatus::NotStarted;
		}
	}

	/// Configuration trait of this pallet.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_evm::Config + pallet_balances::Config {
		type WeightInfo: WeightInfo;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The limit cannot be zero
		LimitCannotBeZero,
		/// There must be at least one address
		AddressesLengthCannotBeZero,
		/// The contract is not corrupted (Still exist or properly suicided)
		ContractNotCorrupted,
		/// The key lengths exceeds the maximum allowed
		KeyTooLong,
	}

	pub(crate) const MAX_ITEM_PROOF_SIZE: u64 = 30 * 1024; // 30 KB
	pub(crate) const PROOF_SIZE_BUFFER: u64 = 100 * 1024; // 100 KB

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_idle(_n: BlockNumberFor<T>, remaining_weight: Weight) -> Weight {
			let proof_size_before: u64 = get_proof_size().unwrap_or(0);
			let res = Pallet::<T>::handle_migration(remaining_weight);
			if_std! {
				println!(
					"State migration: reads: {}, writes: {}",
					res.reads,
					res.writes
				);
			}
			let proof_size_after: u64 = get_proof_size().unwrap_or(0);
			let proof_size_diff = proof_size_after.saturating_sub(proof_size_before);

			Weight::from_parts(0, proof_size_diff)
				.saturating_add(T::DbWeight::get().reads_writes(res.reads, res.writes))
		}
	}

	#[derive(Default, Clone, PartialEq, Eq, Encode, Decode, Debug)]
	pub(crate) struct ReadWriteOps {
		pub reads: u64,
		pub writes: u64,
	}

	impl ReadWriteOps {
		pub fn new() -> Self {
			Self {
				reads: 0,
				writes: 0,
			}
		}

		pub fn add_one_read(&mut self) {
			self.reads += 1;
		}

		pub fn add_one_write(&mut self) {
			self.writes += 1;
		}

		pub fn add_reads(&mut self, reads: u64) {
			self.reads += reads;
		}

		pub fn add_writes(&mut self, writes: u64) {
			self.writes += writes;
		}
	}

	#[derive(Clone)]
	struct StateMigrationResult {
		last_key: Option<StorageKey>,
		error: Option<&'static str>,
		reads: u64,
		writes: u64,
	}

	enum NextKeyResult {
		NextKey(StorageKey),
		NoMoreKeys,
		Error(&'static str),
	}

	impl<T: Config> Pallet<T> {
		/// Handle the migration of the storage keys, returns the number of read and write operations
		pub(crate) fn handle_migration(remaining_weight: Weight) -> ReadWriteOps {
			let mut read_write_ops = ReadWriteOps::new();

			// maximum number of items that can be migrated in one block
			let migration_limit = remaining_weight
				.proof_size()
				.saturating_sub(PROOF_SIZE_BUFFER)
				.saturating_div(MAX_ITEM_PROOF_SIZE);

			if migration_limit == 0 {
				return read_write_ops;
			}

			if_std! {
				println!("Migration limit: {}", migration_limit);
			}

			let status = StateMigrationStatusValue::<T>::get();
			if_std! {
				println!("r++ State migration status: {:?}", status);
			}
			read_write_ops.add_one_read();

			let next_key = match &status {
				StateMigrationStatus::NotStarted => Default::default(),
				StateMigrationStatus::Started(storage_key) => {
					let (reads, next_key_result) = Pallet::<T>::get_next_key(storage_key);
					read_write_ops.add_reads(reads);
					match next_key_result {
						NextKeyResult::NextKey(next_key) => next_key,
						NextKeyResult::NoMoreKeys => {
							StateMigrationStatusValue::<T>::put(StateMigrationStatus::Complete);
							if_std! {
								println!("w++ StateMigrationStatus::Complete 0");
							}
							read_write_ops.add_one_write();
							return read_write_ops;
						}
						NextKeyResult::Error(e) => {
							StateMigrationStatusValue::<T>::put(StateMigrationStatus::Error(
								e.as_bytes().to_vec().try_into().unwrap_or_default(),
							));
							if_std! {
								println!("w++ StateMigrationStatus::Error 0");
							}
							read_write_ops.add_one_write();
							return read_write_ops;
						}
					}
				}
				StateMigrationStatus::Complete | StateMigrationStatus::Error(_) => {
					return read_write_ops;
				}
			};

			let res = Pallet::<T>::migrate_keys(next_key, migration_limit);
			if_std! {
				println!("!! migrate_keys {} reads", res.reads);
				println!("!! migrate_keys {} writes", res.writes);
			}
			read_write_ops.add_reads(res.reads);
			read_write_ops.add_writes(res.writes);

			match (res.last_key, res.error) {
				(None, None) => {
					StateMigrationStatusValue::<T>::put(StateMigrationStatus::Complete);
					if_std! {
						println!("w++ StateMigrationStatus::Complete");
					}
					read_write_ops.add_one_write();
				}
				// maybe we should store the previous key in the storage as well
				(_, Some(e)) => {
					StateMigrationStatusValue::<T>::put(StateMigrationStatus::Error(
						e.as_bytes().to_vec().try_into().unwrap_or_default(),
					));
					if_std! {
						println!("w++ StateMigrationStatus::Error");
					}
					read_write_ops.add_one_write();
				}
				(Some(key), None) => {
					StateMigrationStatusValue::<T>::put(StateMigrationStatus::Started(key));
					if_std! {
						println!("w++ StateMigrationStatus::Started");
					}
					read_write_ops.add_one_write();
				}
			}

			read_write_ops
		}

		/// Tries to get the next key in the storage, returns None if there are no more keys to migrate.
		/// Returns an error if the key is too long.
		fn get_next_key(key: &StorageKey) -> (u64, NextKeyResult) {
			if let Some(next) = sp_io::storage::next_key(key) {
				let next: Result<StorageKey, _> = next.try_into();
				match next {
					Ok(next_key) => {
						if next_key.as_slice() == sp_core::storage::well_known_keys::CODE {
							if_std! {
								println!("r++ Skipping code storage");
							}
							let (reads, next_key_res) = Pallet::<T>::get_next_key(&next_key);
							return (1 + reads, next_key_res);
						}
						if_std! {
							println!("r++ Next key");
						}
						(1, NextKeyResult::NextKey(next_key))
					}
					Err(_) => (1, NextKeyResult::Error("Key too long")),
				}
			} else {
				if_std! {
					println!("r++ No more keys to migrate, previous key: {}", sp_core::hexdisplay::ascii_format(&key));
				}
				(1, NextKeyResult::NoMoreKeys)
			}
		}

		/// Migrate maximum of `limit` keys starting from `start`, returns the next key to migrate
		/// Returns None if there are no more keys to migrate.
		/// Returns an error if an error occurred during migration.
		fn migrate_keys(start: StorageKey, limit: u64) -> StateMigrationResult {
			let mut key = start;
			let mut migrated = 0;
			let mut next_key_reads = 0;
			let mut writes = 0;

			while migrated < limit {
				if_std! {
					println!("Key: {}", sp_core::hexdisplay::ascii_format(&key));
				}
				let data = sp_io::storage::get(&key);
				if_std! {
					println!("r++ Reading key");
				}
				if let Some(data) = data {
					if_std! {
						println!("w++ Writing key");
					}
					sp_io::storage::set(&key, &data);
					writes += 1;
				}

				migrated += 1;

				if migrated < limit {
					let (reads, next_key_res) = Pallet::<T>::get_next_key(&key);
					next_key_reads += reads;

					match next_key_res {
						NextKeyResult::NextKey(next_key) => {
							if_std! {
								println!("Next Key of {} is {}", sp_core::hexdisplay::ascii_format(&key), sp_core::hexdisplay::ascii_format(&next_key));
							}
							key = next_key;
						}
						NextKeyResult::NoMoreKeys => {
							return StateMigrationResult {
								last_key: None,
								error: None,
								reads: migrated + next_key_reads,
								writes,
							};
						}
						NextKeyResult::Error(e) => {
							return StateMigrationResult {
								last_key: Some(key),
								error: Some(e),
								reads: migrated + next_key_reads,
								writes,
							};
						}
					};
				}
			}

			StateMigrationResult {
				last_key: Some(key),
				error: None,
				reads: migrated + next_key_reads,
				writes,
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// TODO(rodrigo): This extrinsic should be removed once the storage of destroyed contracts
		// has been removed
		#[pallet::call_index(1)]
		#[pallet::weight({
			let addresses_len = addresses.len() as u32;
			<T as crate::Config>::WeightInfo::clear_suicided_storage(addresses_len, *limit)
		})]
		pub fn clear_suicided_storage(
			origin: OriginFor<T>,
			addresses: BoundedVec<H160, GetArrayLimit>,
			limit: u32,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			ensure!(limit != 0, Error::<T>::LimitCannotBeZero);
			ensure!(
				addresses.len() != 0,
				Error::<T>::AddressesLengthCannotBeZero
			);

			let mut limit = limit as usize;

			for address in &addresses {
				// Ensure that the contract is corrupted by checking
				// that it has no code and at least one storage entry.
				let suicided = pallet_evm::Suicided::<T>::contains_key(&address);
				let has_code = pallet_evm::AccountCodes::<T>::contains_key(&address);
				ensure!(
					!suicided
						&& !has_code && pallet_evm::AccountStorages::<T>::iter_key_prefix(&address)
						.next()
						.is_some(),
					Error::<T>::ContractNotCorrupted
				);

				let deleted = pallet_evm::AccountStorages::<T>::drain_prefix(*address)
					.take(limit)
					.count();

				// Check if the storage of this contract has been completly removed
				if pallet_evm::AccountStorages::<T>::iter_key_prefix(&address)
					.next()
					.is_none()
				{
					// All entries got removed, lets count this address as migrated
					SuicidedContractsRemoved::<T>::mutate(|x| *x = x.saturating_add(1));
				}

				limit = limit.saturating_sub(deleted);
				if limit == 0 {
					return Ok(Pays::No.into());
				}
			}
			Ok(Pays::No.into())
		}
	}
}
