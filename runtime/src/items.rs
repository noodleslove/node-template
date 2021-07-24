#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

use crate::AccountId;

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ItemType {
    OfflineEvent,
    OnlineEvent,
}

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ItemStatus {
    Checked,
    Unchecked,
    Refund,
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ItemClassData {
    pub item_type: ItemType,
    pub info: Vec<u8>,
    pub uri: Vec<u8>,
    pub poster: Vec<u8>,

    pub start_time: u64,
    pub end_time: u64,
    pub start_sale_time: u64,
    pub end_sale_time: u64,
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ItemTokenData {
    pub name: Vec<u8>,
    pub price: u128,
    pub zone_id: u64,
    pub seat_id: Option<u64>,
    pub status: ItemStatus,
}

pub const MAX_CLASS_METADATA: u32 = 1024;
pub const MAX_TOKEN_METADATA: u32 = 1024;

// This is returning the testnet genesis config
pub fn items_genesis(
    owner: &AccountId,
) -> Vec<(
    AccountId,
    Vec<u8>,
    ItemClassData,
    Vec<(AccountId, Vec<u8>, ItemTokenData)>,
)> {
    vec![
        (
            owner.clone(),
            b"hongkong_concerts".to_vec(),
            ItemClassData {
                item_type: ItemType::OfflineEvent,
                info: b"Hong Kong Concerts".to_vec(),
                uri: b"https://fantour.io".to_vec(),
                poster: b"https://fantour.io".to_vec(),

                start_time: 1800,
                end_time: 2000,
                start_sale_time: 1200,
                end_sale_time: 1400,
            },
            vec![
                (
                    owner.clone(),
                    b"gin_lee_concert".to_vec(),
                    ItemTokenData {
                        name: b"Gin Lee Concert".to_vec(),
                        price: 5000,
                        zone_id: 1,
                        seat_id: Some(1),
                        status: ItemStatus::Unchecked,
                    },
                ),
                (
                    owner.clone(),
                    b"eason_chan_concert".to_vec(),
                    ItemTokenData {
                        name: b"Eason Chan Concert".to_vec(),
                        price: 8000,
                        zone_id: 1,
                        seat_id: Some(1),
                        status: ItemStatus::Unchecked,
                    },
                ),
            ],
        ),
        (
            owner.clone(),
            b"hongkong_musical".to_vec(),
            ItemClassData {
                item_type: ItemType::OfflineEvent,
                info: b"Hong Kong Musical".to_vec(),
                uri: b"https://fantour.io".to_vec(),
                poster: b"https://fantour.io".to_vec(),

                start_time: 1800,
                end_time: 2000,
                start_sale_time: 1200,
                end_sale_time: 1400,
            },
            vec![],
        ),
    ]
}
