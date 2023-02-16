#![allow(clippy::disallowed_methods)] // allow unwrap() in tests

#[cfg(test)]
mod extrinsics;

#[cfg(test)]
mod traits;

/*
routing tests
*/
use crate::{
	mocks::{AssetId, RuntimeOrigin, Test},
	Pallet,
};
use composable_traits::{
	assets::{AssetInfo, BiBoundedAssetName, BiBoundedAssetSymbol, CreateAsset},
	xcm::assets::XcmAssetLocation,
};

pub const NATIVE_ASSET_ID: AssetId = 1;
const ACCOUNT_NATIVE: u128 = 1;
const ACCOUNT_NATIVE_BALANCE: u128 = 3000;
const ACCOUNT_LOCAL: u128 = 2;
const ACCOUNT_LOCAL_BALANCE: u128 = 1000;
pub const ACCOUNT_FOREIGN: u128 = 3;
const ACCOUNT_FOREIGN_BALANCE: u128 = 2000;
const ACCOUNT_TO: u128 = 4;

// creates for routing 1 local asset and 1 foreign asset(native asset is specified in config)
fn create_assets() -> (AssetId, AssetId) {
	let protocol_id_local = *b"testloca";
	let nonce_local = 0;
	let protocol_id_foreign = *b"testfore";
	let nonce_foreign = 0;
	let asset_info_local = AssetInfo {
		name: Some(
			BiBoundedAssetName::from_vec(b"local asset".to_vec()).expect("string is within bound"),
		),
		symbol: None,
		decimals: Some(12),
		ratio: None,
		existential_deposit: 100,
	};
	let asset_id_local =
		Pallet::<Test>::create_local_asset(protocol_id_local, nonce_local, asset_info_local)
			.unwrap();

	let foreign_asset_id = XcmAssetLocation(xcm::v2::MultiLocation::parent());
	let foreign_asset_info = AssetInfo {
		name: Some(
			BiBoundedAssetName::from_vec(b"Kusama".to_vec()).expect("string is within bound"),
		),
		symbol: Some(
			BiBoundedAssetSymbol::from_vec(b"KSM".to_vec()).expect("string is withing bound"),
		),
		decimals: Some(12),
		ratio: None,
		existential_deposit: 1000,
	};

	let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
		protocol_id_foreign,
		nonce_foreign,
		foreign_asset_info,
		foreign_asset_id,
	)
	.unwrap();
	(asset_id_local, asset_id_foreign)
}

// issue assets to different accounts and in different amount
fn mint_assets(asset_id_local: AssetId, asset_id_foreign: AssetId) {
	Pallet::<Test>::mint_into(
		RuntimeOrigin::root(),
		asset_id_local,
		ACCOUNT_LOCAL,
		ACCOUNT_LOCAL_BALANCE,
	)
	.unwrap();
	Pallet::<Test>::mint_into(
		RuntimeOrigin::root(),
		asset_id_foreign,
		ACCOUNT_FOREIGN,
		ACCOUNT_FOREIGN_BALANCE,
	)
	.unwrap();
	Pallet::<Test>::mint_into(
		RuntimeOrigin::root(),
		NATIVE_ASSET_ID,
		ACCOUNT_NATIVE,
		ACCOUNT_NATIVE_BALANCE,
	)
	.unwrap();
}

#[cfg(test)]
mod orml_route;

#[cfg(test)]
mod orml_route_asset_type;

#[cfg(test)]
mod fungibles_route;
