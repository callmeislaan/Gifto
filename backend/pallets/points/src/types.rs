use crate::Config;
use codec::*;
use scale_info::TypeInfo;
use frame_support::BoundedVec;

pub type PointSymbol<T> = BoundedVec<u8, <T as Config>::PointSymbolLimit>;
pub type Image<T> = BoundedVec<u8, <T as Config>::ImageLimit>;

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Point<T: Config> {
	brand_hash: T::Hash, 
	point_symbol: PointSymbol<T>,
	avatar: Image<T>,
}

impl<T: Config> Point<T> {

	pub fn new(brand_hash: T::Hash, point_symbol: PointSymbol<T>, avatar: Image<T>) -> Self {
		Point { brand_hash, point_symbol, avatar }
	}

	pub fn brand_hash(&self) -> T::Hash {
		self.brand_hash.clone()
	}

	pub fn point_symbol(&self) -> PointSymbol<T> {
		self.point_symbol.clone()
	}

	pub fn avatar(&self) -> Image<T> {
		self.avatar.clone()
	}

	pub fn set_avatar(&mut self, avatar: Image<T>) {
		self.avatar = avatar;
	}

}
