///经验球
use super::Value;

use super::IDModule;
use super::TypeID;
use super::Tag;

pub struct Module {}
impl IDModule for Module {
    const TYPE_ID: TypeID = 0;
    const TAG_LIST: &'static [Tag] = &[Tag::Stone, Tag::CanBeBurn];
    type BlockValue = ();
    type EntityValue = Entity;
    type ItemValue = ();
}



#[derive(std::fmt::Debug)]
pub struct Entity {
    exp_amount: u64,
}
impl Value for Entity {
    const SERIALIZED_SIZE_HINT: usize = 0;
    fn deserialize_from(src: &[u8]) -> *mut () {
        if src.len() < 8 {
            Box::into_raw(Box::new(0u64)) as *mut ()
        }
        else {
            let i = u64::from_le_bytes(unsafe{*(src.as_ptr() as *const [u8;8])});
            Box::into_raw(Box::new(i)) as *mut ()
        }
    }
    fn serialize_into(dynamic_value: *const ()) -> Vec<u8> {
        let ra = unsafe{&*(dynamic_value as *const Entity)}.exp_amount.to_le_bytes();
        (&ra[..]).to_vec()
    }
    fn drop(dynamic_value: *mut ()) {
        unsafe{Box::from_raw(dynamic_value as *mut Entity)};
    }
}