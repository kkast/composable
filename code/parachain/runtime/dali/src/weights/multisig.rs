
//! Autogenerated weights for `multisig`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-08, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `05551ac21fb8`, CPU: `Intel(R) Xeon(R) CPU @ 3.10GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/9gdd70pyc12n9i1v6gx99rhz8q2n67z0-composable/bin/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=10
// --output=code/parachain/runtime/dali/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `multisig`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> multisig::WeightInfo for WeightInfo<T> {
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Minimum execution time: 44_452 nanoseconds.
		Weight::from_ref_time(50_295_522 as u64)
			// Standard Error: 29
			.saturating_add(Weight::from_ref_time(658 as u64).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 84_303 nanoseconds.
		Weight::from_ref_time(67_814_692 as u64)
			// Standard Error: 4_439
			.saturating_add(Weight::from_ref_time(290_740 as u64).saturating_mul(s as u64))
			// Standard Error: 43
			.saturating_add(Weight::from_ref_time(2_093 as u64).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	/// The range of component `s` is `[3, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 66_655 nanoseconds.
		Weight::from_ref_time(47_998_550 as u64)
			// Standard Error: 3_541
			.saturating_add(Weight::from_ref_time(286_578 as u64).saturating_mul(s as u64))
			// Standard Error: 34
			.saturating_add(Weight::from_ref_time(2_155 as u64).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 96_372 nanoseconds.
		Weight::from_ref_time(79_046_215 as u64)
			// Standard Error: 4_339
			.saturating_add(Weight::from_ref_time(346_920 as u64).saturating_mul(s as u64))
			// Standard Error: 42
			.saturating_add(Weight::from_ref_time(2_300 as u64).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Minimum execution time: 61_746 nanoseconds.
		Weight::from_ref_time(66_918_491 as u64)
			// Standard Error: 3_828
			.saturating_add(Weight::from_ref_time(277_971 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Minimum execution time: 44_078 nanoseconds.
		Weight::from_ref_time(47_525_113 as u64)
			// Standard Error: 3_521
			.saturating_add(Weight::from_ref_time(280_878 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	/// The range of component `s` is `[2, 100]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Minimum execution time: 63_256 nanoseconds.
		Weight::from_ref_time(66_268_622 as u64)
			// Standard Error: 4_175
			.saturating_add(Weight::from_ref_time(283_383 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
