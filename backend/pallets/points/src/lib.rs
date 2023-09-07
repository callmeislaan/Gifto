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

    type PointValue<T: Config> = BoundedVec<u32, <T as Config>::PointValueLimit>;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type PointLimit: Get<u32>;
        type PointSymbolLimit: Get<u32>;
        type PointValueLimit: Get<u32>;
        type ImageLimit: Get<u32>;
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
    #[pallet::getter(fn point_symbols)]
    pub type PointSymbols<T: Config> = StorageValue<_, PointSymbol<T>, ValueQuery, >;

    #[pallet::storage]
    #[pallet::getter(fn point_owner)]
    pub type PointOwner<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::Hash, PointValue<T>, ValueQuery, >;
    
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        PointCreated(T::Hash, PointSymbol<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        PointSymbolExisted,
        AppNumberLimited,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn create_new_point(origin: OriginFor<T>, symbol: Vec<u8>, branch_hash: T::Hash, name: Vec<u8>, avatar: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // pallet_brands::Brands::get(who.clone());

            // ensure!(pallet_brands::Brands::contains_key(who.clone()), "must have a brand to create new point");

            // let bounded_symbol: PointSymbol<T> = symbol.clone().try_into().expect("symbol is too long");
            // let bounded_avatar: Image<T> = avatar.clone().try_into().expect("avatar is too long");  

            // // ensure point symbol not duplicate
            // let point_symbols = <PointSymbols<T>>::get();
            // ensure!(point_symbols.contains(&bounded_symbol) == false, <Error<T>>::PointSymbolExisted);

            // let point = Point::new(bounded_symbol.clone(), who.clone(), bounded_avatar.clone());
            
            // let point_hash = T::Hashing::hash_of(&point);

            // storage point
            // <Points<T>>::insert(point_hash.clone(), point.clone());

            // // storage point symbols
            // <PointSymbols<T>>::mutate(|symbol_vec| {
            //     symbol_vec.try_push(bounded_symbol.clone())
            // }).map_err(|_| <Error<T>>::AppNumberLimited)?;

            // // increase and storage point count
            // let next_point_count = Self::point_count().checked_add(1_u32).ok_or(<Error<T>>::AppNumberLimited)?;

            // <PointCount<T>>::put(next_point_count);

            // Self::deposit_event(Event::PointCreated(point_hash, bounded_symbol));
            
            Ok(())
        }


        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn send_point(origin: OriginFor<T>, receiver: T::AccountId, value: PointValue<T>) -> DispatchResult {
            // TODO send point
            Ok(())
        }



    }
}

