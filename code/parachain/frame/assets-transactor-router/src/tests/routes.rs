#![allow(clippy::disallowed_methods)] // allow unwrap() in tests
use crate::mocks::AssetId;

use mocks::{new_test_ext, Test};

use crate::*;

const NATIVE_ASSET_ID: AssetId = 1;
const ACCOUNT_NATIVE: u128 = 1;
const ACCOUNT_LOCAL: u128 = 2;
const ACCOUNT_FOREIGN: u128 = 3;
const ACCOUNT_TO: u128 = 4;
mod orml_route {
	use composable_traits::{
		assets::{AssetInfo, BiBoundedAssetName, BiBoundedAssetSymbol, CreateAsset},
		xcm::assets::XcmAssetLocation,
	};
	use frame_support::{
		assert_ok,
		traits::{Currency, ReservableCurrency},
	};
	use orml_traits::{MultiCurrency, MultiReservableCurrency};

	use crate::mocks::{AccountId, RuntimeOrigin};

	use super::*;

	#[test]
	fn minimum_balance() {
		let protocol_id_local = *b"testloca";
		let nonce_local = 0;
		let protocol_id_foreign = *b"testfore";
		let nonce_foreign = 0;
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
				protocol_id_local,
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
				existential_deposit: 1000,
			};

			let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
				protocol_id_foreign,
				nonce_foreign,
				foreign_asset_info,
				foreign_asset_id,
			)
			.unwrap();

			assert_eq!(
				<Pallet::<Test> as MultiCurrency<AccountId>>::minimum_balance(NATIVE_ASSET_ID),
				1
			);
			// should be 0 because of ExistentialDeposits for orml_tokens
			assert_eq!(
				<Pallet::<Test> as MultiCurrency<AccountId>>::minimum_balance(asset_id_local),
				0
			);
			// should be 0 because of ExistentialDeposits for orml_tokens
			assert_eq!(
				<Pallet::<Test> as MultiCurrency<AccountId>>::minimum_balance(asset_id_foreign),
				0
			);

			assert_eq!(<<Test as Config>::NativeTransactor>::minimum_balance(), 1);
			assert_eq!(<<Test as Config>::LocalTransactor>::minimum_balance(asset_id_local), 0);
			assert_eq!(<<Test as Config>::ForeignTransactor>::minimum_balance(asset_id_foreign), 0);
		});
	}

	#[test]
	fn total_issuance() {
		let protocol_id_local = *b"testloca";
		let nonce_local = 0;
		let protocol_id_foreign = *b"testfore";
		let nonce_foreign = 0;
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
				protocol_id_local,
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
				existential_deposit: 1000,
			};

			let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
				protocol_id_foreign,
				nonce_foreign,
				foreign_asset_info,
				foreign_asset_id,
			)
			.unwrap();

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
				<Pallet::<Test> as MultiCurrency<AccountId>>::total_issuance(NATIVE_ASSET_ID),
				3000
			);
			assert_eq!(
				<Pallet::<Test> as MultiCurrency<AccountId>>::total_issuance(asset_id_local),
				1000
			);
			assert_eq!(
				<Pallet::<Test> as MultiCurrency<AccountId>>::total_issuance(asset_id_foreign),
				2000
			);

			assert_eq!(<<Test as Config>::NativeTransactor>::total_issuance(), 3000);
			assert_eq!(<<Test as Config>::LocalTransactor>::total_issuance(asset_id_local), 1000);
			assert_eq!(
				<<Test as Config>::ForeignTransactor>::total_issuance(asset_id_foreign),
				2000
			);
		});
	}
	#[test]
	fn total_balance() {
		let protocol_id_local = *b"testloca";
		let nonce_local = 0;
		let protocol_id_foreign = *b"testfore";
		let nonce_foreign = 0;
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
				protocol_id_local,
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
				existential_deposit: 1000,
			};

			let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
				protocol_id_foreign,
				nonce_foreign,
				foreign_asset_info,
				foreign_asset_id,
			)
			.unwrap();

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
	#[test]
	fn free_balance() {
		let protocol_id_local = *b"testloca";
		let nonce_local = 0;
		let protocol_id_foreign = *b"testfore";
		let nonce_foreign = 0;
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
				protocol_id_local,
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
				existential_deposit: 1000,
			};

			let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
				protocol_id_foreign,
				nonce_foreign,
				foreign_asset_info,
				foreign_asset_id,
			)
			.unwrap();

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
				<Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(
					NATIVE_ASSET_ID,
					&ACCOUNT_NATIVE
				),
				3000
			);
			assert_eq!(
				<Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(
					asset_id_local,
					&ACCOUNT_LOCAL
				),
				1000
			);
			assert_eq!(
				<Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(
					asset_id_foreign,
					&ACCOUNT_FOREIGN
				),
				2000
			);

			assert_eq!(<<Test as Config>::NativeTransactor>::free_balance(&ACCOUNT_NATIVE), 3000);
			assert_eq!(
				<<Test as Config>::LocalTransactor>::free_balance(asset_id_local, &ACCOUNT_LOCAL),
				1000
			);
			assert_eq!(
				<<Test as Config>::ForeignTransactor>::free_balance(
					asset_id_foreign,
					&ACCOUNT_FOREIGN
				),
				2000
			);
		});
	}

	#[test]
	fn can_slash() {
		let protocol_id_local = *b"testloca";
		let nonce_local = 0;
		let protocol_id_foreign = *b"testfore";
		let nonce_foreign = 0;
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
				protocol_id_local,
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
				existential_deposit: 1000,
			};

			let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
				protocol_id_foreign,
				nonce_foreign,
				foreign_asset_info,
				foreign_asset_id,
			)
			.unwrap();

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

			assert!(<Pallet::<Test> as MultiCurrency<AccountId>>::can_slash(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				3000
			),);
			assert!(<Pallet::<Test> as MultiCurrency<AccountId>>::can_slash(
				asset_id_local,
				&ACCOUNT_LOCAL,
				1000
			),);
			assert!(<Pallet::<Test> as MultiCurrency<AccountId>>::can_slash(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				2000
			),);

			assert!(<<Test as Config>::NativeTransactor>::can_slash(&ACCOUNT_NATIVE, 3000));
			assert!(<<Test as Config>::LocalTransactor>::can_slash(
				asset_id_local,
				&ACCOUNT_LOCAL,
				1000
			),);
			assert!(<<Test as Config>::ForeignTransactor>::can_slash(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				2000
			),);
		});
	}

	#[test]
	fn can_reserve() {
		let protocol_id_local = *b"testloca";
		let nonce_local = 0;
		let protocol_id_foreign = *b"testfore";
		let nonce_foreign = 0;
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
				protocol_id_local,
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
				existential_deposit: 1000,
			};

			let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
				protocol_id_foreign,
				nonce_foreign,
				foreign_asset_info,
				foreign_asset_id,
			)
			.unwrap();

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

			assert!(<Pallet::<Test> as MultiReservableCurrency<AccountId>>::can_reserve(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				3000
			),);
			assert!(<Pallet::<Test> as MultiReservableCurrency<AccountId>>::can_reserve(
				asset_id_local,
				&ACCOUNT_LOCAL,
				1000
			),);
			assert!(<Pallet::<Test> as MultiReservableCurrency<AccountId>>::can_reserve(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				2000
			),);

			assert!(<<Test as Config>::NativeTransactor>::can_reserve(&ACCOUNT_NATIVE, 3000));
			assert!(<<Test as Config>::LocalTransactor>::can_reserve(
				asset_id_local,
				&ACCOUNT_LOCAL,
				1000
			));
			assert!(<<Test as Config>::ForeignTransactor>::can_reserve(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				2000
			));
		});
	}

	#[test]
	fn reserve() {
		let protocol_id_local = *b"testloca";
		let nonce_local = 0;
		let protocol_id_foreign = *b"testfore";
		let nonce_foreign = 0;
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
				protocol_id_local,
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
				existential_deposit: 1000,
			};

			let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
				protocol_id_foreign,
				nonce_foreign,
				foreign_asset_info,
				foreign_asset_id,
			)
			.unwrap();

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

			assert_ok!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::reserve(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				2500,
			));
			assert_ok!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::reserve(
				asset_id_local,
				&ACCOUNT_LOCAL,
				900,
			));
			assert_ok!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::reserve(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				2000,
			));
			assert_eq!(
				<Pallet::<Test> as MultiReservableCurrency<AccountId>>::reserved_balance(
					NATIVE_ASSET_ID,
					&ACCOUNT_NATIVE
				),
				2500
			);
			assert_eq!(
				<Pallet::<Test> as MultiReservableCurrency<AccountId>>::reserved_balance(
					asset_id_local,
					&ACCOUNT_LOCAL
				),
				900
			);
			assert_eq!(
				<Pallet::<Test> as MultiReservableCurrency<AccountId>>::reserved_balance(
					asset_id_foreign,
					&ACCOUNT_FOREIGN
				),
				2000
			);

			assert_eq!(
				<<Test as Config>::NativeTransactor>::reserved_balance(&ACCOUNT_NATIVE),
				2500
			);
			assert_eq!(
				<<Test as Config>::LocalTransactor>::reserved_balance(
					asset_id_local,
					&ACCOUNT_LOCAL
				),
				900
			);
			assert_eq!(
				<<Test as Config>::ForeignTransactor>::reserved_balance(
					asset_id_foreign,
					&ACCOUNT_FOREIGN
				),
				2000
			);
			<Pallet<Test> as MultiReservableCurrency<AccountId>>::unreserve(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				1000,
			);
			<Pallet<Test> as MultiReservableCurrency<AccountId>>::unreserve(
				asset_id_local,
				&ACCOUNT_LOCAL,
				700,
			);
			<Pallet<Test> as MultiReservableCurrency<AccountId>>::unreserve(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				1500,
			);
			assert_eq!(
				<Pallet::<Test> as MultiReservableCurrency<AccountId>>::reserved_balance(
					NATIVE_ASSET_ID,
					&ACCOUNT_NATIVE
				),
				1500
			);
			assert_eq!(
				<Pallet::<Test> as MultiReservableCurrency<AccountId>>::reserved_balance(
					asset_id_local,
					&ACCOUNT_LOCAL
				),
				200
			);
			assert_eq!(
				<Pallet::<Test> as MultiReservableCurrency<AccountId>>::reserved_balance(
					asset_id_foreign,
					&ACCOUNT_FOREIGN
				),
				500
			);

			assert_eq!(
				<<Test as Config>::NativeTransactor>::reserved_balance(&ACCOUNT_NATIVE),
				1500
			);
			assert_eq!(
				<<Test as Config>::LocalTransactor>::reserved_balance(
					asset_id_local,
					&ACCOUNT_LOCAL
				),
				200
			);
			assert_eq!(
				<<Test as Config>::ForeignTransactor>::reserved_balance(
					asset_id_foreign,
					&ACCOUNT_FOREIGN
				),
				500
			);
		});
	}

	#[test]
	fn repatriate_reserved() {
		let protocol_id_local = *b"testloca";
		let nonce_local = 0;
		let protocol_id_foreign = *b"testfore";
		let nonce_foreign = 0;
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
				protocol_id_local,
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
				existential_deposit: 1000,
			};

			let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
				protocol_id_foreign,
				nonce_foreign,
				foreign_asset_info,
				foreign_asset_id,
			)
			.unwrap();

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
			// create accounts
			Pallet::<Test>::mint_into(RuntimeOrigin::root(), asset_id_local, ACCOUNT_TO, 100)
				.unwrap();
			Pallet::<Test>::mint_into(RuntimeOrigin::root(), asset_id_foreign, ACCOUNT_TO, 100)
				.unwrap();
			Pallet::<Test>::mint_into(RuntimeOrigin::root(), NATIVE_ASSET_ID, ACCOUNT_TO, 100)
				.unwrap();
			assert_ok!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::reserve(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				2500,
			));
			assert_ok!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::reserve(
				asset_id_local,
				&ACCOUNT_LOCAL,
				900,
			));
			assert_ok!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::reserve(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				2000,
			));
			assert_ok!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::repatriate_reserved(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				&ACCOUNT_TO,
				300,
				orml_traits::BalanceStatus::Reserved
			));
			assert_ok!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::repatriate_reserved(
				asset_id_local,
				&ACCOUNT_LOCAL,
				&ACCOUNT_TO,
				100,
				orml_traits::BalanceStatus::Reserved
			));
			assert_ok!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::repatriate_reserved(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				&ACCOUNT_TO,
				200,
				orml_traits::BalanceStatus::Reserved
			));
			assert_eq!(
				<<Test as Config>::NativeTransactor>::reserved_balance(&ACCOUNT_NATIVE),
				2200
			);
			assert_eq!(
				<<Test as Config>::LocalTransactor>::reserved_balance(
					asset_id_local,
					&ACCOUNT_LOCAL
				),
				800
			);
			assert_eq!(
				<<Test as Config>::ForeignTransactor>::reserved_balance(
					asset_id_foreign,
					&ACCOUNT_FOREIGN
				),
				1800
			);
			assert_eq!(<<Test as Config>::NativeTransactor>::reserved_balance(&ACCOUNT_TO), 300);
			assert_eq!(
				<<Test as Config>::LocalTransactor>::reserved_balance(asset_id_local, &ACCOUNT_TO),
				100
			);
			assert_eq!(
				<<Test as Config>::ForeignTransactor>::reserved_balance(
					asset_id_foreign,
					&ACCOUNT_TO
				),
				200
			);
			assert_ok!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::repatriate_reserved(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				&ACCOUNT_TO,
				300,
				orml_traits::BalanceStatus::Free
			));
			assert_ok!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::repatriate_reserved(
				asset_id_local,
				&ACCOUNT_LOCAL,
				&ACCOUNT_TO,
				100,
				orml_traits::BalanceStatus::Free
			));
			assert_ok!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::repatriate_reserved(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				&ACCOUNT_TO,
				200,
				orml_traits::BalanceStatus::Free
			));
			assert_eq!(
				<<Test as Config>::NativeTransactor>::reserved_balance(&ACCOUNT_NATIVE),
				1900
			);
			assert_eq!(
				<<Test as Config>::LocalTransactor>::reserved_balance(
					asset_id_local,
					&ACCOUNT_LOCAL
				),
				700
			);
			assert_eq!(
				<<Test as Config>::ForeignTransactor>::reserved_balance(
					asset_id_foreign,
					&ACCOUNT_FOREIGN
				),
				1600
			);
			// initial 100 + 300 slashed
			assert_eq!(<<Test as Config>::NativeTransactor>::free_balance(&ACCOUNT_TO), 400);
			assert_eq!(
				<<Test as Config>::LocalTransactor>::free_balance(asset_id_local, &ACCOUNT_TO),
				200
			);
			assert_eq!(
				<<Test as Config>::ForeignTransactor>::free_balance(asset_id_foreign, &ACCOUNT_TO),
				300
			);
			assert_eq!(<<Test as Config>::NativeTransactor>::total_balance(&ACCOUNT_TO), 700);
			assert_eq!(
				<<Test as Config>::LocalTransactor>::total_balance(asset_id_local, &ACCOUNT_TO),
				300
			);
			assert_eq!(
				<<Test as Config>::ForeignTransactor>::total_balance(asset_id_foreign, &ACCOUNT_TO),
				500
			);
		});
	}
}
mod orml_route_asset_type {
	use composable_traits::{
		assets::{AssetInfo, BiBoundedAssetName, BiBoundedAssetSymbol, CreateAsset},
		xcm::assets::XcmAssetLocation,
	};
	use frame_support::{
		assert_ok,
		traits::{Currency, WithdrawReasons},
	};
	use orml_traits::{MultiCurrency, MultiReservableCurrency};
	

	use crate::mocks::{AccountId, RuntimeOrigin};

	use super::*;

	#[test]
	fn ensure_can_withdraw() {
		let protocol_id_local = *b"testloca";
		let nonce_local = 0;
		let protocol_id_foreign = *b"testfore";
		let nonce_foreign = 0;
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
				protocol_id_local,
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
				existential_deposit: 1000,
			};

			let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
				protocol_id_foreign,
				nonce_foreign,
				foreign_asset_info,
				foreign_asset_id,
			)
			.unwrap();

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

			assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				3000
			),);
			assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(
				asset_id_local,
				&ACCOUNT_LOCAL,
				1000
			),);
			assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				2000
			),);

			assert_ok!(<<Test as Config>::NativeTransactor>::ensure_can_withdraw(
				&ACCOUNT_NATIVE,
				3000,
				WithdrawReasons::all(),
				0
			));
			assert_ok!(<<Test as Config>::LocalTransactor>::ensure_can_withdraw(
				asset_id_local,
				&ACCOUNT_LOCAL,
				1000
			));
			<<Test as Config>::LocalTransactor>::ensure_can_withdraw(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				1000,
			)
			.expect_err("wrong route");
			assert_ok!(<<Test as Config>::ForeignTransactor>::ensure_can_withdraw(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				2000
			),);
		});
	}
	#[test]
	fn transfer() {
		let protocol_id_local = *b"testloca";
		let nonce_local = 0;
		let protocol_id_foreign = *b"testfore";
		let nonce_foreign = 0;
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
				protocol_id_local,
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
				existential_deposit: 1000,
			};

			let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
				protocol_id_foreign,
				nonce_foreign,
				foreign_asset_info,
				foreign_asset_id,
			)
			.unwrap();

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

			assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::transfer(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				&ACCOUNT_TO,
				3000
			),);
			assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::transfer(
				asset_id_local,
				&ACCOUNT_LOCAL,
				&ACCOUNT_TO,
				1000
			),);
			assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::transfer(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				&ACCOUNT_TO,
				2000
			),);

			assert_eq!(<<Test as Config>::NativeTransactor>::total_balance(&ACCOUNT_TO), 3000);
			assert_eq!(
				<<Test as Config>::LocalTransactor>::total_balance(asset_id_local, &ACCOUNT_TO),
				1000
			);
			assert_eq!(
				<<Test as Config>::ForeignTransactor>::total_balance(asset_id_foreign, &ACCOUNT_TO),
				2000
			);
		});
	}
	#[test]
	fn deposit() {
		let protocol_id_local = *b"testloca";
		let nonce_local = 0;
		let protocol_id_foreign = *b"testfore";
		let nonce_foreign = 0;
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
				protocol_id_local,
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
				existential_deposit: 1000,
			};

			let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
				protocol_id_foreign,
				nonce_foreign,
				foreign_asset_info,
				foreign_asset_id,
			)
			.unwrap();

			assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::deposit(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				3000
			),);
			assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::deposit(
				asset_id_local,
				&ACCOUNT_LOCAL,
				1000
			),);
			assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::deposit(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				2000
			),);

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
	#[test]
	fn withdraw() {
		let protocol_id_local = *b"testloca";
		let nonce_local = 0;
		let protocol_id_foreign = *b"testfore";
		let nonce_foreign = 0;
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
				protocol_id_local,
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
				existential_deposit: 1000,
			};

			let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
				protocol_id_foreign,
				nonce_foreign,
				foreign_asset_info,
				foreign_asset_id,
			)
			.unwrap();

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

			assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::withdraw(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				3000
			),);
			assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::withdraw(
				asset_id_local,
				&ACCOUNT_LOCAL,
				1000
			),);
			assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::withdraw(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				2000
			),);

			assert_eq!(<<Test as Config>::NativeTransactor>::total_balance(&ACCOUNT_NATIVE), 0);
			assert_eq!(
				<<Test as Config>::LocalTransactor>::total_balance(asset_id_local, &ACCOUNT_LOCAL),
				0
			);
			assert_eq!(
				<<Test as Config>::ForeignTransactor>::total_balance(
					asset_id_foreign,
					&ACCOUNT_FOREIGN
				),
				0
			);
		});
	}
	#[test]
	fn slash() {
		let protocol_id_local = *b"testloca";
		let nonce_local = 0;
		let protocol_id_foreign = *b"testfore";
		let nonce_foreign = 0;
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
				protocol_id_local,
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
				existential_deposit: 1000,
			};

			let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
				protocol_id_foreign,
				nonce_foreign,
				foreign_asset_info,
				foreign_asset_id,
			)
			.unwrap();

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
				<Pallet::<Test> as MultiCurrency<AccountId>>::slash(
					NATIVE_ASSET_ID,
					&ACCOUNT_NATIVE,
					3000
				),
				0
			);
			assert_eq!(
				<Pallet::<Test> as MultiCurrency<AccountId>>::slash(
					asset_id_local,
					&ACCOUNT_LOCAL,
					1000
				),
				0
			);
			assert_eq!(
				<Pallet::<Test> as MultiCurrency<AccountId>>::slash(
					asset_id_foreign,
					&ACCOUNT_FOREIGN,
					2000
				),
				0
			);

			assert_eq!(<<Test as Config>::NativeTransactor>::total_balance(&ACCOUNT_NATIVE), 0);
			assert_eq!(
				<<Test as Config>::LocalTransactor>::total_balance(asset_id_local, &ACCOUNT_LOCAL),
				0
			);
			assert_eq!(
				<<Test as Config>::ForeignTransactor>::total_balance(
					asset_id_foreign,
					&ACCOUNT_FOREIGN
				),
				0
			);
		});
	}
	#[test]
	fn slash_reserved() {
		let protocol_id_local = *b"testloca";
		let nonce_local = 0;
		let protocol_id_foreign = *b"testfore";
		let nonce_foreign = 0;
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
				protocol_id_local,
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
				existential_deposit: 1000,
			};

			let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
				protocol_id_foreign,
				nonce_foreign,
				foreign_asset_info,
				foreign_asset_id,
			)
			.unwrap();

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
			assert_ok!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::reserve(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				2500,
			));
			assert_ok!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::reserve(
				asset_id_local,
				&ACCOUNT_LOCAL,
				900,
			));
			assert_ok!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::reserve(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				2000,
			));

			assert_eq!(
				<Pallet::<Test> as MultiReservableCurrency<AccountId>>::slash_reserved(
					NATIVE_ASSET_ID,
					&ACCOUNT_NATIVE,
					2500
				),
				0
			);
			assert_eq!(
				<Pallet::<Test> as MultiReservableCurrency<AccountId>>::slash_reserved(
					asset_id_local,
					&ACCOUNT_LOCAL,
					900
				),
				0
			);
			assert_eq!(
				<Pallet::<Test> as MultiReservableCurrency<AccountId>>::slash_reserved(
					asset_id_foreign,
					&ACCOUNT_FOREIGN,
					2000
				),
				0
			);

			assert_eq!(<<Test as Config>::NativeTransactor>::total_balance(&ACCOUNT_NATIVE), 500);
			assert_eq!(
				<<Test as Config>::LocalTransactor>::total_balance(asset_id_local, &ACCOUNT_LOCAL),
				100
			);
			assert_eq!(
				<<Test as Config>::ForeignTransactor>::total_balance(
					asset_id_foreign,
					&ACCOUNT_FOREIGN
				),
				0
			);
		});
	}
}
mod fungibles_route {
	use composable_traits::{
		assets::{AssetInfo, BiBoundedAssetName, BiBoundedAssetSymbol, CreateAsset},
		xcm::assets::XcmAssetLocation,
	};
	use frame_support::{
		assert_ok,
		traits::{
			fungibles::{Inspect, InspectHold, Mutate, MutateHold, Transfer, Unbalanced},
			Currency,
		},
	};
	use orml_traits::{MultiCurrency};

	use crate::mocks::{AccountId, RuntimeOrigin};

	use super::*;

	#[test]
	fn set_balance() {
		let protocol_id_local = *b"testloca";
		let nonce_local = 0;
		let protocol_id_foreign = *b"testfore";
		let nonce_foreign = 0;
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
				protocol_id_local,
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
				existential_deposit: 1000,
			};

			let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
				protocol_id_foreign,
				nonce_foreign,
				foreign_asset_info,
				foreign_asset_id,
			)
			.unwrap();

			// Pallet::<Test>::mint_into(RuntimeOrigin::root(), asset_id_local, ACCOUNT_LOCAL, 1000)
			// 	.unwrap();
			// Pallet::<Test>::mint_into(
			// 	RuntimeOrigin::root(),
			// 	asset_id_foreign,
			// 	ACCOUNT_FOREIGN,
			// 	2000,
			// )
			// .unwrap();
			// Pallet::<Test>::mint_into(RuntimeOrigin::root(), NATIVE_ASSET_ID, ACCOUNT_NATIVE,
			// 3000) 	.unwrap();
			assert_ok!(<Pallet<Test> as Unbalanced<AccountId>>::set_balance(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				3000,
			));
			assert_ok!(<Pallet<Test> as Unbalanced<AccountId>>::set_balance(
				asset_id_local,
				&ACCOUNT_LOCAL,
				1000,
			));
			assert_ok!(<Pallet<Test> as Unbalanced<AccountId>>::set_balance(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				2000,
			));

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
	#[test]
	fn set_total_issuance() {
		let protocol_id_local = *b"testloca";
		let nonce_local = 0;
		let protocol_id_foreign = *b"testfore";
		let nonce_foreign = 0;
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
				protocol_id_local,
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
				existential_deposit: 1000,
			};

			let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
				protocol_id_foreign,
				nonce_foreign,
				foreign_asset_info,
				foreign_asset_id,
			)
			.unwrap();

			<Pallet<Test> as Unbalanced<AccountId>>::set_total_issuance(NATIVE_ASSET_ID, 3000);
			<Pallet<Test> as Unbalanced<AccountId>>::set_total_issuance(asset_id_local, 1000);
			<Pallet<Test> as Unbalanced<AccountId>>::set_total_issuance(asset_id_foreign, 2000);

			assert_eq!(<<Test as Config>::NativeTransactor>::total_issuance(), 3000);
			assert_eq!(<<Test as Config>::LocalTransactor>::total_issuance(asset_id_local), 1000);
			assert_eq!(
				<<Test as Config>::ForeignTransactor>::total_issuance(asset_id_foreign,),
				2000
			);
		});
	}
	#[test]
	fn transfer() {
		let protocol_id_local = *b"testloca";
		let nonce_local = 0;
		let protocol_id_foreign = *b"testfore";
		let nonce_foreign = 0;
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
				protocol_id_local,
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
				existential_deposit: 1000,
			};

			let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
				protocol_id_foreign,
				nonce_foreign,
				foreign_asset_info,
				foreign_asset_id,
			)
			.unwrap();

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

			assert_ok!(<Pallet::<Test> as Transfer<AccountId>>::transfer(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				&ACCOUNT_TO,
				3000,
				false
			));
			assert_ok!(<Pallet::<Test> as Transfer<AccountId>>::transfer(
				asset_id_local,
				&ACCOUNT_LOCAL,
				&ACCOUNT_TO,
				1000,
				false
			));
			assert_ok!(<Pallet::<Test> as Transfer<AccountId>>::transfer(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				&ACCOUNT_TO,
				2000,
				false
			));
			assert_eq!(
				<Pallet::<Test> as Inspect<AccountId>>::balance(NATIVE_ASSET_ID, &ACCOUNT_TO,),
				3000
			);
			assert_eq!(
				<Pallet::<Test> as Inspect<AccountId>>::balance(asset_id_local, &ACCOUNT_TO,),
				1000
			);
			assert_eq!(
				<Pallet::<Test> as Inspect<AccountId>>::balance(asset_id_foreign, &ACCOUNT_TO,),
				2000
			);

			assert_eq!(<<Test as Config>::NativeTransactor>::total_balance(&ACCOUNT_TO), 3000);
			assert_eq!(
				<<Test as Config>::LocalTransactor>::total_balance(asset_id_local, &ACCOUNT_TO),
				1000
			);
			assert_eq!(
				<<Test as Config>::ForeignTransactor>::total_balance(asset_id_foreign, &ACCOUNT_TO),
				2000
			);
		});
	}
	#[test]
	fn hold() {
		let protocol_id_local = *b"testloca";
		let nonce_local = 0;
		let protocol_id_foreign = *b"testfore";
		let nonce_foreign = 0;
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
				protocol_id_local,
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
				existential_deposit: 1000,
			};

			let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
				protocol_id_foreign,
				nonce_foreign,
				foreign_asset_info,
				foreign_asset_id,
			)
			.unwrap();

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

			assert_ok!(<Pallet::<Test> as MutateHold<AccountId>>::hold(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				300,
			));
			assert_ok!(<Pallet::<Test> as MutateHold<AccountId>>::hold(
				asset_id_local,
				&ACCOUNT_LOCAL,
				100,
			));
			assert_ok!(<Pallet::<Test> as MutateHold<AccountId>>::hold(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				200,
			));

			assert_eq!(<<Test as Config>::NativeTransactor>::free_balance(&ACCOUNT_NATIVE), 2700);
			assert_eq!(
				<<Test as Config>::LocalTransactor>::free_balance(asset_id_local, &ACCOUNT_LOCAL),
				900
			);
			assert_eq!(
				<<Test as Config>::ForeignTransactor>::free_balance(
					asset_id_foreign,
					&ACCOUNT_FOREIGN
				),
				1800
			);
			assert_eq!(
				<Pallet::<Test> as InspectHold<AccountId>>::balance_on_hold(
					NATIVE_ASSET_ID,
					&ACCOUNT_NATIVE,
				),
				300
			);
			assert_eq!(
				<Pallet::<Test> as InspectHold<AccountId>>::balance_on_hold(
					asset_id_local,
					&ACCOUNT_LOCAL,
				),
				100
			);
			assert_eq!(
				<Pallet::<Test> as InspectHold<AccountId>>::balance_on_hold(
					asset_id_foreign,
					&ACCOUNT_FOREIGN,
				),
				200
			);
		});
	}
	#[test]
	fn slash() {
		let protocol_id_local = *b"testloca";
		let nonce_local = 0;
		let protocol_id_foreign = *b"testfore";
		let nonce_foreign = 0;
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
				protocol_id_local,
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
				existential_deposit: 1000,
			};

			let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
				protocol_id_foreign,
				nonce_foreign,
				foreign_asset_info,
				foreign_asset_id,
			)
			.unwrap();

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

			assert_ok!(<Pallet<Test> as Mutate<AccountId>>::slash(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				300
			));
			assert_ok!(<Pallet<Test> as Mutate<AccountId>>::slash(
				asset_id_local,
				&ACCOUNT_LOCAL,
				200
			));
			assert_ok!(<Pallet<Test> as Mutate<AccountId>>::slash(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				100
			));

			assert_eq!(<<Test as Config>::NativeTransactor>::total_balance(&ACCOUNT_NATIVE), 2700);
			assert_eq!(
				<<Test as Config>::LocalTransactor>::total_balance(asset_id_local, &ACCOUNT_LOCAL),
				800
			);
			assert_eq!(
				<<Test as Config>::ForeignTransactor>::total_balance(
					asset_id_foreign,
					&ACCOUNT_FOREIGN
				),
				1900
			);
		});
	}
}