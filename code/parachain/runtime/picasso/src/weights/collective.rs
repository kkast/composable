
//! Autogenerated weights for `collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-01, STEPS: `50`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `3a6013dfb40d`, CPU: `Intel(R) Xeon(R) CPU @ 3.10GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/jif3kmz9kgiwz8hg8nzb9d2kiga1rnga-composable/bin/composable
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

/// Weight functions for `collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> collective::WeightInfo for WeightInfo<T> {
	/// Storage: Council Members (r:1 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:100 w:100)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (3233 ±0) + p * (3223 ±0)`
		//  Estimated: `16054 + m * (7806 ±32) + p * (10238 ±32)`
		// Minimum execution time: 30_392 nanoseconds.
		Weight::from_parts(30_512_000, 0)
			.saturating_add(Weight::from_parts(0, 16054))
			// Standard Error: 137_779
			.saturating_add(Weight::from_parts(8_517_426, 0).saturating_mul(m.into()))
			// Standard Error: 137_779
			.saturating_add(Weight::from_parts(13_319_061, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 7806).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 10238).saturating_mul(p.into()))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// Proof: CallFilter DisabledCalls (max_values: None, max_size: Some(212), added: 2687, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `177 + m * (32 ±0)`
		//  Estimated: `3360 + m * (32 ±0)`
		// Minimum execution time: 35_362 nanoseconds.
		Weight::from_parts(35_237_771, 0)
			.saturating_add(Weight::from_parts(0, 3360))
			// Standard Error: 216
			.saturating_add(Weight::from_parts(1_705, 0).saturating_mul(b.into()))
			// Standard Error: 2_231
			.saturating_add(Weight::from_parts(42_621, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:1 w:0)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// Proof: CallFilter DisabledCalls (max_values: None, max_size: Some(212), added: 2687, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `177 + m * (32 ±0)`
		//  Estimated: `6013 + m * (64 ±0)`
		// Minimum execution time: 38_700 nanoseconds.
		Weight::from_parts(37_686_715, 0)
			.saturating_add(Weight::from_parts(0, 6013))
			// Standard Error: 230
			.saturating_add(Weight::from_parts(2_149, 0).saturating_mul(b.into()))
			// Standard Error: 2_374
			.saturating_add(Weight::from_parts(83_432, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:1 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalCount (r:1 w:1)
	/// Proof Skipped: Council ProposalCount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:0 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `423 + m * (32 ±0) + p * (36 ±0)`
		//  Estimated: `5680 + m * (165 ±0) + p * (180 ±0)`
		// Minimum execution time: 46_233 nanoseconds.
		Weight::from_parts(38_669_945, 0)
			.saturating_add(Weight::from_parts(0, 5680))
			// Standard Error: 553
			.saturating_add(Weight::from_parts(8_278, 0).saturating_mul(b.into()))
			// Standard Error: 5_780
			.saturating_add(Weight::from_parts(65_371, 0).saturating_mul(m.into()))
			// Standard Error: 5_707
			.saturating_add(Weight::from_parts(512_154, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 165).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 180).saturating_mul(p.into()))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `873 + m * (64 ±0)`
		//  Estimated: `4714 + m * (128 ±0)`
		// Minimum execution time: 47_714 nanoseconds.
		Weight::from_parts(50_236_639, 0)
			.saturating_add(Weight::from_parts(0, 4714))
			// Standard Error: 7_604
			.saturating_add(Weight::from_parts(175_738, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 128).saturating_mul(m.into()))
	}
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:0 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `493 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `5353 + m * (260 ±0) + p * (144 ±0)`
		// Minimum execution time: 52_813 nanoseconds.
		Weight::from_parts(44_312_363, 0)
			.saturating_add(Weight::from_parts(0, 5353))
			// Standard Error: 6_911
			.saturating_add(Weight::from_parts(118_429, 0).saturating_mul(m.into()))
			// Standard Error: 6_739
			.saturating_add(Weight::from_parts(470_986, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 260).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 144).saturating_mul(p.into()))
	}
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:1 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// Proof: CallFilter DisabledCalls (max_values: None, max_size: Some(212), added: 2687, mode: MaxEncodedLen)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `905 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `11611 + b * (4 ±0) + m * (264 ±0) + p * (160 ±0)`
		// Minimum execution time: 81_984 nanoseconds.
		Weight::from_parts(76_111_513, 0)
			.saturating_add(Weight::from_parts(0, 11611))
			// Standard Error: 946
			.saturating_add(Weight::from_parts(7_057, 0).saturating_mul(b.into()))
			// Standard Error: 10_003
			.saturating_add(Weight::from_parts(37_864, 0).saturating_mul(m.into()))
			// Standard Error: 9_751
			.saturating_add(Weight::from_parts(641_837, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 4).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 264).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 160).saturating_mul(p.into()))
	}
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:0)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:0 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `513 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `6420 + m * (325 ±0) + p * (180 ±0)`
		// Minimum execution time: 55_945 nanoseconds.
		Weight::from_parts(47_555_230, 0)
			.saturating_add(Weight::from_parts(0, 6420))
			// Standard Error: 6_055
			.saturating_add(Weight::from_parts(140_771, 0).saturating_mul(m.into()))
			// Standard Error: 5_904
			.saturating_add(Weight::from_parts(454_163, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 325).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 180).saturating_mul(p.into()))
	}
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:0)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:1 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// Proof: CallFilter DisabledCalls (max_values: None, max_size: Some(212), added: 2687, mode: MaxEncodedLen)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `925 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `12952 + b * (5 ±0) + m * (330 ±0) + p * (200 ±0)`
		// Minimum execution time: 87_447 nanoseconds.
		Weight::from_parts(67_202_564, 0)
			.saturating_add(Weight::from_parts(0, 12952))
			// Standard Error: 1_041
			.saturating_add(Weight::from_parts(13_407, 0).saturating_mul(b.into()))
			// Standard Error: 11_007
			.saturating_add(Weight::from_parts(159_500, 0).saturating_mul(m.into()))
			// Standard Error: 10_729
			.saturating_add(Weight::from_parts(624_600, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 5).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 330).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 200).saturating_mul(p.into()))
	}
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:0 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:0 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `258 + p * (32 ±0)`
		//  Estimated: `1269 + p * (96 ±0)`
		// Minimum execution time: 27_088 nanoseconds.
		Weight::from_parts(30_463_705, 0)
			.saturating_add(Weight::from_parts(0, 1269))
			// Standard Error: 5_203
			.saturating_add(Weight::from_parts(427_241, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 96).saturating_mul(p.into()))
	}
}
