use crate::{
    Error, Order, OrderStatus, Pallet,
    mock::*,
};
use frame_support::{assert_ok, assert_noop};

use orml_traits::MultiCurrency;

fn events() -> Vec<Event> {
	let evt = System::events()
		.into_iter()
		.map(|evt| evt.event)
		.collect::<Vec<_>>();

	System::reset_events();
	evt
}

fn last_event() -> Event {
	events().pop().expect("Should have one event")
}

#[test]
fn test_submit_order_should_fail() {
	new_test_ext().execute_with(|| {
		// Test `Error::<T>::SameToFromCurrency`
		assert_noop!(
			Exchange::submit_order(Origin::signed(ALICE), DOT, 1, DOT, 2),
			Error::<TestRuntime>::SameToFromCurrency
		);

		// Test `Error::<T>::OrderWithZeroBal`
		assert_noop!(
			Exchange::submit_order(Origin::signed(ALICE), DOT, 1, BTC, 0),
			Error::<TestRuntime>::OrderWithZeroBal
		);

		// Test `Error::<T>::OrderWithZeroBal`
		assert_noop!(
			Exchange::submit_order(Origin::signed(ALICE), DOT, 0, BTC, 1),
			Error::<TestRuntime>::OrderWithZeroBal
		);

		// Test `Error::<T>::NotEnoughBalance`
		// assert_noop!(
		// 	Exchange::submit_order(Origin::signed(ALICE), BTC, 1, DOT, 1),
		// 	Error::<TestRuntime>::NotEnoughBalance
		// );
	});
}

#[test]
fn test_submit_order_should_succeed() {
	new_test_ext().execute_with(|| {
		assert_ok!(Exchange::submit_order(Origin::signed(ALICE), DOT, 2, BTC, 1));
		assert_eq!(Tokens::free_balance(DOT, &ALICE), ENDOWED_AMT - 2);
		assert_eq!(Tokens::free_balance(BTC, &ALICE), 0);

		// Verify there is an order
		let order = Pallet::<TestRuntime>::orders(0).expect("It should contains an order");
		assert_eq!(order, Order {
			owner: ALICE,
			from_cid: DOT,
			from_bal: 2,
			to_cid: BTC,
			to_bal: 1,
			status: OrderStatus::Alive,
			executed_with: None,
			created_at: 1_u64.into(),
			cancelled_at: None,
			executed_at: None
		});

		// Verify event emitted
		assert_eq!(
			last_event(),
			Event::Exchange(
				crate::Event::OrderSubmitted(ALICE, DOT, 2, BTC, 1)
			)
		);
	});
}

#[test]
fn test_cancel_order() {
	new_test_ext().execute_with(|| {
		// Test `Error::<T>::NotOrderOwner
		assert_noop!(
			Exchange::cancel_order(Origin::signed(ALICE), 0),
			Error::<TestRuntime>::NotOrderOwner
		);

		assert_ok!(Exchange::submit_order(Origin::signed(ALICE), DOT, 2, BTC, 1));

		// Test `Error::<T>::NotOrderOwner
		assert_noop!(
			Exchange::cancel_order(Origin::signed(BOB), 0),
			Error::<TestRuntime>::NotOrderOwner
		);

		// Cancel order successfully
		assert_ok!(Exchange::cancel_order(Origin::signed(ALICE), 0));

		// Amount refunded to Alice
		assert_eq!(Tokens::free_balance(DOT, &ALICE), ENDOWED_AMT);

		// Verify the order is cancelled
		let order = Pallet::<TestRuntime>::orders(0).expect("Order should exist.");
		assert_eq!(order, Order {
			owner: ALICE,
			from_cid: DOT,
			from_bal: 2,
			to_cid: BTC,
			to_bal: 1,
			status: OrderStatus::Cancelled,
			executed_with: None,
			created_at: 1_u64.into(),
			cancelled_at: 1_u64.into(),
			executed_at: None
		});

		// Verify event emitted
		assert_eq!(
			last_event(),
			Event::Exchange(
				crate::Event::OrderCancelled(ALICE, 0)
			)
		);

		// Cannot cancel executed order
		assert_noop!(
			Exchange::cancel_order(Origin::signed(ALICE), 0),
			Error::<TestRuntime>::OrderCannotBeCancelled
		);

	});
}

#[test]
fn test_take_order() {
	new_test_ext().execute_with(|| {
		assert_ok!(Exchange::submit_order(Origin::signed(ALICE), DOT, 2, BTC, 1));

		// Test `Error::<T>::OrderNotExist`
		assert_noop!(
			Exchange::take_order(Origin::signed(BOB), 1),
			Error::<TestRuntime>::OrderNotExist
		);

		// Test `Error::<T>::NotEnoughBalance`
		assert_noop!(
			Exchange::take_order(Origin::signed(CHARLIE), 0),
			Error::<TestRuntime>::NotEnoughBalance
		);

		assert_ok!(Exchange::take_order(Origin::signed(BOB), 0));

		// Verify ALICE and BOB balance has updated
		assert_eq!(Tokens::free_balance(DOT, &ALICE), ENDOWED_AMT - 2);
		assert_eq!(Tokens::free_balance(BTC, &ALICE), 1);
		assert_eq!(Tokens::free_balance(DOT, &BOB), 2);
		assert_eq!(Tokens::free_balance(BTC, &BOB), ENDOWED_AMT - 1);

		// Verify the order status
		let order = Pallet::<TestRuntime>::orders(0).expect("It should contains an order");
		assert_eq!(order, Order {
			owner: ALICE,
			from_cid: DOT,
			from_bal: 2,
			to_cid: BTC,
			to_bal: 1,
			status: OrderStatus::Executed,
			executed_with: Some(BOB),
			created_at: 1_u64.into(),
			cancelled_at: None,
			executed_at: 1_u64.into()
		});

		// Verify emitted
		assert_eq!(
			last_event(),
			Event::Exchange(
				crate::Event::OrderTaken(BOB, 0)
			)
		);

		// Cannot cancel executed order
		assert_noop!(
			Exchange::cancel_order(Origin::signed(ALICE), 0),
			Error::<TestRuntime>::OrderCannotBeCancelled
		);
	});
}
