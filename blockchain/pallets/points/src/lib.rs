#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod types;

#[frame_support::pallet]
pub mod pallet {

    pub use crate::types::*;
    use frame_support::{Blake2_128Concat, ensure};
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::{*, OriginFor};
    use frame_support::sp_runtime::traits::Hash;
	use frame_support::dispatch::Vec;

    pub type PointValue<T> = BoundedVec<u32, <T as Config>::PointValueLimit>;
    pub type PointSymbol<T> = BoundedVec<u8, <T as Config>::PointSymbolLimit>;
    pub type Image<T> = BoundedVec<u8, <T as Config>::ImageLimit>;
    pub type Description<T> = BoundedVec<u8, <T as Config>::DescriptionLimit>;
    pub type PointManager<T> = BoundedVec<<T as frame_system::Config>::AccountId, <T as Config>::PointManagerLimit>;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_brands::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type PointLimit: Get<u32>;
        type PointSymbolLimit: Get<u32>;
        type PointValueLimit: Get<u32>;
        type ImageLimit: Get<u32>;
        type DescriptionLimit: Get<u32>;
        type PointManagerLimit: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn point_count)]
    pub type PointCount<T> = StorageValue<_, u32, ValueQuery, >;

    #[pallet::storage]
    #[pallet::getter(fn points)]
    pub type Points<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Point<T>, OptionQuery, >;

    #[pallet::storage]
    #[pallet::getter(fn symbol_points)]
    pub type SymbolPoints<T: Config> = StorageMap<_, Blake2_128Concat, PointSymbol<T>, T::Hash, OptionQuery, >;

    #[pallet::storage]
    #[pallet::getter(fn point_owners)]
    pub type PointOwners<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::Hash, u32, ValueQuery, >;
    
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        PointCreated(T::Hash, PointSymbol<T>),
        PointExchange(T::Hash, T::AccountId, u32),

    }

    #[pallet::error]
    pub enum Error<T> {
        PointSymbolExisted,
        PointNumberLimited,
        NoBrandFound,
        PointNotFound,
        ActionDenined,
        PointOwnerNumberLimited,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn create_new_point(origin: OriginFor<T>, symbol: Vec<u8>, brand_hash: T::Hash, name: Vec<u8>, avatar: Vec<u8>, description: Vec<u8>, managers: Vec<T::AccountId>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // let brand = pallet_brands::Brands::get(branch_hash).ok_or(<Error<T>>::NoBrandFound)?;

            let bounded_symbol: PointSymbol<T> = symbol.clone().try_into().expect("symbol is too long");
            let bounded_avatar: Image<T> = avatar.clone().try_into().expect("avatar is too long");  
            let bounded_description: Description<T> = description.clone().try_into().expect("avatar is too long");  
            let bounded_manager: PointManager<T> = managers.clone().try_into().expect("managers is too much");

            // ensure owner has brand
            ensure!(Self::is_branch_hash_beyond_owner(brand_hash.clone(), who.clone()), <Error<T>>::NoBrandFound);

            // ensure point symbol not duplicate
            ensure!(Self::symbol_points(bounded_symbol.clone()) == None, <Error<T>>::PointSymbolExisted);

            let point = Point::new(brand_hash.clone(), bounded_symbol.clone(), bounded_avatar.clone(), bounded_description.clone(), bounded_manager.clone());
            
            let point_hash = T::Hashing::hash_of(&point);

            <Points<T>>::insert(point_hash.clone(), point.clone());

            // storage point symbols
            <SymbolPoints<T>>::insert(bounded_symbol.clone(), point_hash.clone());

            // increase and storage point count
            let next_point_count = Self::point_count().checked_add(1_u32).ok_or(<Error<T>>::PointNumberLimited)?;

            <PointCount<T>>::put(next_point_count);

            Self::deposit_event(Event::PointCreated(point_hash, bounded_symbol));
            
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn manager_send_point(origin: OriginFor<T>, receiver: T::AccountId, point_hash: T::Hash, value: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let point = Self::points(&point_hash).ok_or(<Error<T>>::PointNotFound)?;

            Self::is_account_has_permission_with_point(&who, &point)?;

            let next_value = Self::point_owners(&receiver, &point_hash).checked_add(value).ok_or(<Error<T>>::PointOwnerNumberLimited)?;


            <PointOwners<T>>::insert(&receiver, &point_hash, next_value);

            Self::deposit_event(Event::PointExchange(point_hash, receiver, value));

            Ok(())
        }

    }

    impl<T: Config> Pallet<T> {
        pub fn is_branch_hash_beyond_owner(brand_hash: T::Hash, owner: T::AccountId) -> bool{
            if let Some(brand) = pallet_brands::Pallet::<T>::brands(brand_hash) {
                if brand.owner() == owner {
                    return true;
                }
            }
            false
        }

        pub fn is_account_has_permission_with_point(account: &T::AccountId, point: &Point<T>) -> Result<(), Error<T>> {
            if !point.manager().contains(account) {
                return Err(<Error<T>>::ActionDenined);
            }
            Ok(())
        }

    }
}

