#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

    use frame_support::{Blake2_128Concat, ensure};
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::{*, OriginFor};
    use frame_support::sp_runtime::traits::Hash;
	use frame_support::dispatch::Vec;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_points::Config + pallet_promos::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        PointExchange(T::Hash, T::AccountId, u32),
        PromoExchange(T::Hash, T::AccountId, u32),
        PointCheck(u32),
    }

    #[pallet::error]
    pub enum Error<T> {
        PointOwnerQuantityNotEnough,
        PointOwnerQuantityLimited,
        PromoOwnerQuantityNotEnough,
        PromoOwnerQuantityLimited,
        PointNotFound,
        PromoNotFound,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn send_point(origin: OriginFor<T>, receiver: T::AccountId, point_hash: T::Hash, value: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let point = pallet_points::Pallet::<T>::points(&point_hash).ok_or(<Error<T>>::PointNotFound)?;

            let left_value = pallet_points::Pallet::<T>::point_owners(&who, &point_hash).checked_sub(value).ok_or(<Error<T>>::PointOwnerQuantityNotEnough)?;

            pallet_points::PointOwners::<T>::insert(&who, &point_hash, left_value);

            let next_value = pallet_points::Pallet::<T>::point_owners(&receiver, &point_hash).checked_add(value).ok_or(<Error<T>>::PointOwnerQuantityLimited)?;

            pallet_points::PointOwners::<T>::insert(&receiver, &point_hash, next_value);

            Self::deposit_event(Event::PointExchange(point_hash, receiver, value));

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn send_promo(origin: OriginFor<T>, receiver: T::AccountId, promo_hash: T::Hash, value: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let promo = pallet_promos::Pallet::<T>::promos(&promo_hash).ok_or(<Error<T>>::PromoNotFound)?;

            let left_value = pallet_promos::Pallet::<T>::promo_owners(&who, &promo_hash).checked_sub(value).ok_or(<Error<T>>::PromoOwnerQuantityNotEnough)?;

            pallet_promos::PromoOwners::<T>::insert(&who, &promo_hash, left_value);

            let next_value = pallet_promos::Pallet::<T>::promo_owners(&receiver, &promo_hash).checked_add(value).ok_or(<Error<T>>::PromoOwnerQuantityLimited)?;

            pallet_promos::PromoOwners::<T>::insert(&receiver, &promo_hash, next_value);

            Self::deposit_event(Event::PointExchange(promo_hash, receiver, value));

            Ok(())
        }

    }
}

