///石头
use super::BlockValue;
use super::BlockDynamicValue;
use super::ItemValue;
use super::ItemDynamicValue;
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
impl BlockValue for Block {
    const SERIALIZED_SIZE_HINT: usize = 0;
    fn deserialize_from(_src: &[u8]) -> BlockDynamicValue {
        BlockDynamicValue {
            data: std::ptr::null_mut(),
            type_id: Module::TYPE_ID,
        }
    }
    fn serialize_into(_dynamic_value: &BlockDynamicValue) -> Vec<u8> {
        Vec::with_capacity(Self::SERIALIZED_SIZE_HINT)
    }
    fn drop(_dynamic_value: &mut BlockDynamicValue) {}
}



#[derive(std::fmt::Debug)]
pub struct Item {}
impl ItemValue for Item {
    const SERIALIZED_SIZE_HINT: usize = 0;
    fn deserialize_from(_src: &[u8]) -> ItemDynamicValue {
        ItemDynamicValue {
            data: std::ptr::null_mut(),
            type_id: Module::TYPE_ID,
        }
    }
    fn serialize_into(_dynamic_value: &ItemDynamicValue) -> Vec<u8> {
        vec![]
    }
    fn drop(_dynamic_types: &mut ItemDynamicValue) {}
}