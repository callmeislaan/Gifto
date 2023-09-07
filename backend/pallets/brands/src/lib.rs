#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod types;

#[frame_support::pallet]
pub mod pallet {

    pub use crate::types::*;
    use frame_support::{pallet_prelude::{*, OptionQuery, ValueQuery}, Blake2_128Concat, ensure};
    use frame_system::{pallet_prelude::*};
    use frame_support::sp_runtime::traits::Hash;
	use frame_support::dispatch::Vec;

    pub type BrandSymbolType<T> = BoundedVec<u8, <T as Config>::BrandSymbolLimit>;
    pub type BrandNameType<T> = BoundedVec<u8, <T as Config>::BrandNameLimit>;
    pub type Image<T> = BoundedVec<u8, <T as Config>::ImageLimit>;    

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type BrandSymbolLimit: Get<u32>;
        type BrandNameLimit: Get<u32>;
        type BrandLimit: Get<u32>;
        type ImageLimit: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn brand_count)]
    pub type BrandCount<T> = StorageValue<_, u32, ValueQuery, >;

    #[pallet::storage]
    #[pallet::getter(fn brands)]
    pub type Brands<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Brand<T>, OptionQuery, >;

    #[pallet::storage]
    #[pallet::getter(fn symbol_brands)]
    pub type SymbolBrands<T: Config> = StorageMap<_, Blake2_128Concat, BrandSymbolType<T>, T::Hash, OptionQuery, >;

    #[pallet::storage]
    #[pallet::getter(fn brand_owner)]
    pub type BrandOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<T::Hash, T::BrandLimit>, OptionQuery, >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        BrandCreated(T::Hash, BrandSymbolType<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        BrandSymbolExisted,
        AppNumberLimited,
        BrandOwnerLimited,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn create_new_brand(origin: OriginFor<T>, symbol: Vec<u8>, name: Vec<u8>, avatar: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            
            let bounded_symbol: BrandSymbolType<T> = symbol.clone().try_into().expect("symbol is too long");
            let bounded_name: BrandNameType<T> = name.clone().try_into().expect("name is too long");
            let bounded_avatar: Image<T> = avatar.clone().try_into().expect("avatar is too long");  

            // ensure brand symbol not duplicate
            ensure!(Self::symbol_brands(bounded_symbol.clone()) == None, <Error<T>>::BrandSymbolExisted);

            let brand = Brand::new(bounded_symbol.clone(), bounded_name.clone(), who.clone(), bounded_avatar.clone());
            
            let brand_hash = T::Hashing::hash_of(&brand);

            // storage brand
            <Brands<T>>::insert(brand_hash.clone(), brand.clone());

            <SymbolBrands<T>>::insert(bounded_symbol.clone(), brand_hash.clone());

            // increase and storage brand count
            let next_brand_count = Self::brand_count().checked_add(1_u32).ok_or(<Error<T>>::AppNumberLimited)?;

            <BrandOwner<T>>::try_append(who.clone(), brand_hash.clone()).map_err(|_| <Error<T>>::BrandOwnerLimited)?;

            <BrandCount<T>>::put(next_brand_count);

            Self::deposit_event(Event::BrandCreated(brand_hash, bounded_symbol));
            
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(5_000)]
        pub fn update_brand_info(origin: OriginFor<T>, symbol: Vec<u8>, name: Vec<u8>, avatar: Vec<u8>) -> DispatchResult {

            Ok(())
        }

    }
}

