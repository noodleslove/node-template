//! Unit tests for the non-fungible-token module.

#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::*;

use orml_nft::{Classes, NextTokenId};

#[test]
fn create_class_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		let next_class_id = NonFungibleTokenModule::next_class_id();
		assert_eq!(next_class_id, CLASS_ID);
		assert_ok!(
			Items::create_item_class(Origin::signed(ALICE), vec![1], ItemClassData {
				item_type: ItemType::OfflineEvent,
				info: b"info".to_vec(),
				uri: b"https://fantour.io".to_vec(),
				poster: b"https://fantour.io".to_vec(),

				start_time: 0,
				end_time: 0,
				start_sale_time: 0,
				end_sale_time: 0,

				inspector: ALICE,

				created_at: Some(<frame_system::Pallet<Runtime>>::block_number()),
			})
		);
		assert_eq!(NonFungibleTokenModule::next_token_id(CLASS_ID), 0);
		assert_eq!(NonFungibleTokenModule::next_class_id(), next_class_id + 1);
		let next_class_id = NonFungibleTokenModule::next_class_id();
		assert_ok!(
			Items::create_item_class(Origin::signed(BOB), vec![1], ItemClassData {
				item_type: ItemType::OfflineEvent,
				info: b"info".to_vec(),
				uri: b"https://fantour.io".to_vec(),
				poster: b"https://fantour.io".to_vec(),

				start_time: 0,
				end_time: 0,
				start_sale_time: 0,
				end_sale_time: 0,

				inspector: ALICE,

				created_at: Some(<frame_system::Pallet<Runtime>>::block_number()),
			})
		);
		assert_eq!(NonFungibleTokenModule::next_token_id(CLASS_ID), 0);
		assert_eq!(NonFungibleTokenModule::next_class_id(), next_class_id + 1);
	});
}

#[test]
fn destroy_item_class_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		let next_class_id = NonFungibleTokenModule::next_class_id();
		assert_eq!(next_class_id, CLASS_ID);
		assert_ok!(
			Items::create_item_class(Origin::signed(ALICE), vec![1], ItemClassData {
				item_type: ItemType::OfflineEvent,
				info: b"info".to_vec(),
				uri: b"https://fantour.io".to_vec(),
				poster: b"https://fantour.io".to_vec(),

				start_time: 0,
				end_time: 0,
				start_sale_time: 0,
				end_sale_time: 0,

				inspector: ALICE,

				created_at: Some(<frame_system::Pallet<Runtime>>::block_number()),
			})
		);
		assert_eq!(NonFungibleTokenModule::next_token_id(CLASS_ID), 0);
		assert_ok!(Items::destroy_item_class(Origin::signed(ALICE), next_class_id));
		let next_class_id = NonFungibleTokenModule::next_class_id();
		assert_ok!(
			Items::create_item_class(Origin::signed(BOB), vec![1], ItemClassData {
				item_type: ItemType::OfflineEvent,
				info: b"info".to_vec(),
				uri: b"https://fantour.io".to_vec(),
				poster: b"https://fantour.io".to_vec(),

				start_time: 0,
				end_time: 0,
				start_sale_time: 0,
				end_sale_time: 0,

				inspector: ALICE,

				created_at: Some(<frame_system::Pallet<Runtime>>::block_number()),
			})
		);
		assert_eq!(NonFungibleTokenModule::next_token_id(CLASS_ID), 0);
		assert_ok!(Items::destroy_item_class(Origin::signed(BOB), next_class_id));
	});
}

#[test]
fn destroy_item_class_should_fail() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(
			Items::create_item_class(Origin::signed(BOB), vec![1], ItemClassData {
				item_type: ItemType::OfflineEvent,
				info: b"info".to_vec(),
				uri: b"https://fantour.io".to_vec(),
				poster: b"https://fantour.io".to_vec(),

				start_time: 0,
				end_time: 0,
				start_sale_time: 0,
				end_sale_time: 0,

				inspector: ALICE,

				created_at: Some(<frame_system::Pallet<Runtime>>::block_number()),
			})
		);
		assert_noop!(
			Items::destroy_item_class(Origin::signed(ALICE), CLASS_ID),
			Error::<Runtime>::NotClassOwner,
		);
		assert_ok!(
			Items::create_item_class(Origin::signed(ALICE), vec![1], ItemClassData {
				item_type: ItemType::OfflineEvent,
				info: b"info".to_vec(),
				uri: b"https://fantour.io".to_vec(),
				poster: b"https://fantour.io".to_vec(),

				start_time: 0,
				end_time: 0,
				start_sale_time: 0,
				end_sale_time: 0,

				inspector: ALICE,

				created_at: Some(<frame_system::Pallet<Runtime>>::block_number()),
			})
		);
		assert_noop!(
			Items::destroy_item_class(Origin::signed(ALICE), CLASS_ID_NOT_EXIST),
			Error::<Runtime>::ClassNotExists,
		);
	});
}

#[test]
fn mint_item_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		let next_class_id = NonFungibleTokenModule::next_class_id();
		assert_eq!(next_class_id, CLASS_ID);
		assert_ok!(
			NonFungibleTokenModule::create_class(&ALICE, vec![1], ItemClassData {
				item_type: ItemType::OfflineEvent,
				info: b"info".to_vec(),
				uri: b"https://fantour.io".to_vec(),
				poster: b"https://fantour.io".to_vec(),

				start_time: 0,
				end_time: 0,
				start_sale_time: 0,
				end_sale_time: 0,

				inspector: ALICE,

				created_at: Some(<frame_system::Pallet<Runtime>>::block_number()),
			})
		);
		assert_eq!(NonFungibleTokenModule::next_token_id(CLASS_ID), 0);
		assert_ok!(Items::mint_item(Origin::signed(ALICE), CLASS_ID, vec![1], ItemTokenData {
			name: b"name".to_vec(),
			price: 100,
			zone_id: 1,
			seat_id: Some(1),
			status: ItemStatus::Unchecked,

			created_at: Some(<frame_system::Pallet<Runtime>>::block_number()),
			inspected_at: None,
			inspected_with: None,
		}));
		assert_eq!(NonFungibleTokenModule::next_token_id(CLASS_ID), 1);
		assert_ok!(
			Items::mint_item(Origin::signed(ALICE), CLASS_ID, vec![1], ItemTokenData {
				name: b"name".to_vec(),
				price: 100,
				zone_id: 1,
				seat_id: Some(1),
				status: ItemStatus::Unchecked,

				created_at: Some(<frame_system::Pallet<Runtime>>::block_number()),
				inspected_at: None,
				inspected_with: None,
			})
		);
		assert_eq!(NonFungibleTokenModule::next_token_id(CLASS_ID), 2);

		let next_class_id = NonFungibleTokenModule::next_class_id();
		assert_ok!(
			NonFungibleTokenModule::create_class(&ALICE, vec![1], ItemClassData {
				item_type: ItemType::OfflineEvent,
				info: b"info".to_vec(),
				uri: b"https://fantour.io".to_vec(),
				poster: b"https://fantour.io".to_vec(),

				start_time: 0,
				end_time: 0,
				start_sale_time: 0,
				end_sale_time: 0,

				inspector: ALICE,

				created_at: Some(<frame_system::Pallet<Runtime>>::block_number()),
			})
		);
		assert_eq!(NonFungibleTokenModule::next_token_id(next_class_id), 0);
		assert_ok!(
			Items::mint_item(Origin::signed(ALICE), next_class_id, vec![1], ItemTokenData {
				name: b"name".to_vec(),
				price: 100,
				zone_id: 1,
				seat_id: Some(1),
				status: ItemStatus::Unchecked,

				created_at: Some(<frame_system::Pallet<Runtime>>::block_number()),
				inspected_at: None,
				inspected_with: None,
			})
		);
		assert_eq!(NonFungibleTokenModule::next_token_id(next_class_id), 1);
		assert_ok!(
			Items::mint_item(Origin::signed(ALICE), next_class_id, vec![1], ItemTokenData {
				name: b"name".to_vec(),
				price: 100,
				zone_id: 1,
				seat_id: Some(1),
				status: ItemStatus::Unchecked,

				created_at: Some(<frame_system::Pallet<Runtime>>::block_number()),
				inspected_at: None,
				inspected_with: None,
			})
		);

		assert_eq!(NonFungibleTokenModule::next_token_id(CLASS_ID), 2);
	});
}

#[test]
fn mint_item_should_fail() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(
			NonFungibleTokenModule::create_class(&BOB, vec![1], ItemClassData {
				item_type: ItemType::OfflineEvent,
				info: b"info".to_vec(),
				uri: b"https://fantour.io".to_vec(),
				poster: b"https://fantour.io".to_vec(),

				start_time: 0,
				end_time: 0,
				start_sale_time: 0,
				end_sale_time: 0,

				inspector: ALICE,

				created_at: Some(<frame_system::Pallet<Runtime>>::block_number()),
			})
		);
		Classes::<Runtime>::mutate(CLASS_ID, |class_info| {
			class_info.as_mut().unwrap().total_issuance = <Runtime as orml_nft::Config>::TokenId::max_value();
		});
		assert_noop!(
			Items::mint_item(Origin::signed(BOB), CLASS_ID, vec![1], ItemTokenData {
				name: b"name".to_vec(),
				price: 100,
				zone_id: 1,
				seat_id: Some(1),
				status: ItemStatus::Unchecked,

				created_at: Some(<frame_system::Pallet<Runtime>>::block_number()),
				inspected_at: None,
				inspected_with: None,
			}),
			Error::<Runtime>::CannotMintItem,
		);

		NextTokenId::<Runtime>::mutate(CLASS_ID, |id| *id = <Runtime as orml_nft::Config>::TokenId::max_value());
		assert_noop!(
			Items::mint_item(Origin::signed(BOB), CLASS_ID, vec![1], ItemTokenData {
				name: b"name".to_vec(),
				price: 100,
				zone_id: 1,
				seat_id: Some(1),
				status: ItemStatus::Unchecked,

				created_at: Some(<frame_system::Pallet<Runtime>>::block_number()),
				inspected_at: None,
				inspected_with: None,
			}),
			Error::<Runtime>::CannotMintItem,
		);
	});
}

#[test]
fn transfer_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(
			NonFungibleTokenModule::create_class(&BOB, vec![1], ItemClassData {
				item_type: ItemType::OfflineEvent,
				info: b"info".to_vec(),
				uri: b"https://fantour.io".to_vec(),
				poster: b"https://fantour.io".to_vec(),

				start_time: 0,
				end_time: 0,
				start_sale_time: 0,
				end_sale_time: 0,

				inspector: ALICE,

				created_at: Some(<frame_system::Pallet<Runtime>>::block_number()),
			})
		);
		assert_ok!(
			Items::mint_item(Origin::signed(BOB), CLASS_ID, vec![1], ItemTokenData {
				name: b"name".to_vec(),
				price: 100,
				zone_id: 1,
				seat_id: Some(1),
				status: ItemStatus::Unchecked,

				created_at: Some(<frame_system::Pallet<Runtime>>::block_number()),
				inspected_at: None,
				inspected_with: None,
			})
		);
        assert!(NonFungibleTokenModule::is_owner(&BOB, (CLASS_ID, TOKEN_ID)));
		assert_ok!(Items::transfer_item(Origin::signed(BOB), BOB, (CLASS_ID, TOKEN_ID)));
		assert_ok!(Items::transfer_item(Origin::signed(BOB), ALICE, (CLASS_ID, TOKEN_ID)));
		assert_ok!(Items::transfer_item(Origin::signed(ALICE), BOB, (CLASS_ID, TOKEN_ID)));
		assert!(NonFungibleTokenModule::is_owner(&BOB, (CLASS_ID, TOKEN_ID)));
	});
}

#[test]
fn transfer_should_fail() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(
			NonFungibleTokenModule::create_class(&BOB, vec![1], ItemClassData {
				item_type: ItemType::OfflineEvent,
				info: b"info".to_vec(),
				uri: b"https://fantour.io".to_vec(),
				poster: b"https://fantour.io".to_vec(),

				start_time: 0,
				end_time: 0,
				start_sale_time: 0,
				end_sale_time: 0,

				inspector: ALICE,

				created_at: Some(<frame_system::Pallet<Runtime>>::block_number()),
			})
		);
		assert_ok!(
			Items::mint_item(Origin::signed(BOB), CLASS_ID, vec![1], ItemTokenData {
				name: b"name".to_vec(),
				price: 100,
				zone_id: 1,
				seat_id: Some(1),
				status: ItemStatus::Unchecked,

				created_at: Some(<frame_system::Pallet<Runtime>>::block_number()),
				inspected_at: None,
				inspected_with: None,
			})
		);
		assert_noop!(
			Items::transfer_item(Origin::signed(BOB), ALICE, (CLASS_ID, TOKEN_ID_NOT_EXIST)),
			Error::<Runtime>::NotItemOwner
		);
		assert_noop!(
			Items::transfer_item(Origin::signed(ALICE), BOB, (CLASS_ID, TOKEN_ID)),
			Error::<Runtime>::NotItemOwner
		);
		assert_noop!(
			Items::mint_item(Origin::signed(BOB), CLASS_ID_NOT_EXIST, vec![1], ItemTokenData {
				name: b"name".to_vec(),
				price: 100,
				zone_id: 1,
				seat_id: Some(1),
				status: ItemStatus::Unchecked,

				created_at: Some(<frame_system::Pallet<Runtime>>::block_number()),
				inspected_at: None,
				inspected_with: None,
			}),
			Error::<Runtime>::ClassNotExists
		);
		assert_noop!(
			Items::transfer_item(Origin::signed(ALICE), ALICE, (CLASS_ID, TOKEN_ID)),
			Error::<Runtime>::NotItemOwner
		);
	});
}
