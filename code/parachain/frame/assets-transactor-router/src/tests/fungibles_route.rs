use frame_support::{
	assert_ok,
	traits::{
		fungibles::{Inspect, InspectHold, Mutate, MutateHold, Transfer, Unbalanced},
		Currency,
	},
};
use orml_traits::MultiCurrency;

use mocks::{new_test_ext, Test};

use super::*;
use crate::{mocks::AccountId, *};

#[test]
fn set_balance() {
	// set balances for assets via common interface, check that total balance is correct per
	// transactor
	new_test_ext().execute_with(|| {
		let (asset_id_local, asset_id_foreign) = create_assets();
		assert_ok!(<Pallet<Test> as Unbalanced<AccountId>>::set_balance(
			NATIVE_ASSET_ID,
			&ACCOUNT_NATIVE,
			ACCOUNT_NATIVE_BALANCE,
		));
		assert_ok!(<Pallet<Test> as Unbalanced<AccountId>>::set_balance(
			asset_id_local,
			&ACCOUNT_LOCAL,
			ACCOUNT_LOCAL_BALANCE,
		));
		assert_ok!(<Pallet<Test> as Unbalanced<AccountId>>::set_balance(
			asset_id_foreign,
			&ACCOUNT_FOREIGN,
			ACCOUNT_FOREIGN_BALANCE,
		));

		assert_eq!(
			<<Test as Config>::NativeTransactor>::total_balance(&ACCOUNT_NATIVE),
			ACCOUNT_NATIVE_BALANCE
		);
		assert_eq!(
			<<Test as Config>::LocalTransactor>::total_balance(asset_id_local, &ACCOUNT_LOCAL),
			ACCOUNT_LOCAL_BALANCE
		);
		assert_eq!(
			<<Test as Config>::ForeignTransactor>::total_balance(
				asset_id_foreign,
				&ACCOUNT_FOREIGN
			),
			ACCOUNT_FOREIGN_BALANCE
		);
	});
}

#[test]
fn set_total_issuance() {
	// set assets' issuance for assets via common interfact, check that total issuance is
	// correct per transactor
	new_test_ext().execute_with(|| {
		let (asset_id_local, asset_id_foreign) = create_assets();

		<Pallet<Test> as Unbalanced<AccountId>>::set_total_issuance(
			NATIVE_ASSET_ID,
			ACCOUNT_NATIVE_BALANCE,
		);
		<Pallet<Test> as Unbalanced<AccountId>>::set_total_issuance(
			asset_id_local,
			ACCOUNT_LOCAL_BALANCE,
		);
		<Pallet<Test> as Unbalanced<AccountId>>::set_total_issuance(
			asset_id_foreign,
			ACCOUNT_FOREIGN_BALANCE,
		);

		assert_eq!(<<Test as Config>::NativeTransactor>::total_issuance(), ACCOUNT_NATIVE_BALANCE);
		assert_eq!(
			<<Test as Config>::LocalTransactor>::total_issuance(asset_id_local),
			ACCOUNT_LOCAL_BALANCE
		);
		assert_eq!(
			<<Test as Config>::ForeignTransactor>::total_issuance(asset_id_foreign),
			ACCOUNT_FOREIGN_BALANCE
		);
	});
}

#[test]
fn transfer() {
	// mint assets, transfer and check balance using common interface, test balances per transactor
	new_test_ext().execute_with(|| {
		let (asset_id_local, asset_id_foreign) = create_assets();
		mint_assets(asset_id_local, asset_id_foreign);

		assert_ok!(<Pallet<Test> as Transfer<AccountId>>::transfer(
			NATIVE_ASSET_ID,
			&ACCOUNT_NATIVE,
			&ACCOUNT_TO,
			ACCOUNT_NATIVE_BALANCE,
			false
		));
		assert_ok!(<Pallet<Test> as Transfer<AccountId>>::transfer(
			asset_id_local,
			&ACCOUNT_LOCAL,
			&ACCOUNT_TO,
			ACCOUNT_LOCAL_BALANCE,
			false
		));
		assert_ok!(<Pallet<Test> as Transfer<AccountId>>::transfer(
			asset_id_foreign,
			&ACCOUNT_FOREIGN,
			&ACCOUNT_TO,
			ACCOUNT_FOREIGN_BALANCE,
			false
		));
		assert_eq!(
			<Pallet<Test> as Inspect<AccountId>>::balance(NATIVE_ASSET_ID, &ACCOUNT_TO),
			ACCOUNT_NATIVE_BALANCE
		);
		assert_eq!(
			<Pallet<Test> as Inspect<AccountId>>::balance(asset_id_local, &ACCOUNT_TO),
			ACCOUNT_LOCAL_BALANCE
		);
		assert_eq!(
			<Pallet<Test> as Inspect<AccountId>>::balance(asset_id_foreign, &ACCOUNT_TO),
			ACCOUNT_FOREIGN_BALANCE
		);

		assert_eq!(
			<<Test as Config>::NativeTransactor>::total_balance(&ACCOUNT_TO),
			ACCOUNT_NATIVE_BALANCE
		);
		assert_eq!(
			<<Test as Config>::LocalTransactor>::total_balance(asset_id_local, &ACCOUNT_TO),
			ACCOUNT_LOCAL_BALANCE
		);
		assert_eq!(
			<<Test as Config>::ForeignTransactor>::total_balance(asset_id_foreign, &ACCOUNT_TO),
			ACCOUNT_FOREIGN_BALANCE
		);
	});
}

#[test]
fn hold() {
	// mint assets, hold  and check balance_on_hold using common interface, test free balances
	// per transactor
	new_test_ext().execute_with(|| {
		let (asset_id_local, asset_id_foreign) = create_assets();
		mint_assets(asset_id_local, asset_id_foreign);

		assert_ok!(<Pallet<Test> as MutateHold<AccountId>>::hold(
			NATIVE_ASSET_ID,
			&ACCOUNT_NATIVE,
			300,
		));
		assert_ok!(<Pallet<Test> as MutateHold<AccountId>>::hold(
			asset_id_local,
			&ACCOUNT_LOCAL,
			100,
		));
		assert_ok!(<Pallet<Test> as MutateHold<AccountId>>::hold(
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
			<<Test as Config>::ForeignTransactor>::free_balance(asset_id_foreign, &ACCOUNT_FOREIGN),
			1800
		);
		assert_eq!(
			<Pallet<Test> as InspectHold<AccountId>>::balance_on_hold(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
			),
			300
		);
		assert_eq!(
			<Pallet<Test> as InspectHold<AccountId>>::balance_on_hold(
				asset_id_local,
				&ACCOUNT_LOCAL,
			),
			100
		);
		assert_eq!(
			<Pallet<Test> as InspectHold<AccountId>>::balance_on_hold(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
			),
			200
		);
	});
}

#[test]
fn slash() {
	// mint assets, slash using common interface, check total balances per transactor
	new_test_ext().execute_with(|| {
		let (asset_id_local, asset_id_foreign) = create_assets();
		mint_assets(asset_id_local, asset_id_foreign);

		assert_ok!(<Pallet<Test> as Mutate<AccountId>>::slash(
			NATIVE_ASSET_ID,
			&ACCOUNT_NATIVE,
			300
		));
		assert_ok!(<Pallet<Test> as Mutate<AccountId>>::slash(asset_id_local, &ACCOUNT_LOCAL, 200));
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
