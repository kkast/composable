use frame_support::{
	assert_ok,
	traits::{Currency, ReservableCurrency},
};
use orml_traits::{MultiCurrency, MultiReservableCurrency};

use super::*;

use crate::{
	mocks::{new_test_ext, AccountId, RuntimeOrigin, Test},
	*,
};

#[test]
fn minimum_balance() {
	// tests ED specified in pallet's Configs via common interface and per transactor
	new_test_ext().execute_with(|| {
		let (asset_id_local, asset_id_foreign) = create_assets();
		assert_eq!(<Pallet<Test> as MultiCurrency<AccountId>>::minimum_balance(NATIVE_ASSET_ID), 1);
		// should be 0 because of ExistentialDeposits for orml_tokens
		assert_eq!(<Pallet<Test> as MultiCurrency<AccountId>>::minimum_balance(asset_id_local), 0);
		// should be 0 because of ExistentialDeposits for orml_tokens
		assert_eq!(
			<Pallet<Test> as MultiCurrency<AccountId>>::minimum_balance(asset_id_foreign),
			0
		);

		assert_eq!(NativeTransactor::minimum_balance(), 1);
		assert_eq!(LocalTransactor::minimum_balance(asset_id_local), 0);
		assert_eq!(ForeignTransactor::minimum_balance(asset_id_foreign), 0);
	});
}

#[test]
fn total_issuance() {
	// mint assets and check new issuance via common interface and per transactor
	new_test_ext().execute_with(|| {
		let (asset_id_local, asset_id_foreign) = create_assets();
		mint_assets(asset_id_local, asset_id_foreign);
		assert_eq!(
			<Pallet<Test> as MultiCurrency<AccountId>>::total_issuance(NATIVE_ASSET_ID),
			ACCOUNT_NATIVE_BALANCE
		);
		assert_eq!(
			<Pallet<Test> as MultiCurrency<AccountId>>::total_issuance(asset_id_local),
			ACCOUNT_LOCAL_BALANCE
		);
		assert_eq!(
			<Pallet<Test> as MultiCurrency<AccountId>>::total_issuance(asset_id_foreign),
			ACCOUNT_FOREIGN_BALANCE
		);

		assert_eq!(NativeTransactor::total_issuance(), ACCOUNT_NATIVE_BALANCE);
		assert_eq!(LocalTransactor::total_issuance(asset_id_local), ACCOUNT_LOCAL_BALANCE);
		assert_eq!(ForeignTransactor::total_issuance(asset_id_foreign), ACCOUNT_FOREIGN_BALANCE);
	});
}

#[test]
fn total_balance() {
	// mint assets and check that balances are correct
	new_test_ext().execute_with(|| {
		let (asset_id_local, asset_id_foreign) = create_assets();
		mint_assets(asset_id_local, asset_id_foreign);

		assert_eq!(
			<Pallet<Test> as MultiCurrency<AccountId>>::total_balance(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE
			),
			ACCOUNT_NATIVE_BALANCE
		);
		assert_eq!(
			<Pallet<Test> as MultiCurrency<AccountId>>::total_balance(
				asset_id_local,
				&ACCOUNT_LOCAL
			),
			ACCOUNT_LOCAL_BALANCE
		);
		assert_eq!(
			<Pallet<Test> as MultiCurrency<AccountId>>::total_balance(
				asset_id_foreign,
				&ACCOUNT_FOREIGN
			),
			ACCOUNT_FOREIGN_BALANCE
		);

		assert_eq!(NativeTransactor::total_balance(&ACCOUNT_NATIVE), ACCOUNT_NATIVE_BALANCE);
		assert_eq!(
			LocalTransactor::total_balance(asset_id_local, &ACCOUNT_LOCAL),
			ACCOUNT_LOCAL_BALANCE
		);
		assert_eq!(
			ForeignTransactor::total_balance(asset_id_foreign, &ACCOUNT_FOREIGN),
			ACCOUNT_FOREIGN_BALANCE
		);
	});
}

#[test]
fn free_balance() {
	// mint assets. via common interface check that all amounts are free balances
	// check that free balances are correct per transactor
	new_test_ext().execute_with(|| {
		let (asset_id_local, asset_id_foreign) = create_assets();
		mint_assets(asset_id_local, asset_id_foreign);

		assert_eq!(
			<Pallet<Test> as MultiCurrency<AccountId>>::free_balance(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE
			),
			ACCOUNT_NATIVE_BALANCE
		);
		assert_eq!(
			<Pallet<Test> as MultiCurrency<AccountId>>::free_balance(
				asset_id_local,
				&ACCOUNT_LOCAL
			),
			ACCOUNT_LOCAL_BALANCE
		);
		assert_eq!(
			<Pallet<Test> as MultiCurrency<AccountId>>::free_balance(
				asset_id_foreign,
				&ACCOUNT_FOREIGN
			),
			ACCOUNT_FOREIGN_BALANCE
		);

		assert_eq!(NativeTransactor::free_balance(&ACCOUNT_NATIVE), ACCOUNT_NATIVE_BALANCE);
		assert_eq!(
			LocalTransactor::free_balance(asset_id_local, &ACCOUNT_LOCAL),
			ACCOUNT_LOCAL_BALANCE
		);
		assert_eq!(
			ForeignTransactor::free_balance(asset_id_foreign, &ACCOUNT_FOREIGN),
			ACCOUNT_FOREIGN_BALANCE
		);
	});
}

#[test]
fn can_slash() {
	// mint assets. via common interface check that can_slash
	// check that can_slash per transactor
	new_test_ext().execute_with(|| {
		let (asset_id_local, asset_id_foreign) = create_assets();
		mint_assets(asset_id_local, asset_id_foreign);
		assert!(<Pallet<Test> as MultiCurrency<AccountId>>::can_slash(
			NATIVE_ASSET_ID,
			&ACCOUNT_NATIVE,
			ACCOUNT_NATIVE_BALANCE
		));
		assert!(<Pallet<Test> as MultiCurrency<AccountId>>::can_slash(
			asset_id_local,
			&ACCOUNT_LOCAL,
			ACCOUNT_LOCAL_BALANCE
		));
		assert!(<Pallet<Test> as MultiCurrency<AccountId>>::can_slash(
			asset_id_foreign,
			&ACCOUNT_FOREIGN,
			ACCOUNT_FOREIGN_BALANCE
		));

		assert!(NativeTransactor::can_slash(&ACCOUNT_NATIVE, ACCOUNT_NATIVE_BALANCE));
		assert!(LocalTransactor::can_slash(asset_id_local, &ACCOUNT_LOCAL, ACCOUNT_LOCAL_BALANCE));
		assert!(ForeignTransactor::can_slash(
			asset_id_foreign,
			&ACCOUNT_FOREIGN,
			ACCOUNT_FOREIGN_BALANCE
		));
	});
}

#[test]
fn can_reserve() {
	// mint assets. via common interface check that can reserve
	// check that can reserve per transactor
	new_test_ext().execute_with(|| {
		let (asset_id_local, asset_id_foreign) = create_assets();
		mint_assets(asset_id_local, asset_id_foreign);

		assert!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::can_reserve(
			NATIVE_ASSET_ID,
			&ACCOUNT_NATIVE,
			ACCOUNT_NATIVE_BALANCE
		));
		assert!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::can_reserve(
			asset_id_local,
			&ACCOUNT_LOCAL,
			ACCOUNT_LOCAL_BALANCE
		));
		assert!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::can_reserve(
			asset_id_foreign,
			&ACCOUNT_FOREIGN,
			ACCOUNT_FOREIGN_BALANCE
		));

		assert!(NativeTransactor::can_reserve(&ACCOUNT_NATIVE, ACCOUNT_NATIVE_BALANCE));
		assert!(LocalTransactor::can_reserve(
			asset_id_local,
			&ACCOUNT_LOCAL,
			ACCOUNT_LOCAL_BALANCE
		));
		assert!(ForeignTransactor::can_reserve(
			asset_id_foreign,
			&ACCOUNT_FOREIGN,
			ACCOUNT_FOREIGN_BALANCE
		));
	});
}

#[test]
fn reserve() {
	// mint assets. via common interface reserve check that reserved balances are correct
	// check that reserved balances are correct per transactor
	// unreserve some of the reserved funds and check reserved balance via common interface
	// check that reserved balances are correct per transactor
	new_test_ext().execute_with(|| {
		let (asset_id_local, asset_id_foreign) = create_assets();
		mint_assets(asset_id_local, asset_id_foreign);

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
			ACCOUNT_FOREIGN_BALANCE,
		));
		assert_eq!(
			<Pallet<Test> as MultiReservableCurrency<AccountId>>::reserved_balance(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE
			),
			2500
		);
		assert_eq!(
			<Pallet<Test> as MultiReservableCurrency<AccountId>>::reserved_balance(
				asset_id_local,
				&ACCOUNT_LOCAL
			),
			900
		);
		assert_eq!(
			<Pallet<Test> as MultiReservableCurrency<AccountId>>::reserved_balance(
				asset_id_foreign,
				&ACCOUNT_FOREIGN
			),
			ACCOUNT_FOREIGN_BALANCE
		);

		assert_eq!(NativeTransactor::reserved_balance(&ACCOUNT_NATIVE), 2500);
		assert_eq!(LocalTransactor::reserved_balance(asset_id_local, &ACCOUNT_LOCAL), 900);
		assert_eq!(
			ForeignTransactor::reserved_balance(asset_id_foreign, &ACCOUNT_FOREIGN),
			ACCOUNT_FOREIGN_BALANCE
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
			<Pallet<Test> as MultiReservableCurrency<AccountId>>::reserved_balance(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE
			),
			1500
		);
		assert_eq!(
			<Pallet<Test> as MultiReservableCurrency<AccountId>>::reserved_balance(
				asset_id_local,
				&ACCOUNT_LOCAL
			),
			200
		);
		assert_eq!(
			<Pallet<Test> as MultiReservableCurrency<AccountId>>::reserved_balance(
				asset_id_foreign,
				&ACCOUNT_FOREIGN
			),
			500
		);

		assert_eq!(NativeTransactor::reserved_balance(&ACCOUNT_NATIVE), 1500);
		assert_eq!(LocalTransactor::reserved_balance(asset_id_local, &ACCOUNT_LOCAL), 200);
		assert_eq!(ForeignTransactor::reserved_balance(asset_id_foreign, &ACCOUNT_FOREIGN), 500);
	});
}

#[test]
fn repatriate_reserved() {
	// mint assets, reserve some of these amounts via common interface
	// create initial balances for account ACCOUNT_TO for local, foreign, native assets
	// via common interface repatriate reserved amounts to ACCOUNT_TO's reserved balance, check
	// all accounts' reserved balances per transactor via common interface repatriate reserved
	// amounts to ACCOUNT_TO's free balance, check source account's reserved balances and
	// reciever's free balance per transactor
	new_test_ext().execute_with(|| {
		let (asset_id_local, asset_id_foreign) = create_assets();
		mint_assets(asset_id_local, asset_id_foreign);

		// create accounts
		Pallet::<Test>::mint_into(RuntimeOrigin::root(), asset_id_local, ACCOUNT_TO, 100).unwrap();
		Pallet::<Test>::mint_into(RuntimeOrigin::root(), asset_id_foreign, ACCOUNT_TO, 100)
			.unwrap();
		Pallet::<Test>::mint_into(RuntimeOrigin::root(), NATIVE_ASSET_ID, ACCOUNT_TO, 100).unwrap();
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
			ACCOUNT_FOREIGN_BALANCE,
		));
		// test repatriate to reserved
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
		assert_eq!(NativeTransactor::reserved_balance(&ACCOUNT_NATIVE), 2200);
		assert_eq!(LocalTransactor::reserved_balance(asset_id_local, &ACCOUNT_LOCAL), 800);
		assert_eq!(ForeignTransactor::reserved_balance(asset_id_foreign, &ACCOUNT_FOREIGN), 1800);
		assert_eq!(NativeTransactor::reserved_balance(&ACCOUNT_TO), 300);
		assert_eq!(LocalTransactor::reserved_balance(asset_id_local, &ACCOUNT_TO), 100);
		assert_eq!(ForeignTransactor::reserved_balance(asset_id_foreign, &ACCOUNT_TO), 200);
		// test repatriate to free
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
		assert_eq!(NativeTransactor::reserved_balance(&ACCOUNT_NATIVE), 1900);
		assert_eq!(LocalTransactor::reserved_balance(asset_id_local, &ACCOUNT_LOCAL), 700);
		assert_eq!(ForeignTransactor::reserved_balance(asset_id_foreign, &ACCOUNT_FOREIGN), 1600);
		// initial 100 + 300 slashed
		assert_eq!(NativeTransactor::free_balance(&ACCOUNT_TO), 400);
		// initial 100 + 100 slashed
		assert_eq!(LocalTransactor::free_balance(asset_id_local, &ACCOUNT_TO), 200);
		// initial 100 + 200 slashed
		assert_eq!(ForeignTransactor::free_balance(asset_id_foreign, &ACCOUNT_TO), 300);
		assert_eq!(NativeTransactor::total_balance(&ACCOUNT_TO), 700);
		assert_eq!(LocalTransactor::total_balance(asset_id_local, &ACCOUNT_TO), 300);
		assert_eq!(ForeignTransactor::total_balance(asset_id_foreign, &ACCOUNT_TO), 500);
	});
}
