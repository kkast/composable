//! Autogenerated weights for farming
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-17, STEPS: `100`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/interbtc-standalone
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// farming
// --extrinsic
// *
// --steps
// 100
// --repeat
// 10
// --output
// crates/farming/src/default_weights.rs
// --template
// .deploy/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for farming.
pub trait WeightInfo {
	fn on_initialize(c: u32, ) -> Weight;
	fn update_reward_schedule() -> Weight;
	fn remove_reward_schedule() -> Weight;
	fn deposit() -> Weight;
	fn withdraw() -> Weight;
	fn claim() -> Weight;
}

/// Weights for farming using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Farming RewardSchedules (r:1 w:0)
	// Storage: FarmingRewards TotalStake (r:1 w:0)
	fn on_initialize(c: u32, ) -> Weight {
		Weight::from_parts(18_073_005u64, 0)
			// Standard Error: 183_362
			.saturating_add(Weight::from_parts(18_555_611u64, 0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1u64))
			.saturating_add(T::DbWeight::get().reads((2u64).saturating_mul(c as u64)))
	}
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:2 w:1)
	// Storage: Farming RewardSchedules (r:1 w:1)
	fn update_reward_schedule() -> Weight {
		Weight::from_parts(105_531_000u64, 0)
			.saturating_add(T::DbWeight::get().reads(5u64))
			.saturating_add(T::DbWeight::get().writes(4u64))
	}
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	// Storage: Farming RewardSchedules (r:0 w:1)
	fn remove_reward_schedule() -> Weight {
		Weight::from_parts(83_988_000u64, 0)
			.saturating_add(T::DbWeight::get().reads(3u64))
			.saturating_add(T::DbWeight::get().writes(3u64))
	}
	// Storage: Farming RewardSchedules (r:2 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: FarmingRewards Stake (r:1 w:1)
	// Storage: FarmingRewards TotalStake (r:1 w:1)
	// Storage: FarmingRewards RewardTally (r:2 w:2)
	// Storage: FarmingRewards RewardPerToken (r:2 w:0)
	fn deposit() -> Weight {
		Weight::from_parts(108_507_000u64, 0)
			.saturating_add(T::DbWeight::get().reads(9u64))
			.saturating_add(T::DbWeight::get().writes(5u64))
	}
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: FarmingRewards Stake (r:1 w:1)
	// Storage: FarmingRewards TotalStake (r:1 w:1)
	// Storage: FarmingRewards RewardTally (r:2 w:2)
	// Storage: FarmingRewards RewardPerToken (r:2 w:0)
	fn withdraw() -> Weight {
		Weight::from_parts(96_703_000u64, 0)
			.saturating_add(T::DbWeight::get().reads(7u64))
			.saturating_add(T::DbWeight::get().writes(5u64))
	}
	// Storage: FarmingRewards Stake (r:1 w:0)
	// Storage: FarmingRewards RewardPerToken (r:1 w:0)
	// Storage: FarmingRewards RewardTally (r:1 w:1)
	// Storage: FarmingRewards TotalRewards (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:2 w:1)
	fn claim() -> Weight {
		Weight::from_parts(136_142_000u64, 0)
			.saturating_add(T::DbWeight::get().reads(8u64))
			.saturating_add(T::DbWeight::get().writes(5u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Farming RewardSchedules (r:1 w:0)
	// Storage: FarmingRewards TotalStake (r:1 w:0)
	fn on_initialize(c: u32, ) -> Weight {
		Weight::from_parts(18_073_005u64, 0)
			// Standard Error: 183_362
			.saturating_add(Weight::from_parts(18_555_611u64, 0).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(1u64))
			.saturating_add(RocksDbWeight::get().reads((2u64).saturating_mul(c as u64)))
	}
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:2 w:1)
	// Storage: Farming RewardSchedules (r:1 w:1)
	fn update_reward_schedule() -> Weight {
		Weight::from_parts(105_531_000u64, 0)
			.saturating_add(RocksDbWeight::get().reads(5u64))
			.saturating_add(RocksDbWeight::get().writes(4u64))
	}
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	// Storage: Farming RewardSchedules (r:0 w:1)
	fn remove_reward_schedule() -> Weight {
		Weight::from_parts(83_988_000u64, 0)
			.saturating_add(RocksDbWeight::get().reads(3u64))
			.saturating_add(RocksDbWeight::get().writes(3u64))
	}
	// Storage: Farming RewardSchedules (r:2 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: FarmingRewards Stake (r:1 w:1)
	// Storage: FarmingRewards TotalStake (r:1 w:1)
	// Storage: FarmingRewards RewardTally (r:2 w:2)
	// Storage: FarmingRewards RewardPerToken (r:2 w:0)
	fn deposit() -> Weight {
		Weight::from_parts(108_507_000u64, 0)
			.saturating_add(RocksDbWeight::get().reads(9u64))
			.saturating_add(RocksDbWeight::get().writes(5u64))
	}
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: FarmingRewards Stake (r:1 w:1)
	// Storage: FarmingRewards TotalStake (r:1 w:1)
	// Storage: FarmingRewards RewardTally (r:2 w:2)
	// Storage: FarmingRewards RewardPerToken (r:2 w:0)
	fn withdraw() -> Weight {
		Weight::from_parts(96_703_000u64, 0)
			.saturating_add(RocksDbWeight::get().reads(7u64))
			.saturating_add(RocksDbWeight::get().writes(5u64))
	}
	// Storage: FarmingRewards Stake (r:1 w:0)
	// Storage: FarmingRewards RewardPerToken (r:1 w:0)
	// Storage: FarmingRewards RewardTally (r:1 w:1)
	// Storage: FarmingRewards TotalRewards (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:2 w:1)
	fn claim() -> Weight {
		Weight::from_parts(136_142_000u64, 0)
			.saturating_add(RocksDbWeight::get().reads(8u64))
			.saturating_add(RocksDbWeight::get().writes(5u64))
	}
}
