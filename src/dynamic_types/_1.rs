///石头
use super::Value;
use super::IDModule;
use super::TypeID;
use super::Tag;

pub struct Module {}
impl IDModule for Module {
    const TYPE_ID: TypeID = 1;
    const TAG_LIST: &'static [Tag] = &[Tag::Stone, Tag::CanBeBurn];
    type BlockValue = Block;
    type EntityValue = ();
    type ItemValue = Item;
}



#[derive(std::fmt::Debug)]
pub struct Block {}
impl Value for Block {
    fn deserialize_from(_src: &[u8]) -> *mut () {
        std::ptr::null_mut()
    }
    fn serialize_into(_dynamic_value: *const ()) -> Vec<u8> {
        Vec::with_capacity(0)
    }
    fn drop(_dynamic_value: *mut ()) {}
}



#[derive(std::fmt::Debug)]
pub struct Item {}
impl Value for Item {
    fn deserialize_from(_src: &[u8]) -> *mut () {
        std::ptr::null_mut()
    }
    fn serialize_into(_dynamic_value: *const ()) -> Vec<u8> {
        Vec::with_capacity(0)
    }
    fn drop(_dynamic_types: *mut ()) {}
}