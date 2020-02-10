///每个ID写一个类型，实现这个Trait，此类型作为模块使用
pub trait IDModule: 'static {
    const TYPE_ID: TypeID;
    const TAG_LIST: &'static [Tag];
    type BlockValue: Value; //若此ID不可作为方块，则使用()来禁用
    type EntityValue: Value; //若此ID不可作为实体，则使用()来禁用
    type ItemValue: Value; //若此ID不可作为物品，则使用()来禁用
}

///描述ID模块的ID，Tags，各个函数
#[derive(Copy, Clone)]
pub struct IDModuleInfo {
    type_id: TypeID,
    tags: &'static [Tag],
    block_functions: Option<Functions>,
    entity_functions: Option<Functions>,
    item_functions: Option<Functions>,
}

///ID类型，是一个编译期字符串
pub type TypeID = u16;

///某个ID拥有的方块、物品、实体标签
#[derive(PartialEq,Eq,Copy,Clone)]
pub enum Tag {
    CanBeBurn,
    RedStonePowerSource,
    Wood,
    Stone,
}

pub const fn type_eq<T: ?Sized, U: ?Sized>() -> bool {
    // Helper trait. `VALUE` is false, except for the specialization of the
    // case where `T == U`.
    trait TraitEq<U: ?Sized> {
        const VALUE: bool;
    }

    // Default implementation.
    impl<T: ?Sized, U: ?Sized> TraitEq<U> for T {
        default const VALUE: bool = false;
    }
    
    // Specialization for `T == U`.
    impl<T: ?Sized> TraitEq<T> for T {
        const VALUE: bool = true;
    }

    <T as TraitEq<U>>::VALUE
}

///将一个ID模块转换为IDModuleInfo值
const fn into_id_module_info<T: IDModule>() -> IDModuleInfo {
    let block = if !type_eq::<T::BlockValue, ()>() {Some(Functions{
        deserialize_from: T::BlockValue::deserialize_from,
        serialize_into: T::BlockValue::serialize_into,
        drop: T::BlockValue::drop,
    })} else {None};
    let entity = if !type_eq::<T::EntityValue, ()>() {Some(Functions{
        deserialize_from: T::EntityValue::deserialize_from,
        serialize_into: T::EntityValue::serialize_into,
        drop: T::EntityValue::drop,
    })} else {None};
    let item = if !type_eq::<T::ItemValue, ()>() {Some(Functions{
        deserialize_from: T::ItemValue::deserialize_from,
        serialize_into: T::ItemValue::serialize_into,
        drop: T::ItemValue::drop,
    })} else {None};
    IDModuleInfo {
        type_id: T::TYPE_ID,
        tags: T::TAG_LIST,
        block_functions: block,
        entity_functions: entity,
        item_functions: item,
    }
}







///每个NBT数据类型皆应实现此Trait
pub trait Value: std::fmt::Debug {
    fn deserialize_from(src: &[u8]) -> *mut ();//必须正确实现，返回的type_id必须正确，不允许失败，无论src为何都必须正确alloc heap并返回*mut ()
    fn serialize_into(dynamic_value: *const ()) -> Vec<u8>;//不允许失败，因为内存中的DynamicValue的数据一定处于正确的状态
    fn drop(dynamic_value: *mut ());//析构函数
}
impl Value for () {
    fn deserialize_from(_src: &[u8]) -> *mut () {unreachable!("IMPOSSIBLE TO TO CALL TYPE () AS VALUE")}
    fn serialize_into(_dynamic_value: *const ()) -> Vec<u8> {unreachable!("IMPOSSIBLE TO TO CALL TYPE () AS VALUE")}
    fn drop(_dynamic_value: *mut ()) {unreachable!("IMPOSSIBLE TO TO CALL TYPE () AS VALUE")}
}
trait DynamicValue: Drop {
    fn deserialize_from(src: &[u8], type_id: TypeID) -> Option<*mut ()>;
    fn serialize_into(&self) -> Vec<u8>;
}
#[derive(Copy, Clone)]
struct Functions {
    drop: fn(*mut ()),
    deserialize_from: fn(&[u8]) -> *mut (),
    serialize_into: fn(*const ()) -> Vec<u8>,
}




























pub mod _0;
pub mod _1;

///TODO：编译期生成FEATURE_MAP，确保写了的ID模块全都被转换为TypeInfo
static FEATURE_MAP: [IDModuleInfo; 2] = [
    into_id_module_info::<_0::Module>(),
    into_id_module_info::<_1::Module>(),
];

pub fn filter_ids_by_tag(tag: Tag) -> Vec<TypeID> {
    let mut r = Vec::<TypeID>::with_capacity(4);
    for i in &FEATURE_MAP {
        if i.tags.contains(&tag) {r.push(i.type_id)}
    }
    r
}
///若type_id不存在于FEATURE_MAP而失败
pub fn get_type_info_by_type_id(type_id: TypeID) -> Option<IDModuleInfo> {
    let i = usize::from(type_id);
    if i >= FEATURE_MAP.len() {None} else {Some(FEATURE_MAP[i])}
}



























pub struct BlockDynamicValue {
    data: *mut (),
    type_id: TypeID,
}
impl Drop for BlockDynamicValue {
    fn drop(&mut self) {
        let type_info = get_type_info_by_type_id(self.type_id).expect(&format!("CAN'T DROP BLOCK {} AT {:p}, TYPE_ID DOES NOT EXIST", self.type_id, self.data));
        let functions = type_info.block_functions.expect(&format!("CAN'T DROP BLOCK {} AT {:p}, TYPE_ID EXIST BUT CAN NOT BE A BLOCK", self.type_id, self.data));
        (functions.drop)(self.data)
    }
}
impl DynamicValue for BlockDynamicValue {
    fn deserialize_from(src: &[u8], type_id: TypeID) -> Option<*mut ()> {
        if let Some(x) = get_type_info_by_type_id(type_id) {
            if let Some(y) = x.block_functions {Some((y.deserialize_from)(src))} else {None}
        } else {None}
    }
    fn serialize_into(&self) -> Vec<u8> {
        let type_info = get_type_info_by_type_id(self.type_id).expect(&format!("CAN'T SERIALIZE BLOCK {} AT {:p}, TYPE_ID DOES NOT EXIST", self.type_id, self.data));
        let functions = type_info.block_functions.expect(&format!("CAN'T SERIALIZE BLOCK {} AT {:p}, TYPE_ID EXIST BUT CAN NOT BE A BLOCK", self.type_id, self.data));
        (functions.serialize_into)(self.data)
    }
}












pub struct EntityDynamicValue {
    data: *mut (),
    type_id: TypeID,
}
impl Drop for EntityDynamicValue {
    fn drop(&mut self) {
        let type_info = get_type_info_by_type_id(self.type_id).expect(&format!("CAN'T DROP ENTITY {} AT {:p}, TYPE_ID DOES NOT EXIST", self.type_id, self.data));
        let functions = type_info.entity_functions.expect(&format!("CAN'T DROP ENTITY {} AT {:p}, TYPE_ID EXIST BUT CAN NOT BE A ENTITY", self.type_id, self.data));
        (functions.drop)(self.data)
    }
}
impl DynamicValue for EntityDynamicValue {
    fn deserialize_from(src: &[u8], type_id: TypeID) -> Option<*mut ()> {
        if let Some(x) = get_type_info_by_type_id(type_id) {
            if let Some(y) = x.entity_functions {Some((y.deserialize_from)(src))} else {None}
        } else {None}
    }
    fn serialize_into(&self) -> Vec<u8> {
        let type_info = get_type_info_by_type_id(self.type_id).expect(&format!("CAN'T SERIALIZE ENTITY {} AT {:p}, TYPE_ID DOES NOT EXIST", self.type_id, self.data));
        let functions = type_info.entity_functions.expect(&format!("CAN'T SERIALIZE ENTITY {} AT {:p}, TYPE_ID EXIST BUT CAN NOT BE A ENTITY", self.type_id, self.data));
        (functions.serialize_into)(self.data)
    }
}
















pub struct ItemDynamicValue {
    data: *mut (),
    type_id: TypeID,
}
impl Drop for ItemDynamicValue {
    fn drop(&mut self) {
        let type_info = get_type_info_by_type_id(self.type_id).expect(&format!("CAN'T DROP ITEM {} AT {:p}, TYPE_ID DOES NOT EXIST", self.type_id, self.data));
        let functions = type_info.item_functions.expect(&format!("CAN'T DROP ITEM {} AT {:p}, TYPE_ID EXIST BUT CAN NOT BE A ITEM", self.type_id, self.data));
        (functions.drop)(self.data)
    }
}
impl DynamicValue for ItemDynamicValue {
    fn deserialize_from(src: &[u8], type_id: TypeID) -> Option<*mut ()> {
        if let Some(x) = get_type_info_by_type_id(type_id) {
            if let Some(y) = x.item_functions {Some((y.deserialize_from)(src))} else {None}
        } else {None}
    }
    fn serialize_into(&self) -> Vec<u8> {
        let type_info = get_type_info_by_type_id(self.type_id).expect(&format!("CAN'T SERIALIZE ITEM {} AT {:p}, TYPE_ID DOES NOT EXIST", self.type_id, self.data));
        let functions = type_info.item_functions.expect(&format!("CAN'T SERIALIZE ITEM {} AT {:p}, TYPE_ID EXIST BUT CAN NOT BE A ITEM", self.type_id, self.data));
        (functions.serialize_into)(self.data)
    }
}