///石头
use super::Value;
use super::IDModule;
use super::Tag;

pub enum Module;
impl IDModule for Module {
    const TAG_LIST: &'static [Tag] = &[Tag::Stone, Tag::CanBeBurn];
    type BlockValue = Block;
    type EntityValue = ();
    type ItemValue = Item;
}



#[derive(std::fmt::Debug, PartialEq, Eq, Clone)]
pub struct Block;
impl Value for Block {
    fn deserialize_from(_src: &[u8]) -> *mut () {
        Box::into_raw(Box::new(Block{}))
    }
    fn serialize_into(_dynamic_value: *const ()) -> Vec<u8> {
        Vec::with_capacity(0)
    }
    fn drop(dynamic_value: *mut ()) {
        unsafe{Box::from_raw(dynamic_value as *mut Block)}
    }
}



#[derive(std::fmt::Debug, PartialEq, Eq, Clone)]
pub struct Item;
impl Value for Item {
    fn deserialize_from(_src: &[u8]) -> *mut () {
        Box::into_raw(Box::new(Block{}))
    }
    fn serialize_into(_dynamic_value: *const ()) -> Vec<u8> {
        Vec::with_capacity(0)
    }
    fn drop(_dynamic_types: *mut ()) {
        unsafe{Box::from_raw(dynamic_value as *mut Block)}
    }
}