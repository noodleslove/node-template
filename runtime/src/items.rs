#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;

use crate::{AccountId, BlockNumber, Runtime};

pub const MAX_CLASS_METADATA: u32 = 1024;
pub const MAX_TOKEN_METADATA: u32 = 1024;

// This is returning the testnet genesis config
pub fn items_genesis(
    owner: &AccountId,
) -> Vec<(
    AccountId,
    Vec<u8>,
    pallet_items::ItemClassData<AccountId, BlockNumber>,
    Vec<(AccountId, Vec<u8>, pallet_items::ItemTokenData<AccountId, BlockNumber>)>,
)> {
    vec![
        (
            owner.clone(),
            b"hongkong_concerts".to_vec(),
            pallet_items::ItemClassData {
                item_type: pallet_items::ItemType::OfflineEvent,
                info: b"Hong Kong Concerts".to_vec(),
                uri: b"https://fantour.io".to_vec(),
                poster: b"https://fantour.io".to_vec(),

                start_time: 1800,
                end_time: 2000,
                start_sale_time: 1200,
                end_sale_time: 1400,

                inspector: owner.clone(),

                created_at: None,
            },
            vec![
                (
                    owner.clone(),
                    b"gin_lee_concert".to_vec(),
                    pallet_items::ItemTokenData {
                        name: b"Gin Lee Concert".to_vec(),
                        price: 5000,
                        zone_id: 1,
                        seat_id: Some(1),
                        status: pallet_items::ItemStatus::Unchecked,

                        created_at: None,
                        inspected_at: None,
                        inspected_with: None,
                    },
                ),
                (
                    owner.clone(),
                    b"eason_chan_concert".to_vec(),
                    pallet_items::ItemTokenData {
                        name: b"Eason Chan Concert".to_vec(),
                        price: 8000,
                        zone_id: 1,
                        seat_id: Some(1),
                        status: pallet_items::ItemStatus::Unchecked,

                        created_at: None,
                        inspected_at: None,
                        inspected_with: None,
                    },
                ),
            ],
        ),
        (
            owner.clone(),
            b"hongkong_musical".to_vec(),
            pallet_items::ItemClassData {
                item_type: pallet_items::ItemType::OfflineEvent,
                info: b"Hong Kong Musical".to_vec(),
                uri: b"https://fantour.io".to_vec(),
                poster: b"https://fantour.io".to_vec(),

                start_time: 1800,
                end_time: 2000,
                start_sale_time: 1200,
                end_sale_time: 1400,

                inspector: owner.clone(),

                created_at: None,
            },
            vec![],
        ),
    ]
}
