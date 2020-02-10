///每个ID写一个类型，实现这个Trait，此类型作为模块使用
pub trait IDModule: 'static {
    const TYPE_ID: TypeID;
    const TAG_LIST: &'static [Tag];
    type BlockValue: BlockValue; //若此ID不可作为方块，则使用()来禁用
    type EntityValue: EntityValue; //若此ID不可作为实体，则使用()来禁用
    type ItemValue: ItemValue; //若此ID不可作为物品，则使用()来禁用
}

///描述ID模块的ID，Tags，各个函数
#[derive(Copy, Clone)]
pub struct IDModuleInfo {
    type_id: TypeID,
    tags: &'static [Tag],
    block_functions: Option<BlockFunctions>,
    entity_functions: Option<EntityFunctions>,
    item_functions: Option<ItemFunctions>,
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
    let block = if !type_eq::<T::BlockValue, ()>() {Some(BlockFunctions{
        deserialize_from: T::BlockValue::deserialize_from,
        serialize_into: T::BlockValue::serialize_into,
        serialize_size_hint: T::BlockValue::SERIALIZED_SIZE_HINT,
        drop: T::BlockValue::drop,
    })} else {None};
    let entity = if !type_eq::<T::EntityValue, ()>() {Some(EntityFunctions{
        deserialize_from: T::EntityValue::deserialize_from,
        serialize_into: T::EntityValue::serialize_into,
        serialize_size_hint: T::EntityValue::SERIALIZED_SIZE_HINT,
        drop: T::EntityValue::drop,
    })} else {None};
    let item = if !type_eq::<T::ItemValue, ()>() {Some(ItemFunctions{
        deserialize_from: T::ItemValue::deserialize_from,
        serialize_into: T::ItemValue::serialize_into,
        serialize_size_hint: T::ItemValue::SERIALIZED_SIZE_HINT,
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


























pub trait BlockValue: std::fmt::Debug {
    const SERIALIZED_SIZE_HINT: usize; //此NBT数据类型序列化的预估大小
    fn deserialize_from(src: &[u8]) -> BlockDynamicValue;//必须正确实现，返回的type_id必须正确，不允许失败，无论src为何都必须返回一个正确的DynamicValue
    fn serialize_into(dynamic_value: &BlockDynamicValue) -> Vec<u8>;//不允许失败，因为内存中的DynamicValue的数据一定处于正确的状态
    fn drop(dynamic_value: &mut BlockDynamicValue);//析构函数
}
impl BlockValue for () {
    const SERIALIZED_SIZE_HINT: usize = 0;
    fn deserialize_from(_src: &[u8]) -> BlockDynamicValue {
        unimplemented!("TYPE () CAN'T BE USED AS DATA AT ALL, USE EMPTY STRUCT INSTEAD")
    }
    fn serialize_into(_dynamic_value: &BlockDynamicValue) -> Vec<u8> {
        unimplemented!("TYPE () CAN'T BE USED AS DATA AT ALL, USE EMPTY STRUCT INSTEAD")
    }
    fn drop(_dynamic_value: &mut BlockDynamicValue) {
        unimplemented!("TYPE () CAN'T BE USED AS DATA AT ALL, USE EMPTY STRUCT INSTEAD")
    }  
}
#[derive(Copy, Clone)]
struct BlockFunctions {
    drop: fn(&mut BlockDynamicValue),
    deserialize_from: fn(&[u8]) -> BlockDynamicValue,
    serialize_into: fn(&BlockDynamicValue) -> Vec<u8>,
    serialize_size_hint: usize,
}
pub struct BlockDynamicValue {
    data: *mut (),
    type_id: TypeID,
}
impl BlockDynamicValue {
    fn deserialize_from(src: &[u8], type_id: TypeID) -> Option<Self> {
        if let Some(x) = get_type_info_by_type_id(type_id) {
            if let Some(y) = x.block_functions {Some((y.deserialize_from)(src))} else {None}
        } else {None}
    }
    fn serialize_into(&self) -> Vec<u8> {
        let type_info = get_type_info_by_type_id(self.type_id).expect(&format!("CAN'T SERIALIZE BLOCK {} AT {:p}, TYPE_ID DOES NOT EXIST", self.type_id, self.data));
        let functions = type_info.block_functions.expect(&format!("CAN'T SERIALIZE BLOCK {} AT {:p}, TYPE_ID EXIST BUT CAN NOT BE A BLOCK", self.type_id, self.data));
        (functions.serialize_into)(self)
    }
}
impl Drop for BlockDynamicValue {
    fn drop(&mut self) {
        let type_info = get_type_info_by_type_id(self.type_id).expect(&format!("CAN'T DROP BLOCK {} AT {:p}, TYPE_ID DOES NOT EXIST", self.type_id, self.data));
        let functions = type_info.block_functions.expect(&format!("CAN'T DROP BLOCK {} AT {:p}, TYPE_ID EXIST BUT CAN NOT BE A BLOCK", self.type_id, self.data));
        (functions.drop)(self)
    }
}












pub trait EntityValue: std::fmt::Debug {
    const SERIALIZED_SIZE_HINT: usize; //此NBT数据类型序列化的预估大小
    fn deserialize_from(src: &[u8]) -> EntityDynamicValue;//必须正确实现，返回的type_id必须正确，不允许失败，无论src为何都必须返回一个正确的DynamicValue
    fn serialize_into(dynamic_value: &EntityDynamicValue) -> Vec<u8>;//不允许失败，因为内存中的DynamicValue的数据一定处于正确的状态
    fn drop(dynamic_value: &mut EntityDynamicValue);//析构函数
}
impl EntityValue for () {
    const SERIALIZED_SIZE_HINT: usize = 0;
    fn deserialize_from(_src: &[u8]) -> EntityDynamicValue {
        unimplemented!("TYPE () CAN'T BE USED AS DATA AT ALL, USE EMPTY STRUCT INSTEAD")
    }
    fn serialize_into(_dynamic_value: &EntityDynamicValue) -> Vec<u8> {
        unimplemented!("TYPE () CAN'T BE USED AS DATA AT ALL, USE EMPTY STRUCT INSTEAD")
    }
    fn drop(_dynamic_value: &mut EntityDynamicValue) {
        unimplemented!("TYPE () CAN'T BE USED AS DATA AT ALL, USE EMPTY STRUCT INSTEAD")
    }
}
#[derive(Copy, Clone)]
struct EntityFunctions {
    drop: fn(&mut EntityDynamicValue),
    deserialize_from: fn(&[u8]) -> EntityDynamicValue,
    serialize_into: fn(&EntityDynamicValue) -> Vec<u8>,
    serialize_size_hint: usize,
}
pub struct EntityDynamicValue {
    data: *mut (),
    type_id: TypeID,
}
impl EntityDynamicValue {
    fn deserialize_from(src: &[u8], type_id: TypeID) -> Option<Self> {
        if let Some(x) = get_type_info_by_type_id(type_id) {
            if let Some(y) = x.entity_functions {Some((y.deserialize_from)(src))} else {None}
        } else {None}
    }
    fn serialize_into(&self) -> Vec<u8> {
        let type_info = get_type_info_by_type_id(self.type_id).expect(&format!("CAN'T SERIALIZE ENTITY {} AT {:p}, TYPE_ID DOES NOT EXIST", self.type_id, self.data));
        let functions = type_info.entity_functions.expect(&format!("CAN'T SERIALIZE ENTITY {} AT {:p}, TYPE_ID EXIST BUT CAN NOT BE A ENTITY", self.type_id, self.data));
        (functions.serialize_into)(self)
    }
}
impl Drop for EntityDynamicValue {
    fn drop(&mut self) {
        let type_info = get_type_info_by_type_id(self.type_id).expect(&format!("CAN'T DROP ENTITY {} AT {:p}, TYPE_ID DOES NOT EXIST", self.type_id, self.data));
        let functions = type_info.entity_functions.expect(&format!("CAN'T DROP ENTITY {} AT {:p}, TYPE_ID EXIST BUT CAN NOT BE A ENTITY", self.type_id, self.data));
        (functions.drop)(self)
    }
}















pub trait ItemValue: std::fmt::Debug {
    const SERIALIZED_SIZE_HINT: usize; //此NBT数据类型序列化的预估大小
    fn deserialize_from(src: &[u8]) -> ItemDynamicValue;//必须正确实现，返回的type_id必须正确，不允许失败，无论src为何都必须返回一个正确的DynamicValue
    fn serialize_into(dynamic_value: &ItemDynamicValue) -> Vec<u8>;//不允许失败，因为内存中的DynamicValue的数据一定处于正确的状态
    fn drop(dynamic_value: &mut ItemDynamicValue);//析构函数
}
impl ItemValue for () {
    const SERIALIZED_SIZE_HINT: usize = 0;
    fn deserialize_from(_src: &[u8]) -> ItemDynamicValue {
        unimplemented!("TYPE () CAN'T BE USED AS DATA AT ALL, USE EMPTY STRUCT INSTEAD")
    }
    fn serialize_into(_dynamic_value: &ItemDynamicValue) -> Vec<u8> {
        unimplemented!("TYPE () CAN'T BE USED AS DATA AT ALL, USE EMPTY STRUCT INSTEAD")
    }
    fn drop(_dynamic_value: &mut ItemDynamicValue) {
        unimplemented!("TYPE () CAN'T BE USED AS DATA AT ALL, USE EMPTY STRUCT INSTEAD")
    }
}
#[derive(Copy, Clone)]
struct ItemFunctions {
    drop: fn(&mut ItemDynamicValue),
    deserialize_from: fn(&[u8]) -> ItemDynamicValue,
    serialize_into: fn(&ItemDynamicValue) -> Vec<u8>,
    serialize_size_hint: usize,
}
pub struct ItemDynamicValue {
    data: *mut (),
    type_id: TypeID,
}
impl ItemDynamicValue {
    fn deserialize_from(src: &[u8], type_id: TypeID) -> Option<Self> {
        if let Some(x) = get_type_info_by_type_id(type_id) {
            if let Some(y) = x.item_functions {Some((y.deserialize_from)(src))} else {None}
        } else {None}
    }
    fn serialize_into(&self) -> Vec<u8> {
        let type_info = get_type_info_by_type_id(self.type_id).expect(&format!("CAN'T SERIALIZE ITEM {} AT {:p}, TYPE_ID DOES NOT EXIST", self.type_id, self.data));
        let functions = type_info.item_functions.expect(&format!("CAN'T SERIALIZE ITEM {} AT {:p}, TYPE_ID EXIST BUT CAN NOT BE A ITEM", self.type_id, self.data));
        (functions.serialize_into)(self)
    }
}
impl Drop for ItemDynamicValue {
    fn drop(&mut self) {
        let type_info = get_type_info_by_type_id(self.type_id).expect(&format!("CAN'T DROP ITEM {} AT {:p}, TYPE_ID DOES NOT EXIST", self.type_id, self.data));
        let functions = type_info.item_functions.expect(&format!("CAN'T DROP ITEM {} AT {:p}, TYPE_ID EXIST BUT CAN NOT BE A ITEM", self.type_id, self.data));
        (functions.drop)(self)
    }
}