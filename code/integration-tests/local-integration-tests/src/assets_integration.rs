///! tests that various assets integration scenarios work well
use crate::{helpers::*, kusama_test_net::This, prelude::*};
use composable_traits::{
	assets::AssetInfo,
	// storage::UpdateValue,
	xcm::assets::XcmAssetLocation,
};

use frame_system::RawOrigin;
// use primitives::currency::*;
use xcm_emulator::TestExt;

#[test]
fn updated_assets_registry_works_well_for_ratios() {
	simtest();
	// This::execute_with(|| {
	// use this_runtime::*;
	// assert_ok!(AssetsRegistry::update_asset(
	// 	RawOrigin::Root.into(),
	// 	CurrencyId(42),
	// 	AssetInfoUpdate {
	// 		name: UpdateValue::Ignore,
	// 		symbol: UpdateValue::Ignore,
	// 		decimals: UpdateValue::Ignore,
	// 		existential_deposit: UpdateValue::Ignore,
	// 		ratio: UpdateValue::Set(Some(Rational64::from(10, 1))),
	// 	},
	// ));
	// assert_ok!(AssetsRegistry::set_reserve_location(
	// 	CurrencyId(42),
	// 	XcmAssetLocation(MultiLocation::new(1, X1(Parachain(666)))),
	// ));
	// AssetsRegistry::update_asset(
	// 	RawOrigin::Root.into(),
	// 	CurrencyId(123),
	// 	AssetInfoUpdate {
	// 		name: UpdateValue::Ignore,
	// 		symbol: UpdateValue::Ignore,
	// 		decimals: UpdateValue::Ignore,
	// 		existential_deposit: UpdateValue::Ignore,
	// 		ratio: UpdateValue::Set(Some(Rational64::from(10, 100))),
	// 	},
	// )
	// .unwrap();
	// AssetsRegistry::set_reserve_location(
	// 	CurrencyId(123),
	// 	XcmAssetLocation(MultiLocation::new(1, X1(Parachain(4321)))),
	// )
	// .unwrap();
	// assert_eq!(
	// 	1000,
	// 	<PriceConverter<AssetsRegistry,
	// this_runtime::WellKnownForeignToNativePriceConverter>>::to_asset_balance(100,
	// CurrencyId(42)).unwrap() );
	// assert_eq!(
	// 	10,
	// 	<PriceConverter<AssetsRegistry,
	// this_runtime::WellKnownForeignToNativePriceConverter>>::to_asset_balance(100,
	// CurrencyId(123)).unwrap() );
	// });
}

#[test]
fn registered_assets_with_smaller_than_native_price() {
	simtest();
	This::execute_with(|| {
		use this_runtime::*;
		AssetsRegistry::register_asset(
			RawOrigin::Root.into(),
			XcmAssetLocation(MultiLocation::new(1, X1(Parachain(666)))),
			AssetInfo {
				name: None,
				symbol: None,
				decimals: None,
				existential_deposit: 0,
				ratio: Some(Rational64::from(10, 1)),
			},
		)
		.unwrap();
		let asset_id = System::events()
			.iter()
			.find_map(|x| match x.event {
				RuntimeEvent::AssetsRegistry(
					assets_registry::Event::<Runtime>::AssetRegistered {
						asset_id,
						location: _,
						asset_info: _,
					},
				) => Some(asset_id),
				_ => None,
			})
			.unwrap();
		assert_eq!(
			1000,
			<PriceConverter<AssetsRegistry, this_runtime::WellKnownForeignToNativePriceConverter>>::to_asset_balance(100, asset_id).unwrap()
		);
	});
}

#[test]
fn registered_assets_with_larger_than_native_price() {
	simtest();
	This::execute_with(|| {
		use this_runtime::*;
		AssetsRegistry::register_asset(
			RawOrigin::Root.into(),
			XcmAssetLocation(MultiLocation::new(1, X1(Parachain(666)))),
			AssetInfo {
				name: None,
				symbol: None,
				decimals: None,
				existential_deposit: 0,
				ratio: Some(Rational64::from(10, 100)),
			},
		)
		.unwrap();
		let asset_id = System::events()
			.iter()
			.find_map(|x| match x.event {
				RuntimeEvent::AssetsRegistry(
					assets_registry::Event::<Runtime>::AssetRegistered {
						asset_id,
						location: _,
						asset_info: _,
					},
				) => Some(asset_id),
				_ => None,
			})
			.unwrap();
		assert_eq!(10, <PriceConverter<AssetsRegistry,this_runtime::WellKnownForeignToNativePriceConverter>>::to_asset_balance(100, asset_id).unwrap());
	});
}
