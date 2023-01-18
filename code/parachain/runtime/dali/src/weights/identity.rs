
//! Autogenerated weights for `identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-16, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `fde3d2d43403`, CPU: `Intel(R) Xeon(R) CPU @ 2.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/y1z2mfgy9msqas77hhxszf78hqg6mx5y-composable/bin/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
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

/// Weight functions for `identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> identity::WeightInfo for WeightInfo<T> {
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 7]`.
	fn add_registrar(r: u32, ) -> Weight {
		Weight::from_ref_time(43_709_000_u64)
			// Standard Error: 289_000
			.saturating_add(Weight::from_ref_time(755_000_u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 8]`.
	/// The range of component `x` is `[1, 32]`.
	fn set_identity(r: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(86_108_000_u64)
			// Standard Error: 396_000
			.saturating_add(Weight::from_ref_time(491_000_u64).saturating_mul(r as u64))
			// Standard Error: 67_000
			.saturating_add(Weight::from_ref_time(1_147_000_u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:1 w:1)
	/// The range of component `s` is `[1, 32]`.
	fn set_subs_new(s: u32, ) -> Weight {
		Weight::from_ref_time(71_034_000_u64)
			// Standard Error: 48_000
			.saturating_add(Weight::from_ref_time(8_197_000_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s as u64)))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:1)
	/// The range of component `p` is `[1, 32]`.
	fn set_subs_old(p: u32, ) -> Weight {
		Weight::from_ref_time(75_813_000_u64)
			// Standard Error: 63_000
			.saturating_add(Weight::from_ref_time(2_857_000_u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p as u64)))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:32)
	/// The range of component `r` is `[1, 8]`.
	/// The range of component `s` is `[1, 32]`.
	/// The range of component `x` is `[1, 32]`.
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(91_020_000_u64)
			// Standard Error: 405_000
			.saturating_add(Weight::from_ref_time(638_000_u64).saturating_mul(r as u64))
			// Standard Error: 61_000
			.saturating_add(Weight::from_ref_time(2_894_000_u64).saturating_mul(s as u64))
			// Standard Error: 61_000
			.saturating_add(Weight::from_ref_time(496_000_u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s as u64)))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 8]`.
	/// The range of component `x` is `[1, 32]`.
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(79_938_000_u64)
			// Standard Error: 422_000
			.saturating_add(Weight::from_ref_time(2_075_000_u64).saturating_mul(r as u64))
			// Standard Error: 71_000
			.saturating_add(Weight::from_ref_time(1_187_000_u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 8]`.
	/// The range of component `x` is `[1, 32]`.
	fn cancel_request(_r: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(95_051_000_u64)
			// Standard Error: 55_000
			.saturating_add(Weight::from_ref_time(1_006_000_u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 7]`.
	fn set_fee(r: u32, ) -> Weight {
		Weight::from_ref_time(22_284_000_u64)
			// Standard Error: 306_000
			.saturating_add(Weight::from_ref_time(876_000_u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 7]`.
	fn set_account_id(r: u32, ) -> Weight {
		Weight::from_ref_time(22_998_000_u64)
			// Standard Error: 163_000
			.saturating_add(Weight::from_ref_time(767_000_u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 7]`.
	fn set_fields(r: u32, ) -> Weight {
		Weight::from_ref_time(23_994_000_u64)
			// Standard Error: 236_000
			.saturating_add(Weight::from_ref_time(528_000_u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 7]`.
	/// The range of component `x` is `[1, 32]`.
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(60_069_000_u64)
			// Standard Error: 370_000
			.saturating_add(Weight::from_ref_time(651_000_u64).saturating_mul(r as u64))
			// Standard Error: 51_000
			.saturating_add(Weight::from_ref_time(1_244_000_u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Identity SuperOf (r:0 w:32)
	/// The range of component `r` is `[1, 8]`.
	/// The range of component `s` is `[1, 32]`.
	/// The range of component `x` is `[1, 32]`.
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(113_694_000_u64)
			// Standard Error: 400_000
			.saturating_add(Weight::from_ref_time(702_000_u64).saturating_mul(r as u64))
			// Standard Error: 61_000
			.saturating_add(Weight::from_ref_time(3_005_000_u64).saturating_mul(s as u64))
			// Standard Error: 61_000
			.saturating_add(Weight::from_ref_time(134_000_u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s as u64)))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[1, 31]`.
	fn add_sub(s: u32, ) -> Weight {
		Weight::from_ref_time(99_260_000_u64)
			// Standard Error: 64_000
			.saturating_add(Weight::from_ref_time(206_000_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	/// The range of component `s` is `[1, 32]`.
	fn rename_sub(s: u32, ) -> Weight {
		Weight::from_ref_time(36_583_000_u64)
			// Standard Error: 31_000
			.saturating_add(Weight::from_ref_time(181_000_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[1, 32]`.
	fn remove_sub(s: u32, ) -> Weight {
		Weight::from_ref_time(96_454_000_u64)
			// Standard Error: 51_000
			.saturating_add(Weight::from_ref_time(347_000_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[1, 31]`.
	fn quit_sub(s: u32, ) -> Weight {
		Weight::from_ref_time(66_854_000_u64)
			// Standard Error: 41_000
			.saturating_add(Weight::from_ref_time(390_000_u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}
