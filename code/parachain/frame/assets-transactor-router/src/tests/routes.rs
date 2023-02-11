#![allow(clippy::disallowed_methods)] // allow unwrap() in tests
use crate::mocks::AssetId;

use mocks::{new_test_ext, Test};

use crate::*;

const NATIVE_ASSET_ID: AssetId = 1;
const ACCOUNT_NATIVE: u128 = 1;
const ACCOUNT_LOCAL: u128 = 2;
const ACCOUNT_FOREIGN: u128 = 3;
mod route {
	use composable_traits::{
		assets::{AssetInfo, BiBoundedAssetName, BiBoundedAssetSymbol, CreateAsset},
		xcm::assets::XcmAssetLocation,
	};
	use frame_support::traits::Currency;
	use orml_traits::MultiCurrency;

	use crate::mocks::{AccountId, RuntimeOrigin};

	use super::*;

	#[test]
	fn minimmum_balance() {
		let prototcol_id_local = *b"unittest";
		let nonce_local = 0;
		let asset_info_local = AssetInfo {
			name: Some(
				BiBoundedAssetName::from_vec(b"local asset".to_vec())
					.expect("string is within bound"),
			),
			symbol: None,
			decimals: Some(12),
			ratio: None,
			existential_deposit: 100,
		};

		new_test_ext().execute_with(|| {
			let asset_id_local = Pallet::<Test>::create_local_asset(
				prototcol_id_local,
				nonce_local,
				asset_info_local,
			)
			.unwrap();

			let foreign_asset_id = XcmAssetLocation(xcm::v2::MultiLocation::parent());
			let foreign_asset_info = AssetInfo {
				name: Some(
					BiBoundedAssetName::from_vec(b"Kusama".to_vec())
						.expect("string is within bound"),
				),
				symbol: Some(
					BiBoundedAssetSymbol::from_vec(b"KSM".to_vec())
						.expect("string is withing bound"),
				),
				decimals: Some(12),
				ratio: None,
				existential_deposit: 10000,
			};

			let asset_id_foreign =
				Pallet::<Test>::create_foreign_asset(foreign_asset_id, foreign_asset_info).unwrap();

			Pallet::<Test>::mint_into(RuntimeOrigin::root(), asset_id_local, ACCOUNT_LOCAL, 1000)
				.unwrap();
			Pallet::<Test>::mint_into(
				RuntimeOrigin::root(),
				asset_id_foreign,
				ACCOUNT_FOREIGN,
				2000,
			)
			.unwrap();
			Pallet::<Test>::mint_into(RuntimeOrigin::root(), NATIVE_ASSET_ID, ACCOUNT_NATIVE, 3000)
				.unwrap();

			assert_eq!(
				<Pallet::<Test> as MultiCurrency<AccountId>>::total_balance(
					NATIVE_ASSET_ID,
					&ACCOUNT_NATIVE
				),
				3000
			);
			assert_eq!(
				<Pallet::<Test> as MultiCurrency<AccountId>>::total_balance(
					asset_id_local,
					&ACCOUNT_LOCAL
				),
				1000
			);
			assert_eq!(
				<Pallet::<Test> as MultiCurrency<AccountId>>::total_balance(
					asset_id_foreign,
					&ACCOUNT_FOREIGN
				),
				2000
			);
			assert_eq!(<<Test as Config>::NativeTransactor>::total_balance(&ACCOUNT_NATIVE), 3000);
			assert_eq!(
				<<Test as Config>::LocalTransactor>::total_balance(asset_id_local, &ACCOUNT_LOCAL),
				1000
			);
			assert_eq!(
				<<Test as Config>::ForeignTransactor>::total_balance(
					asset_id_foreign,
					&ACCOUNT_FOREIGN
				),
				2000
			);
		});
	}
}
