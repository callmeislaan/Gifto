use crate::*;
use codec::*;
use frame_support::BoundedVec;
use scale_info::TypeInfo;

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo, PartialEq)]
#[scale_info(skip_type_params(T))]
pub struct Brand<T: Config> {
	symbol: BrandSymbol<T>,
	name: BrandName<T>,
	owner: T::AccountId,
	avatar: Image<T>,
	description: Description<T>,
	domain: Domain<T>,
}

impl<T: Config> Brand<T> {
	pub fn new(symbol: BrandSymbol<T>, name: BrandName<T>, owner: T::AccountId, avatar: Image<T>, 
													description: Description<T>, domain: Domain<T>) -> Self {
		Brand { symbol, name, owner, avatar, description, domain }
	}

	pub fn symbol(&self) -> BrandSymbol<T> {
		self.symbol.clone()
	}

	pub fn name(&self) -> BrandName<T> {
		self.name.clone()
	}

	pub fn owner(&self) -> T::AccountId {
		self.owner.clone()
	}

	pub fn image(&self) -> Image<T> {
		self.avatar.clone()
	}

	pub fn description(&self) -> Description<T> {
		self.description.clone()
	}

	pub fn domain(&self) -> Domain<T> {
		self.domain.clone()
	}
}
