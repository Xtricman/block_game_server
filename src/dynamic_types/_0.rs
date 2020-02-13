///经验球
use super::Value;
use super::IDModule;
use super::Tag;

pub enum Module;
impl IDModule for Module {
    const TAG_LIST: &'static [Tag] = &[];
    type BlockValue = ();
    type EntityValue = Entity;
    type ItemValue = ();
}



#[derive(std::fmt::Debug, PartialEq, Eq, Clone)]
pub struct Entity {
    exp_amount: u64,
}
impl Value for Entity {
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