use crate::Config;
use codec::*;
use frame_support::BoundedVec;
use scale_info::TypeInfo;

pub type EcosystemSymbolType<T> = BoundedVec<u8, <T as Config>::EcosystemSymbolLimit>;

pub type EcosystemNameType<T> = BoundedVec<u8, <T as Config>::EcosystemNameLimit>;

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Ecosystem<T: Config> {
	symbol: EcosystemSymbolType<T>,
	name: EcosystemNameType<T>,
	owner: T::AccountId,
}

impl<T: Config> Ecosystem<T> {
	pub fn new(symbol: EcosystemSymbolType<T>, name: EcosystemNameType<T>, owner: T::AccountId) -> Self {
		Ecosystem { symbol, name, owner }
	}

	pub fn symbol(&self) -> EcosystemSymbolType<T> {
		self.symbol.clone()
	}

	pub fn name(&self) -> EcosystemNameType<T> {
		self.name.clone()
	}

	pub fn owner(&self) -> T::AccountId {
		self.owner.clone()
	}
}
