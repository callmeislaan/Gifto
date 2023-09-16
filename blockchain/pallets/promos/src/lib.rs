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
    use frame_support::traits::Time;

    pub type PromoValue<T> = BoundedVec<u32, <T as Config>::PromoValueLimit>;
    pub type PromoSymbol<T> = BoundedVec<u8, <T as Config>::PromoSymbolLimit>;
    pub type Image<T> = BoundedVec<u8, <T as Config>::ImageLimit>;
    pub type Description<T> = BoundedVec<u8, <T as Config>::DescriptionLimit>;
    pub type SystemTime<T> = <<T as Config>::Moment as frame_support::traits::Time>::Moment;
    pub type PromoCondition<T> = BoundedVec<u8, <T as Config>::PromoValueLimit>;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_brands::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type PromoLimit: Get<u32>;
        type PromoSymbolLimit: Get<u32>;
        type PromoValueLimit: Get<u32>;
        type ImageLimit: Get<u32>;
        type DescriptionLimit: Get<u32>;
        type Moment: Time;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn promo_count)]
    pub type PromoCount<T> = StorageValue<_, u32, ValueQuery, >;

    #[pallet::storage]
    #[pallet::getter(fn promos)]
    pub type Promos<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Promo<T>, OptionQuery, >;

    #[pallet::storage]
    #[pallet::getter(fn symbol_promos)]
    pub type SymbolPromos<T: Config> = StorageMap<_, Blake2_128Concat, PromoSymbol<T>, T::Hash, OptionQuery, >;

    #[pallet::storage]
    #[pallet::getter(fn promo_owners)]
    pub type PromoOwners<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::Hash, PromoValue<T>, ValueQuery, >;
    
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        PromoCreated(T::Hash, PromoSymbol<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        PromoSymbolExisted,
        PromoNumberLimited,
        NoBrandFound,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn create_new_promo(origin: OriginFor<T>, symbol: Vec<u8>, brand_hash: T::Hash, 
                name: Vec<u8>, avatar: Vec<u8>, description: Vec<u8>, start_date: SystemTime<T>, end_date: SystemTime<T>, maximum_quantity: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let bounded_symbol: PromoSymbol<T> = symbol.clone().try_into().expect("symbol is too long");
            let bounded_avatar: Image<T> = avatar.clone().try_into().expect("avatar is too long");  
            let bounded_description: Description<T> = description.clone().try_into().expect("avatar is too long"); 

            // ensure owner has brand
            ensure!(Self::is_branch_hash_beyond_owner(brand_hash.clone(), who.clone()), <Error<T>>::NoBrandFound);

            // ensure promo symbol not duplicate
            ensure!(Self::symbol_promos(bounded_symbol.clone()) == None, <Error<T>>::PromoSymbolExisted);

            let promo = Promo::new(brand_hash.clone(), bounded_symbol.clone(),
                bounded_avatar.clone(), bounded_description.clone(), start_date.clone(), end_date.clone(), maximum_quantity);
            
            let promo_hash = T::Hashing::hash_of(&promo);

            <Promos<T>>::insert(promo_hash.clone(), promo.clone());

            // storage promo symbols
            <SymbolPromos<T>>::insert(bounded_symbol.clone(), promo_hash.clone());

            // increase and storage promo count
            let next_promo_count = Self::promo_count().checked_add(1_u32).ok_or(<Error<T>>::PromoNumberLimited)?;

            <PromoCount<T>>::put(next_promo_count);

            Self::deposit_event(Event::PromoCreated(promo_hash, bounded_symbol));
            
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn send_promo(origin: OriginFor<T>, receiver: T::AccountId, value: PromoValue<T>) -> DispatchResult {
            // TODO send promo
            
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

    }
}

