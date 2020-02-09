use super::Value;
use super::ValueKind;
use super::DynamicValue;
use super::IDModule;
use super::TypeID;
use super::Tag;

pub struct Module {}
#[derive(std::fmt::Debug)]
pub struct Block {}
#[derive(std::fmt::Debug)]
pub struct Entity {}
#[derive(std::fmt::Debug)]
pub struct Item {}
impl Value for Block {
    const SERIALIZED_SIZE_HINT: usize = 0;
    const VALUE_KIND: ValueKind = ValueKind::BlockKind;
    fn deserialize_from(src: &[u8]) -> DynamicValue {
        DynamicValue {
            data: std::ptr::null_mut(),
            type_id: Module::TYPE_ID,
        }
    }
    fn serialize_into(dynamic_value: &DynamicValue) -> Vec<u8> {
        Vec::with_capacity(Self::SERIALIZED_SIZE_HINT)
    }
}
impl Value for Item {
    const SERIALIZED_SIZE_HINT: usize = 0;
    const VALUE_KIND: ValueKind = ValueKind::ItemKind;
    fn deserialize_from(src: &[u8]) -> DynamicValue {
        DynamicValue {
            data: std::ptr::null_mut(),
            type_id: Module::TYPE_ID,
        }
    }
    fn serialize_into(dynamic_value: &DynamicValue) -> Vec<u8> {
        vec![]
    }
}
impl Value for Entity {
    const SERIALIZED_SIZE_HINT: usize = 0;
    const VALUE_KIND: ValueKind = ValueKind::NoneKind;
    fn deserialize_from(src: &[u8]) -> DynamicValue {
        DynamicValue {
            data: std::ptr::null_mut(),
            type_id: Module::TYPE_ID,
        }
    }
    fn serialize_into(dynamic_value: &DynamicValue) -> Vec<u8> {
        vec![]
    }
}
impl IDModule for Module {
    const TYPE_ID: TypeID<'static> = "stone";
    const TAG_LIST: &'static [Tag] = &[Tag::Stone, Tag::CanBeBurn];
    type BlockValue = Block;
    type EntityValue = Entity;
    type ItemValue = Item;
}