#![cfg_attr(
	not(test),
	deny(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![deny(clippy::unseparated_literal_suffix, unused_imports, dead_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

pub use pallet::*;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;
#[cfg(test)]
mod runtime;

#[cfg(test)]
mod tests;

mod prelude;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	pub use crate::weights::WeightInfo;
	use codec::FullCodec;
	use composable_support::abstractions::{
		nonce::Nonce,
		utils::{
			increment::{Increment, SafeIncrement},
			start_at::ZeroInit,
		},
	};
	use composable_traits::{
		assets::{
			Asset, AssetInfo, AssetInfoUpdate, AssetType, AssetTypeInspect, BiBoundedAssetName,
			BiBoundedAssetSymbol, GenerateAssetId, InspectRegistryMetadata, MutateRegistryMetadata,
		},
		currency::{AssetExistentialDepositInspect, BalanceLike, ForeignByNative},
		storage::UpdateValue,
		xcm::assets::{RemoteAssetRegistryInspect, RemoteAssetRegistryMutate},
	};
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		traits::{tokens::BalanceConversion, EnsureOrigin},
		PalletId,
	};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_runtime::traits::Convert;
	use sp_std::{borrow::ToOwned, fmt::Debug, str, vec::Vec};

	/// The module configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Factory to create new lp-token.
		type AssetId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ From<u128>
			+ Into<u128>
			+ Debug
			+ Default
			+ Ord
			+ TypeInfo
			+ MaxEncodedLen;

		/// Identifier for the class of foreign asset.
		type Location: FullCodec
			+ Eq
			+ PartialEq
			+ MaybeSerializeDeserialize
			+ Debug
			+ Clone
			+ TypeInfo
			+ MaxEncodedLen;

		type Balance: BalanceLike;

		type AssetsRegistry: RemoteAssetRegistryMutate<
				AssetId = Self::AssetId,
				AssetNativeLocation = Self::Location,
				Balance = Self::Balance,
			> + GenerateAssetId<AssetId = Self::AssetId>;

		type WeightInfo: WeightInfo;

		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Asset Nonce
	#[pallet::storage]
	#[allow(clippy::disallowed_types)] // Allow for `ValueQuery` because of nonce
	pub type AssetNonce<T: Config> =
		StorageValue<_, u64, ValueQuery, Nonce<ZeroInit, SafeIncrement>>;

	/// Asset admin
	#[pallet::storage]
	#[pallet::getter(fn asset_symbol)]
	pub type AssetAdmin<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AssetId, T::AccountId, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {
		AssetNotFound,
		NotAssetAdmin,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates an asset for bridge transfer
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::register_asset())]
		pub fn register_asset(
			origin: OriginFor<T>,
			location: T::Location,
			asset_info: AssetInfo<T::Balance>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let asset_id = <T::AssetsRegistry as GenerateAssetId>::generate_asset_id(
				T::PalletId::get().0,
				AssetNonce::<T>::increment().expect("Does not exceed u64::MAX"),
			);

			<T::AssetsRegistry as RemoteAssetRegistryMutate>::register_asset(
				asset_id,
				Some(location),
				asset_info,
			)?;
			AssetAdmin::<T>::insert(&asset_id, who);
			Ok(())
		}

		/// Update stored asset information.
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::update_asset())]
		pub fn update_asset(
			origin: OriginFor<T>,
			asset_id: T::LocalAssetId,
			asset_info: AssetInfoUpdate<T::Balance>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let asset_admin =
				AssetAdmin::<T>::try_get(asset_id).map_err(|_| Error::<T>::AssetNotFound)?;
			ensure!(asset_admin == who, Error::<T>::NotAssetAdmin);
			<Self as RemoteAssetRegistryMutate>::update_asset(asset_id, asset_info)
		}
		#[pallet::call_index(2)]
		#[pallet::weight(<T as Config>::WeightInfo::update_asset_location())]
		pub fn update_asset_location(
			origin: OriginFor<T>,
			asset_id: T::LocalAssetId,
			location: T::ForeignAssetId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let asset_admin =
				AssetAdmin::<T>::try_get(asset_id).map_err(|_| Error::<T>::AssetNotFound)?;
			ensure!(asset_admin == who, Error::<T>::NotAssetAdmin);
			<Self as RemoteAssetRegistryMutate>::set_location(asset_id, asset_info)
		}
	}
	impl<T: Config> Pallet<T> {}
}
