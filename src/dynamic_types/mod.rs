///ID类型
pub type TypeID = u16;

///某个ID拥有的方块、物品、实体标签
#[derive(PartialEq,Eq,Copy,Clone)]
pub enum Tag {
    CanBeBurn,
    RedStonePowerSource,
    Wood,
    Stone,
}

///每个ID写一个类型，实现这个Trait，此类型作为模块使用
pub trait IDModule: 'static {
    const TAG_LIST: &'static [Tag];
    type BlockValue: Value; //若此ID不可作为方块，则使用()来禁用
    type EntityValue: Value; //若此ID不可作为实体，则使用()来禁用
    type ItemValue: Value; //若此ID不可作为物品，则使用()来禁用
}

///描述ID模块的信息
#[derive(Copy, Clone)]
pub struct IDModuleInfo {
    tags: &'static [Tag], //此模块实现的Tag
    block_functions: Option<Functions>, //方块的数据函数，若不能作为方块则为None
    entity_functions: Option<Functions>, //实体的数据函数，若不能作为方块则为None
    item_functions: Option<Functions>, //物品的数据函数，若不能作为方块则为None
}

///判定类型同一性
pub const fn type_eq<T: ?Sized, U: ?Sized>() -> bool {
    trait TraitEq<U: ?Sized> {
        const VALUE: bool;
    }
    impl<T: ?Sized, U: ?Sized> TraitEq<U> for T {
        default const VALUE: bool = false;
    }
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
        tags: T::TAG_LIST,
        block_functions: block,
        entity_functions: entity,
        item_functions: item,
    }
}



///每个NBT数据类型皆应实现此Trait
pub trait Value: std::fmt::Debug+Eq+Clone {
    fn deserialize_from(src: &[u8]) -> *mut ();//必须正确实现，返回的type_id必须正确，不允许失败，无论src为何都必须正确alloc heap并返回*mut ()
    fn serialize_into(dynamic_value: *const ()) -> Vec<u8>;//不允许失败，因为内存中的DynamicValue的数据一定处于正确的状态
    fn drop(dynamic_value: *mut ());//析构函数
}
impl Value for () {
    fn deserialize_from(_src: &[u8]) -> *mut () {unreachable!("IMPOSSIBLE TO TO CALL TYPE () AS VALUE")}
    fn serialize_into(_dynamic_value: *const ()) -> Vec<u8> {unreachable!("IMPOSSIBLE TO TO CALL TYPE () AS VALUE")}
    fn drop(_dynamic_value: *mut ()) {unreachable!("IMPOSSIBLE TO TO CALL TYPE () AS VALUE")}
}

///每个NBT类型的析构，反序列化，序列化函数，只需要实现数据读写即可
#[derive(Copy, Clone)]
struct Functions {
    drop: fn(*mut ()),
    deserialize_from: fn(&[u8]) -> *mut (),
    serialize_into: fn(*const ()) -> Vec<u8>,
}



///TODO：编译期生成FEATURE_MAP，确保写了的ID模块全都被转换为TypeInfo
pub mod _0;
pub mod _1;

static FEATURE_MAP: [IDModuleInfo; 2] = [
    into_id_module_info::<_0::Module>(),
    into_id_module_info::<_1::Module>(),
];

///根据Tag筛选ID
pub fn filter_ids_by_tag(tag: Tag) -> Vec<TypeID> {
    use std::convert::TryFrom;
    let mut r = Vec::<TypeID>::with_capacity(4);
    let len = u16::try_from(std::mem::size_of_val(&FEATURE_MAP)/std::mem::size_of::<IDModuleInfo>()).unwrap();    
    for i in 0..len {
        if FEATURE_MAP[i as usize].tags.contains(&tag) {r.push(i)}
    }
    r
}

///若type_id不存在于FEATURE_MAP则返回None
pub fn get_type_info_by_type_id(type_id: TypeID) -> Option<IDModuleInfo> {
    let i = usize::from(type_id);
    if i >= FEATURE_MAP.len() {None} else {Some(FEATURE_MAP[i])}
}


///类型擦除的值都实现这个Trait
trait DynamicValue: Drop+Sized {
    fn deserialize_from(src: &[u8], type_id: TypeID) -> Option<Self>;
    fn serialize_into(&self) -> Vec<u8>;
}



///方块类型
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
    fn deserialize_from(src: &[u8], type_id: TypeID) -> Option<Self> {
        if let Some(x) = get_type_info_by_type_id(type_id) {
            if let Some(y) = x.block_functions {
                Some(Self {
                    data: (y.deserialize_from)(src),
                    type_id: type_id,
                })
            } else {None}
        } else {None}
    }
    fn serialize_into(&self) -> Vec<u8> {
        let type_info = get_type_info_by_type_id(self.type_id).expect(&format!("CAN'T SERIALIZE BLOCK {} AT {:p}, TYPE_ID DOES NOT EXIST", self.type_id, self.data));
        let functions = type_info.block_functions.expect(&format!("CAN'T SERIALIZE BLOCK {} AT {:p}, TYPE_ID EXIST BUT CAN NOT BE A BLOCK", self.type_id, self.data));
        (functions.serialize_into)(self.data)
    }
}



///实体类型
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
    fn deserialize_from(src: &[u8], type_id: TypeID) -> Option<Self> {
        if let Some(x) = get_type_info_by_type_id(type_id) {
            if let Some(y) = x.entity_functions {
                Some(Self {
                    data: (y.deserialize_from)(src),
                    type_id: type_id,
                })
            } else {None}
        } else {None}
    }
    fn serialize_into(&self) -> Vec<u8> {
        let type_info = get_type_info_by_type_id(self.type_id).expect(&format!("CAN'T SERIALIZE ENTITY {} AT {:p}, TYPE_ID DOES NOT EXIST", self.type_id, self.data));
        let functions = type_info.entity_functions.expect(&format!("CAN'T SERIALIZE ENTITY {} AT {:p}, TYPE_ID EXIST BUT CAN NOT BE A ENTITY", self.type_id, self.data));
        (functions.serialize_into)(self.data)
    }
}



///物品类型
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
    fn deserialize_from(src: &[u8], type_id: TypeID) -> Option<Self> {
        if let Some(x) = get_type_info_by_type_id(type_id) {
            if let Some(y) = x.item_functions {
                Some(Self {
                    data: (y.deserialize_from)(src),
                    type_id: type_id,
                })
            } else {None}
        } else {None}
    }
    fn serialize_into(&self) -> Vec<u8> {
        let type_info = get_type_info_by_type_id(self.type_id).expect(&format!("CAN'T SERIALIZE ITEM {} AT {:p}, TYPE_ID DOES NOT EXIST", self.type_id, self.data));
        let functions = type_info.item_functions.expect(&format!("CAN'T SERIALIZE ITEM {} AT {:p}, TYPE_ID EXIST BUT CAN NOT BE A ITEM", self.type_id, self.data));
        (functions.serialize_into)(self.data)
    }
}