use crate::Config;
use codec::*;
use frame_support::BoundedVec;
use scale_info::TypeInfo;

pub type ConsumerNameType<T> = BoundedVec<u8, <T as Config>::ConsumerNameLimit>;
pub type Image<T> = BoundedVec<u8, <T as Config>::ImageLimit>;

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Consumer<T: Config> {
	name: ConsumerNameType<T>,
	id: T::AccountId,
	avatar: Image<T>,
}

impl<T: Config> Consumer<T> {
	pub fn new(name: ConsumerNameType<T>, id: T::AccountId, avatar: Image<T>) -> Self {
		Consumer { name, id, avatar }
	}

	pub fn name(&self) -> ConsumerNameType<T> {
		self.name.clone()
	}

	pub fn id(&self) -> T::AccountId {
		self.id.clone()
	}

	pub fn avatar(&self) -> Image<T> {
		self.avatar.clone()
	}

	pub fn set_name(&mut self, name: ConsumerNameType<T>) {
		self.name = name;
	}

	pub fn set_avatar(&mut self, avatar: Image<T>) {
		self.avatar = avatar;
	}
}
