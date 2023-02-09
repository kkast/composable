
//! Autogenerated weights for `scheduler`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-08, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `5a3b4d6e5166`, CPU: `Intel(R) Xeon(R) CPU @ 3.10GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/9gdd70pyc12n9i1v6gx99rhz8q2n67z0-composable/bin/composable
// benchmark
// pallet
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=10
// --output=code/parachain/runtime/picasso/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `scheduler`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> scheduler::WeightInfo for WeightInfo<T> {
	// Storage: Scheduler IncompleteSince (r:1 w:1)
	fn service_agendas_base() -> Weight {
		// Minimum execution time: 8_655 nanoseconds.
		Weight::from_ref_time(9_044_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[0, 50]`.
	fn service_agenda_base(s: u32, ) -> Weight {
		// Minimum execution time: 7_578 nanoseconds.
		Weight::from_ref_time(13_321_492 as u64)
			// Standard Error: 5_653
			.saturating_add(Weight::from_ref_time(1_253_413 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn service_task_base() -> Weight {
		// Minimum execution time: 18_408 nanoseconds.
		Weight::from_ref_time(19_175_000 as u64)
	}
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	/// The range of component `s` is `[128, 4194304]`.
	fn service_task_fetched(s: u32, ) -> Weight {
		// Minimum execution time: 41_717 nanoseconds.
		Weight::from_ref_time(42_009_000 as u64)
			// Standard Error: 5
			.saturating_add(Weight::from_ref_time(1_602 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Scheduler Lookup (r:0 w:1)
	fn service_task_named() -> Weight {
		// Minimum execution time: 21_644 nanoseconds.
		Weight::from_ref_time(22_364_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn service_task_periodic() -> Weight {
		// Minimum execution time: 18_972 nanoseconds.
		Weight::from_ref_time(19_941_000 as u64)
	}
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	fn execute_dispatch_signed() -> Weight {
		// Minimum execution time: 13_998 nanoseconds.
		Weight::from_ref_time(14_335_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	fn execute_dispatch_unsigned() -> Weight {
		// Minimum execution time: 8_582 nanoseconds.
		Weight::from_ref_time(8_751_000 as u64)
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[0, 49]`.
	fn schedule(s: u32, ) -> Weight {
		// Minimum execution time: 34_389 nanoseconds.
		Weight::from_ref_time(41_697_360 as u64)
			// Standard Error: 10_005
			.saturating_add(Weight::from_ref_time(1_255_542 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn cancel(s: u32, ) -> Weight {
		// Minimum execution time: 36_581 nanoseconds.
		Weight::from_ref_time(39_548_222 as u64)
			// Standard Error: 7_853
			.saturating_add(Weight::from_ref_time(1_278_112 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[0, 49]`.
	fn schedule_named(s: u32, ) -> Weight {
		// Minimum execution time: 39_054 nanoseconds.
		Weight::from_ref_time(47_183_538 as u64)
			// Standard Error: 9_809
			.saturating_add(Weight::from_ref_time(1_323_313 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn cancel_named(s: u32, ) -> Weight {
		// Minimum execution time: 39_397 nanoseconds.
		Weight::from_ref_time(43_118_094 as u64)
			// Standard Error: 10_395
			.saturating_add(Weight::from_ref_time(1_305_126 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
