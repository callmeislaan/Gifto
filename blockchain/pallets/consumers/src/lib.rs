#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod types;

#[frame_support::pallet]
pub mod pallet {

    pub use crate::types::*;
    use frame_support::{pallet_prelude::{*, OptionQuery}, Blake2_128Concat};
    use frame_system::{pallet_prelude::*};
	use frame_support::dispatch::Vec;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type ConsumerNameLimit: Get<u32>;
        type ConsumerLimit: Get<u32>;
        type ImageLimit: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn consumer_count)]
    pub type ConsumerCount<T> = StorageValue<_, u32, ValueQuery, >;

    #[pallet::storage]
    #[pallet::getter(fn consumers)]
    pub type Consumers<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Consumer<T>, OptionQuery, >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ConsumerCreated(T::AccountId),
        ConsumerUpdated(T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        ConsumerNumberLimited,
        ConsumerNotFound,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn update_info(origin: OriginFor<T>, name: Vec<u8>, avatar: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            
            let bounded_name: ConsumerNameType<T> = name.clone().try_into().expect("name is too long");
            let bounded_image: Image<T> = avatar.clone().try_into().expect("name is too long");

            Consumers::<T>::try_mutate(&who, |change| {
				if let Some(consumer) = change {
					consumer.set_name(bounded_name);
                    consumer.set_avatar(bounded_image);
					return Ok(());
				}
				Err(())
			}).map_err(|_| Error::<T>::ConsumerNotFound)?;

            Self::deposit_event(Event::ConsumerUpdated(who.clone()));
            
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn create(origin: OriginFor<T>, name: Vec<u8>, avatar: Vec<u8>) -> DispatchResult {

            let who = ensure_signed(origin)?;
            let bounded_name: ConsumerNameType<T> = name.clone().try_into().expect("name is too long");
            let bounded_avatar: Image<T> = avatar.clone().try_into().expect("avatar is too long");
            let new_id = who.clone();


            Ok(())
        }

    }
}

