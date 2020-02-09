///ID类型，是一个编译期字符串
pub type TypeID<'a> = &'a str;

///每个ID写一个类型，实现这个Trait，此类型作为模块使用
pub trait IDModule {
    const TYPE_ID: TypeID<'static>;
    const TAG_LIST: &'static [Tag];
    type BlockValue: Value; //使用VALUE_ID==ValueKind::BlockKind的类型，若此ID不可作为方块，则使用一个Self::VALUE_KIND==ValueKind::NoneKind的类型来禁用
    type EntityValue: Value; //必须使用VALUE_ID==ValueKind::EntityKind的类型，若此ID不可作为实体，则使用一个Self::VALUE_KIND==ValueKind::NoneKind的类型来禁用
    type ItemValue: Value; //必须使用VALUE_ID==ValueKind::ItemKind的类型，若此ID不可作为物品，则使用一个Self::VALUE_KIND==ValueKind::NoneKind的类型来禁用
}
#[derive(PartialEq,Eq,Copy,Clone)]
enum ValueKind {
    NoneKind,
    BlockKind,
    EntityKind,
    ItemKind,
}
///某个ID拥有的方块、物品、实体标签
#[derive(PartialEq,Eq,Copy,Clone)]
pub enum Tag {
    CanBeBurn,
    RedStonePowerSource,
    Wood,
    Stone,
    Dirt,
}
pub trait Value: std::fmt::Debug {
    const SERIALIZED_SIZE_HINT: usize; //此NBT数据类型序列化的预估大小
    const VALUE_KIND: ValueKind; //此NBT数据类型是方块、实体还是物品，值为ValueKind::NoneKind表示此NBT数据类型不启用（很可能是当前ID不可做为方块、实体或物品）
    fn deserialize_from(src: &[u8]) -> DynamicValue;//必须正确实现，返回的type_id必须正确，不允许失败，无论src为何都必须返回一个正确的DynamicValue
    fn serialize_into(dynamic_value: &DynamicValue) -> Vec<u8>;//不允许失败，因为内存中的DynamicValue的数据一定处于正确的状态
}
const fn into_id_module_info<T: IDModule>() -> IDModuleInfo {
    let block = if let ValueKind::BlockKind = T::BlockValue::VALUE_KIND {Some(SerializeDeserializeFunctions{
        deserialize_from: T::BlockValue::deserialize_from,
        serialize_into: T::BlockValue::serialize_into,
        serialize_size_hint: T::BlockValue::SERIALIZED_SIZE_HINT,
    })} else {None};
    let entity = if let ValueKind::EntityKind = T::EntityValue::VALUE_KIND {Some(SerializeDeserializeFunctions{
        deserialize_from: T::EntityValue::deserialize_from,
        serialize_into: T::EntityValue::serialize_into,
        serialize_size_hint: T::EntityValue::SERIALIZED_SIZE_HINT,
    })} else {None};
    let item = if let ValueKind::ItemKind = T::ItemValue::VALUE_KIND {Some(SerializeDeserializeFunctions{
        deserialize_from: T::ItemValue::deserialize_from,
        serialize_into: T::ItemValue::serialize_into,
        serialize_size_hint: T::ItemValue::SERIALIZED_SIZE_HINT,
    })} else {None};
    IDModuleInfo {
        type_id: T::TYPE_ID,
        tags: T::TAG_LIST,
        block_functions: block,
        entity_functions: entity,
        item_functions: item,
    }
}
#[derive(Copy, Clone)]
struct SerializeDeserializeFunctions {
    deserialize_from: fn(&[u8]) -> DynamicValue,
    serialize_into: fn(&DynamicValue) -> Vec<u8>,
    serialize_size_hint: usize,
}
///描述ID模块的ID，Tags，各个函数
#[derive(Copy, Clone)]
pub struct IDModuleInfo {
    type_id: TypeID<'static>,
    tags: &'static [Tag],
    block_functions: Option<SerializeDeserializeFunctions>,
    entity_functions: Option<SerializeDeserializeFunctions>,
    item_functions: Option<SerializeDeserializeFunctions>,
}



pub mod stone;



///TODO：编译期生成FEATURE_MAP，确保写了的ID模块全都被转换为TypeInfo
static FEATURE_MAP: &'static [IDModuleInfo] = &[into_id_module_info::<stone::Module>()];
pub fn filter_ids_by_tag(tag: Tag) -> Vec<TypeID<'static>> {
    let mut r = Vec::<TypeID>::with_capacity(4);
    for i in FEATURE_MAP {
        if i.tags.contains(&tag) {r.push(i.type_id)}
    }
    r
}
///可能会因为type_id不存在于FEATURE_MAP而失败
pub fn get_type_info_ref_by_type_id(type_id: TypeID) -> Option<&'static IDModuleInfo> {
    for i in FEATURE_MAP {
        if i.type_id==type_id {return Some(i);}
    }
    {None}
}
///对于一个ID模块，它的ID一定会存在于FEATURE_MAP
pub fn get_type_info_ref_by_id_module<T: IDModule>() -> &'static IDModuleInfo {
    get_type_info_ref_by_type_id(T::TYPE_ID).unwrap()
}








pub struct DynamicValue {
    data: *mut (),
    type_id: TypeID<'static>,
}

impl DynamicValue {
    fn deserialize_block_from<'a>(src: &[u8], type_id: TypeID<'a>) -> Option<Self> {
        if let Some(x) = get_type_info_ref_by_type_id(type_id) {
            if let Some(y) = x.block_functions {Some((y.deserialize_from)(src))} else {None}
        } else {None}
    }
    fn deserialize_entity_from<'a>(src: &[u8], type_id: TypeID<'a>) -> Option<Self> {
        if let Some(x) = get_type_info_ref_by_type_id(type_id) {
            if let Some(y) = x.entity_functions {Some((y.deserialize_from)(src))} else {None}
        } else {None}
    }
    fn deserialize_item_from<'a>(src: &[u8], type_id: TypeID<'a>) -> Option<Self> {
        if let Some(x) = get_type_info_ref_by_type_id(type_id) {
            if let Some(y) = x.item_functions {Some((y.deserialize_from)(src))} else {None}
        } else {None}
    }

    fn serialize_block_into(&self) -> Vec<u8> {
        let type_info = get_type_info_ref_by_type_id(self.type_id).expect(&format!("CAN'T SERIALIZE ENTITY {} AT {:p}, TYPE_ID DOES NOT EXIST", self.type_id, self.data));
        let functions = type_info.block_functions.expect(&format!("CAN'T SERIALIZE ENTITY {} AT {:p}, TYPE_ID EXIST BUT CAN NOT BE A BLOCK", self.type_id, self.data));
        (functions.serialize_into)(self)
    }
    fn serialize_entity_into(&self) -> Vec<u8> {
        let type_info = get_type_info_ref_by_type_id(self.type_id).expect(&format!("CAN'T SERIALIZE ENTITY {} AT {:p}, TYPE_ID DOES NOT EXIST", self.type_id, self.data));
        let functions = type_info.entity_functions.expect(&format!("CAN'T SERIALIZE ENTITY {} AT {:p}, TYPE_ID EXIST BUT CAN NOT BE AN ENTITY", self.type_id, self.data));
        (functions.serialize_into)(self)
    }
    fn serialize_item_into(&self) -> Vec<u8> {
        let type_info = get_type_info_ref_by_type_id(self.type_id).expect(&format!("CAN'T SERIALIZE ENTITY {} AT {:p}, TYPE_ID DOES NOT EXIST", self.type_id, self.data));
        let functions = type_info.item_functions.expect(&format!("CAN'T SERIALIZE ENTITY {} AT {:p}, TYPE_ID EXIST BUT CAN NOT BE AN ITEM", self.type_id, self.data));
        (functions.serialize_into)(self)
    }

    fn get_concrete_block_value<T: IDModule>(&self) -> Option<&mut T::BlockValue> {
        if self.type_id == T::TYPE_ID && T::BlockValue::VALUE_KIND == ValueKind::BlockKind {unsafe{Some(std::mem::transmute(self.data))}} else {None}
    }
    fn get_concrete_entity_value<T: IDModule>(&self) -> Option<&mut T::EntityValue> {
        if self.type_id == T::TYPE_ID && T::EntityValue::VALUE_KIND == ValueKind::EntityKind {unsafe{Some(std::mem::transmute(self.data))}} else {None}
    }
    fn get_concrete_item_value<T: IDModule>(&self) -> Option<&mut T::ItemValue> {
        if self.type_id == T::TYPE_ID && T::ItemValue::VALUE_KIND == ValueKind::ItemKind {unsafe{Some(std::mem::transmute(self.data))}} else {None}
    }
}

impl Drop for DynamicValue {
    fn drop(&mut self) {
        unsafe{drop(Box::from_raw(self.data))};
    }
}