use crate::*;
use codec::*;
use frame_support::BoundedVec;
use scale_info::TypeInfo;

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Brand<T: Config> {
	symbol: BrandSymbolType<T>,
	name: BrandNameType<T>,
	owner: T::AccountId,
	avatar: Image<T>,
}

impl<T: Config> Brand<T> {
	pub fn new(symbol: BrandSymbolType<T>, name: BrandNameType<T>, owner: T::AccountId, avatar: Image<T>) -> Self {
		Brand { symbol, name, owner, avatar }
	}

	pub fn symbol(&self) -> BrandSymbolType<T> {
		self.symbol.clone()
	}

	pub fn name(&self) -> BrandNameType<T> {
		self.name.clone()
	}

	pub fn owner(&self) -> T::AccountId {
		self.owner.clone()
	}

	pub fn image(&self) -> Image<T> {
		self.avatar.clone()
	}
}
