#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod types;

#[frame_support::pallet]
pub mod pallet {

    pub use crate::types::*;
    use frame_support::{pallet_prelude::{*, OptionQuery}, Blake2_128Concat, ensure};
    use frame_system::pallet_prelude::*;
    use frame_support::sp_runtime::traits::Hash;
	use frame_support::dispatch::Vec;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type EcosystemSymbolLimit: Get<u32>;

        type EcosystemNameLimit: Get<u32>;

        type EcosystemLimit: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn ecosystem_count)]
    pub type EcosystemCount<T> = StorageValue<_, u32, ValueQuery, >;

    #[pallet::storage]
    #[pallet::getter(fn ecosystems)]
    pub type Ecosystems<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Ecosystem<T>, OptionQuery, >;

    #[pallet::storage]
    #[pallet::getter(fn ecosystem_symbols)]
    pub type EcosystemSymbols<T: Config> = StorageValue<_, BoundedVec<EcosystemSymbolType<T>, T::EcosystemLimit>, ValueQuery, >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        EcosystemCreated(T::Hash, EcosystemSymbolType<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        EcosystemSymbolExisted,
        AppNumberLimited,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn create_new_ecosystem(origin: OriginFor<T>, symbol: Vec<u8>, name: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            
            let bounded_symbol: EcosystemSymbolType<T> = symbol.clone().try_into().expect("symbol is too long");
            let bounded_name: EcosystemNameType<T> = name.clone().try_into().expect("name is too long");

            // ensure ecosystem symbol not duplicate
            let ecosystem_symbols = <EcosystemSymbols<T>>::get();
            ensure!(ecosystem_symbols.contains(&bounded_symbol) == false, <Error<T>>::EcosystemSymbolExisted);

            let ecosystem = Ecosystem::new(bounded_symbol.clone(), bounded_name.clone(), who.clone());
            
            let ecosystem_hash = T::Hashing::hash_of(&ecosystem);

            // storage ecosystem
            <Ecosystems<T>>::insert(ecosystem_hash.clone(), ecosystem.clone());

            // storage ecosystem symbols
            <EcosystemSymbols<T>>::mutate(|symbol_vec| {
                symbol_vec.try_push(bounded_symbol.clone())
            }).map_err(|_| <Error<T>>::AppNumberLimited)?;

            // increase and storage ecosystem count
            let next_ecosystem_count = Self::ecosystem_count().checked_add(1_u32).ok_or(<Error<T>>::AppNumberLimited)?;

            <EcosystemCount<T>>::put(next_ecosystem_count);

            Self::deposit_event(Event::EcosystemCreated(ecosystem_hash, bounded_symbol));
            
            Ok(())
        }

    }
}

