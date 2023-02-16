use frame_support::{
	assert_ok,
	traits::{Currency, WithdrawReasons},
};
use orml_traits::{MultiCurrency, MultiReservableCurrency};

use super::*;

use crate::{
	mocks::{new_test_ext, AccountId, Test},
	*,
};

#[test]
fn ensure_can_withdraw() {
	// mint assets. check that can withdraw via common interface and per transactor
	new_test_ext().execute_with(|| {
		let (asset_id_local, asset_id_foreign) = create_assets();
		mint_assets(asset_id_local, asset_id_foreign);

		assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(
			NATIVE_ASSET_ID,
			&ACCOUNT_NATIVE,
			ACCOUNT_NATIVE_BALANCE
		));
		assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(
			asset_id_local,
			&ACCOUNT_LOCAL,
			ACCOUNT_LOCAL_BALANCE
		));
		assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(
			asset_id_foreign,
			&ACCOUNT_FOREIGN,
			ACCOUNT_FOREIGN_BALANCE
		));

		assert_ok!(NativeTransactor::ensure_can_withdraw(
			&ACCOUNT_NATIVE,
			ACCOUNT_NATIVE_BALANCE,
			WithdrawReasons::all(),
			0
		));
		assert_ok!(LocalTransactor::ensure_can_withdraw(
			asset_id_local,
			&ACCOUNT_LOCAL,
			ACCOUNT_LOCAL_BALANCE
		));
		LocalTransactor::ensure_can_withdraw(
			NATIVE_ASSET_ID,
			&ACCOUNT_NATIVE,
			ACCOUNT_LOCAL_BALANCE,
		)
		.expect_err("wrong route");
		assert_ok!(ForeignTransactor::ensure_can_withdraw(
			asset_id_foreign,
			&ACCOUNT_FOREIGN,
			ACCOUNT_FOREIGN_BALANCE
		));
	});
}

#[test]
fn transfer() {
	// mint asserface transfer o ACCOUNT_TO, check total_balance per transactor
	new_test_ext().execute_with(|| {
		let (asset_id_local, asset_id_foreign) = create_assets();
		mint_assets(asset_id_local, asset_id_foreign);
		assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::transfer(
			NATIVE_ASSET_ID,
			&ACCOUNT_NATIVE,
			&ACCOUNT_TO,
			ACCOUNT_NATIVE_BALANCE
		));
		assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::transfer(
			asset_id_local,
			&ACCOUNT_LOCAL,
			&ACCOUNT_TO,
			ACCOUNT_LOCAL_BALANCE
		));
		assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::transfer(
			asset_id_foreign,
			&ACCOUNT_FOREIGN,
			&ACCOUNT_TO,
			ACCOUNT_FOREIGN_BALANCE
		));

		assert_eq!(NativeTransactor::total_balance(&ACCOUNT_TO), ACCOUNT_NATIVE_BALANCE);
		assert_eq!(
			LocalTransactor::total_balance(asset_id_local, &ACCOUNT_TO),
			ACCOUNT_LOCAL_BALANCE
		);
		assert_eq!(
			ForeignTransactor::total_balance(asset_id_foreign, &ACCOUNT_TO),
			ACCOUNT_FOREIGN_BALANCE
		);
	});
}

#[test]
fn deposit() {
	// deposit assets via common interface, check total balances per transactor
	new_test_ext().execute_with(|| {
		let (asset_id_local, asset_id_foreign) = create_assets();

		assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::deposit(
			NATIVE_ASSET_ID,
			&ACCOUNT_NATIVE,
			ACCOUNT_NATIVE_BALANCE
		));
		assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::deposit(
			asset_id_local,
			&ACCOUNT_LOCAL,
			ACCOUNT_LOCAL_BALANCE
		));
		assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::deposit(
			asset_id_foreign,
			&ACCOUNT_FOREIGN,
			ACCOUNT_FOREIGN_BALANCE
		));

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
fn withdraw() {
	// mint assets. via common interface withdraw amounts, check total balances per transactor
	new_test_ext().execute_with(|| {
		let (asset_id_local, asset_id_foreign) = create_assets();
		mint_assets(asset_id_local, asset_id_foreign);

		assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::withdraw(
			NATIVE_ASSET_ID,
			&ACCOUNT_NATIVE,
			ACCOUNT_NATIVE_BALANCE
		));
		assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::withdraw(
			asset_id_local,
			&ACCOUNT_LOCAL,
			ACCOUNT_LOCAL_BALANCE
		));
		assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::withdraw(
			asset_id_foreign,
			&ACCOUNT_FOREIGN,
			ACCOUNT_FOREIGN_BALANCE
		));

		assert_eq!(NativeTransactor::total_balance(&ACCOUNT_NATIVE), 0);
		assert_eq!(LocalTransactor::total_balance(asset_id_local, &ACCOUNT_LOCAL), 0);
		assert_eq!(ForeignTransactor::total_balance(asset_id_foreign, &ACCOUNT_FOREIGN), 0);
	});
}

#[test]
fn slash() {
	// mint assets. via common interface slash them, check balances per transactor
	new_test_ext().execute_with(|| {
		let (asset_id_local, asset_id_foreign) = create_assets();
		mint_assets(asset_id_local, asset_id_foreign);

		assert_eq!(
			<Pallet<Test> as MultiCurrency<AccountId>>::slash(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				ACCOUNT_NATIVE_BALANCE
			),
			0
		);
		assert_eq!(
			<Pallet<Test> as MultiCurrency<AccountId>>::slash(
				asset_id_local,
				&ACCOUNT_LOCAL,
				ACCOUNT_LOCAL_BALANCE
			),
			0
		);
		assert_eq!(
			<Pallet<Test> as MultiCurrency<AccountId>>::slash(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				ACCOUNT_FOREIGN_BALANCE
			),
			0
		);

		assert_eq!(NativeTransactor::total_balance(&ACCOUNT_NATIVE), 0);
		assert_eq!(LocalTransactor::total_balance(asset_id_local, &ACCOUNT_LOCAL), 0);
		assert_eq!(ForeignTransactor::total_balance(asset_id_foreign, &ACCOUNT_FOREIGN), 0);
	});
}

#[test]
fn slash_reserved() {
	// mint assets. via common interface reserve them, slash reserved.
	// check total and reserved balances are correct per transactor
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
			<Pallet<Test> as MultiReservableCurrency<AccountId>>::slash_reserved(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				2500
			),
			0
		);
		assert_eq!(
			<Pallet<Test> as MultiReservableCurrency<AccountId>>::slash_reserved(
				asset_id_local,
				&ACCOUNT_LOCAL,
				900
			),
			0
		);
		assert_eq!(
			<Pallet<Test> as MultiReservableCurrency<AccountId>>::slash_reserved(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				ACCOUNT_FOREIGN_BALANCE
			),
			0
		);

		assert_eq!(NativeTransactor::total_balance(&ACCOUNT_NATIVE), 500);
		assert_eq!(LocalTransactor::total_balance(asset_id_local, &ACCOUNT_LOCAL), 100);
		assert_eq!(ForeignTransactor::total_balance(asset_id_foreign, &ACCOUNT_FOREIGN), 0);
		assert_eq!(NativeTransactor::reserved_balance(&ACCOUNT_NATIVE), 0);
		assert_eq!(LocalTransactor::reserved_balance(asset_id_local, &ACCOUNT_LOCAL), 0);
		assert_eq!(ForeignTransactor::reserved_balance(asset_id_foreign, &ACCOUNT_FOREIGN), 0);
	});
}
