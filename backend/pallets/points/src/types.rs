use crate::*;
use codec::*;
use scale_info::TypeInfo;

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Point<T: Config> {
	brand_hash: T::Hash, 
	point_symbol: PointSymbol<T>,
	avatar: Image<T>,
	description: Description<T>,
}

impl<T: Config> Point<T> {

	pub fn new(brand_hash: T::Hash, point_symbol: PointSymbol<T>, avatar: Image<T>, description: Description<T>) -> Self {
		Point { brand_hash, point_symbol, avatar , description}
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

	pub fn description(&self) -> Description<T> {
		self.description.clone()
	}

	pub fn set_avatar(&mut self, avatar: Image<T>) {
		self.avatar = avatar;
	}

}
