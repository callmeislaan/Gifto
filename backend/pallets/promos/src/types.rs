use crate::*;
use codec::*;
use scale_info::TypeInfo;

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Promo<T: Config> {
	brand_hash: T::Hash, 
	promo_symbol: PromoSymbol<T>,
	avatar: Image<T>,
	description: Description<T>,
	start_date: SystemTime<T>,
	end_date: SystemTime<T>,
	maximum_quantity: u32,
	
}

// (Tên Promo, Mã Promo, Avatar, Mô tả, Điều kiện, Số lượng tối đa, Ngày bắt đầu, ngày kết thúc)
impl<T: Config> Promo<T> {

	pub fn new(brand_hash: T::Hash, promo_symbol: PromoSymbol<T>, avatar: Image<T>, 
		description: Description<T>, start_date: SystemTime<T>, end_date: SystemTime<T>, maximum_quantity: u32) -> Self {
		Promo { brand_hash, promo_symbol, avatar , description, start_date, end_date, maximum_quantity}
	}

	pub fn brand_hash(&self) -> T::Hash {
		self.brand_hash.clone()
	}

	pub fn promo_symbol(&self) -> PromoSymbol<T> {
		self.promo_symbol.clone()
	}

	pub fn avatar(&self) -> Image<T> {
		self.avatar.clone()
	}

	pub fn description(&self) -> Description<T> {
		self.description.clone()
	}

	pub fn start_date(&self) -> SystemTime<T> {
		self.start_date.clone()
	}

	pub fn end_date(&self) -> SystemTime<T> {
		self.end_date.clone()
	}

	pub fn maximum_quantity(&self) -> u32 {
		self.maximum_quantity.clone()
	}

	pub fn set_avatar(&mut self, avatar: Image<T>) {
		self.avatar = avatar;
	}

}
