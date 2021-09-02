#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use codec::{Decode, Encode};
    #[cfg(feature = "std")]
    use serde::{Deserialize, Serialize};

    use frame_support::{
		pallet_prelude::*,
		inherent::Vec
	};
	use frame_system::pallet_prelude::*;

    type AccountOf<T> = <T as frame_system::Config>::AccountId;
    type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;

	pub type ClassIdOf<T> = <T as orml_nft::Config>::ClassId;
	// pub type ClassDataOf<T> = <T as orml_nft::Config>::ClassData;

	pub type TokenIdOf<T> = <T as orml_nft::Config>::TokenId;
	// pub type TokenDataOf<T> = <T as orml_nft::Config>::TokenData;

    #[pallet::config]
	pub trait Config: frame_system::Config + 
        orml_nft::Config<
            ClassData = ItemClassData<BlockNumberOf<Self>>,
            TokenData = ItemTokenData<AccountOf<Self>, BlockNumberOf<Self>>
        >
    {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

    #[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

    #[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub enum ItemType {
        OfflineEvent,
        OnlineEvent,
    }

    #[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, PartialOrd, Ord)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub struct ItemClassData<BlockNumber> {
        pub item_type: ItemType,
        pub info: Vec<u8>,
        pub uri: Vec<u8>,
        pub poster: Vec<u8>,

        pub start_time: u64,
        pub end_time: u64,
        pub start_sale_time: u64,
        pub end_sale_time: u64,

        pub created_at: BlockNumber,
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
    pub struct ItemTokenData<AccountId, BlockNumber> {
        pub name: Vec<u8>,
        pub price: u128,
        pub zone_id: u64,
        pub seat_id: Option<u64>,
        pub status: ItemStatus,

        pub created_at: BlockNumber,
        pub inspected_at: Option<BlockNumber>,
        pub inspected_with: Option<AccountId>,
    }

    #[pallet::storage]
    #[pallet::getter(fn inspectors)]
    pub(super) type Inspectors<T> = StorageMap<_, Blake2_128Concat, ClassIdOf<T>, Vec<AccountOf<T>>>;

    #[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        /// \[who, class_id, metadata\]
        ClassCreated(T::AccountId, T::ClassId, Vec<u8>),
        /// \[who, class_id\]
        ClassDestroyed(T::AccountId, T::ClassId),
        /// \[who, (class_id, token_id), metadata\]
		ItemMinted(T::AccountId, (T::ClassId, T::TokenId), Vec<u8>),
        /// \[who, to, (class_id, token_id)\]
		ItemTransferred(T::AccountId, T::AccountId, (T::ClassId, T::TokenId)),
        /// \[who, (class_id, token_id), status\]
        ItemStatusChanged(T::AccountId, (T::ClassId, T::TokenId), ItemStatus),
        /// \[who, class_id, inspector\]
        ClassInspectorAdded(T::AccountId, T::ClassId, T::AccountId),
        /// \[who, class_id, inspector\]
        ClassInspectorRemoved(T::AccountId, T::ClassId, T::AccountId),
	}

    #[pallet::error]
	pub enum Error<T> {
        /// Cannot mint item
		CannotMintItem,
        /// No permission
        NoPermission,
        /// Not item class owner, no permission
		NotClassOwner,
        /// Not item nft owner, no permission
		NotItemOwner,
        /// Not item class inspector, no permission
        NotInspector,
        /// TokenId not found
        TokenNotExists,
        /// ClassId not found
		ClassNotExists,
        /// Can not create class
        CannotCreateClass,
        /// Can not destroy class
		/// Total issuance is not 0
        CannotDestroyClass,
        /// Inspector already exist
        CannotAddInspector,
        /// Inspector not found
        CannotRemoveInspector,
	}

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(10_000)]
        pub fn create_item_class(
            origin: OriginFor<T>,
            metadata: Vec<u8>,
            item_type: ItemType,
            info: Vec<u8>,
            uri: Vec<u8>,
            poster: Vec<u8>,

            start_time: u64,
            end_time: u64,
            start_sale_time: u64,
            end_sale_time: u64,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let next_class_id = orml_nft::Pallet::<T>::next_class_id();

            let data = ItemClassData {
                item_type, info, uri, poster,
                start_time, end_time, start_sale_time, end_sale_time,
                created_at: <frame_system::Pallet<T>>::block_number(),
            };
            orml_nft::Pallet::<T>::create_class(&who, metadata.clone(), data)
                .map_err(|_| Error::<T>::CannotCreateClass)?;

            <Inspectors<T>>::insert(next_class_id, Vec::<AccountOf<T>>::new());

            Self::deposit_event(Event::ClassCreated(who, next_class_id, metadata));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn mint_item(
            origin: OriginFor<T>,
            cid: ClassIdOf<T>,
            metadata: Vec<u8>,
            name: Vec<u8>,
            price: u128,
            zone_id: u64,
            seat_id: Option<u64>,
        ) -> DispatchResult {
            // To get the user
            let who = ensure_signed(origin)?;

            // CHECK: ensure this is called from the class owner
            let class = orml_nft::Pallet::<T>::classes(cid)
                .ok_or(Error::<T>::ClassNotExists)?;

            if class.owner != who {
                return Err(Error::<T>::NotClassOwner)?;
            }

            // EXECUTE
            let data = ItemTokenData {
                name, price, zone_id, seat_id,
                status: ItemStatus::Unchecked,
                created_at: <frame_system::Pallet<T>>::block_number(),
                inspected_at: None,
                inspected_with: None,
            };
            let tid = orml_nft::Pallet::<T>::mint(&who, cid, metadata.clone(), data)
                .map_err(|_| Error::<T>::CannotMintItem)?;

            Self::deposit_event(Event::ItemMinted(who, (cid, tid), metadata));
            
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn destroy_item_class(
            origin:     OriginFor<T>,
            cid:        ClassIdOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // CHECK: ensure this is called from the class owner
            let class = orml_nft::Pallet::<T>::classes(cid)
                .ok_or(Error::<T>::ClassNotExists)?;

            if class.owner != who {
                return Err(Error::<T>::NotClassOwner)?;
            }

            orml_nft::Pallet::<T>::destroy_class(&who, cid)
                .map_err(|_| Error::<T>::CannotDestroyClass)?;

            Self::deposit_event(Event::ClassDestroyed(who, cid));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn transfer_item(
            origin:     OriginFor<T>,
            to:         T::AccountId,
            token:      (ClassIdOf<T>, TokenIdOf<T>),
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // check: if the origin is the owner of the token, implicitly checking the existence of the
			//   class and token ID
            if !orml_nft::Pallet::<T>::is_owner(&who, token.clone()) {
                return Err(Error::<T>::NotItemOwner)?;
            }

            // execute: actualize the transfer
			orml_nft::Pallet::<T>::transfer(&who, &to, token.clone())?;
			
            Self::deposit_event(Event::ItemTransferred(who, to, token));
			
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn set_item_status(
            origin:     OriginFor<T>,
            token:      (ClassIdOf<T>, TokenIdOf<T>),
            status:     ItemStatus,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            orml_nft::Tokens::<T>::try_mutate(token.0, token.1, |token_info| -> DispatchResult {
                let t = token_info.as_mut().ok_or(Error::<T>::TokenNotExists)?;
                // let c = orml_nft::Classes::<T>::try_get(token.0)
                //     .map_err(|_| Error::<T>::ClassNotExists)?;
                ensure!(Self::is_authorized(token.0, &who), Error::<T>::NoPermission);

                if t.data.status == ItemStatus::Checked || t.data.status == ItemStatus::Refund {
                    return Err(Error::<T>::NoPermission)?;
                }

                if status == ItemStatus::Unchecked {
                    return Ok(());
                }

                t.data.status = status;
                t.data.inspected_at = Some(<frame_system::Pallet<T>>::block_number());
                t.data.inspected_with = Some(who.clone());

                Self::deposit_event(Event::ItemStatusChanged(who, token, status));

                Ok(())
            })
        }

        #[pallet::weight(10_000)]
        pub fn add_class_inspector(
            origin:     OriginFor<T>,
            cid:        ClassIdOf<T>,
            inspector:  T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            orml_nft::Classes::<T>::try_mutate(cid, |class_info| -> DispatchResult {
                let c = class_info.as_mut().ok_or(Error::<T>::ClassNotExists)?;
                ensure!(who == c.owner, Error::<T>::NotClassOwner);

                <Inspectors<T>>::try_mutate(cid, |inspector_info| -> DispatchResult {
                    let i = inspector_info.as_mut().ok_or(Error::<T>::CannotAddInspector)?;
                    
                    if i.contains(&inspector) {
                        return Err(Error::<T>::CannotAddInspector)?;
                    }
    
                    i.push(inspector.clone());

                    Self::deposit_event(Event::ClassInspectorAdded(who, cid, inspector));
    
                    Ok(())
                })
            })
        }

        #[pallet::weight(10_000)]
        pub fn remove_class_inspector(
            origin:     OriginFor<T>,
            cid:        ClassIdOf<T>,
            inspector:  T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            <Inspectors<T>>::try_mutate(cid, |inspector_info| -> DispatchResult {
                match orml_nft::Classes::<T>::get(cid) {
                    Some(c) => { ensure!(who == c.owner, Error::<T>::NotClassOwner); },
                    None => { return Err(Error::<T>::ClassNotExists)?; }
                };

                let i = inspector_info.as_mut().ok_or(Error::<T>::CannotRemoveInspector)?;
                if i.is_empty() || !i.contains(&inspector) {
                    return Err(Error::<T>::CannotRemoveInspector)?;
                }

                let idx = i
                    .iter()
                    .position(|k| k == &inspector)
                    .unwrap();
                i.remove(idx);

                Self::deposit_event(Event::ClassInspectorRemoved(who, cid, inspector));

                Ok(())
            })
        }
    }
}

impl<T: Config> Pallet<T> {
    pub fn is_authorized(
        cid: ClassIdOf<T>,
        of: &T::AccountId,
    ) -> bool {
        let mut flag = false;
        
        match orml_nft::Classes::<T>::try_get(cid) {
            Ok(c) => {
                if *of == c.owner { flag = true; }
            },
            Err(_) => {},
        }
        match <Inspectors<T>>::try_get(cid) {
            Ok(i) => {
                if i.iter().any(|k| k == of) {
                    flag = true;
                }
            },
            Err(_) => {},
        }

        flag
    }
}
