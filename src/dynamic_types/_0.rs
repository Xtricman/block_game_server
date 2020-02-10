use super::EntityValue;
use super::EntityDynamicValue;
///经验球
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
    size: u64,
}
impl EntityValue for Entity {
    const SERIALIZED_SIZE_HINT: usize = 0;
    fn deserialize_from(src: &[u8]) -> EntityDynamicValue {
        if src.len() < 8 {
            super::EntityDynamicValue {
                data: Box::into_raw(Box::new(0u64)) as *mut (),
                type_id: Module::TYPE_ID,
            }
        }
        else {
            let i = u64::from_le_bytes(unsafe{*(src.as_ptr() as *const [u8;8])});
            super::EntityDynamicValue {
                data: Box::into_raw(Box::new(i)) as *mut (),
                type_id: Module::TYPE_ID,
            }
        }
    }
    fn serialize_into(dynamic_value: &EntityDynamicValue) -> Vec<u8> {
        let p = dynamic_value.data as *const Entity;
        let ra = unsafe{(*p).size}.to_le_bytes();
        (&ra[..]).to_vec()
    }
    fn drop(dynamic_value: &mut EntityDynamicValue) {
        unsafe{Box::from_raw(dynamic_value.data as *mut Entity)};
    }
}