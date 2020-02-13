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
    const ID_MODULE_INFO: IDModuleInfo = IDModuleInfo {
        tags: Self::TAG_LIST,
        block_functions: Self::BlockValue::FUNCTIONS,
        entity_functions: Self::EntityValue::FUNCTIONS,
        item_functions: Self::ItemValue::FUNCTIONS,
    }; //模块转换为IDModuleInfo
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



///每个NBT数据类型皆应实现此Trait
pub trait Value: std::fmt::Debug+Eq+Clone {
    fn deserialize_from(src: &[u8]) -> *mut ();//反序列化函数，必须返回一个非null且按照真实类型对其的指针
    fn serialize_into(dynamic_value: *const ()) -> Vec<u8>;//不允许失败，因为内存中的DynamicValue的数据一定处于正确的状态
    fn drop(dynamic_value: *mut ());//析构函数
    const FUNCTIONS: Option<Functions> = if !type_eq::<Self, ()>() {Some(Functions{
        deserialize_from: Self::deserialize_from,
        serialize_into: Self::serialize_into,
        drop: Self::drop,
    })} else {None}; //NBT数据类型转换为Functions
}
impl Value for () {
    fn deserialize_from(_src: &[u8]) -> *mut () {unreachable!("IMPOSSIBLE TO TO CALL TYPE () AS VALUE")}
    fn serialize_into(_dynamic_value: *const ()) -> Vec<u8> {unreachable!("IMPOSSIBLE TO TO CALL TYPE () AS VALUE")}
    fn drop(_dynamic_value: *mut ()) {unreachable!("IMPOSSIBLE TO TO CALL TYPE () AS VALUE")}
}

///每个NBT数据类型的析构，反序列化，序列化函数，只需要实现数据读写即可
#[derive(Copy, Clone)]
pub struct Functions {
    drop: fn(*mut ()),
    deserialize_from: fn(&[u8]) -> *mut (),
    serialize_into: fn(*const ()) -> Vec<u8>,
}



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


///方块、实体、物品都实现这个Trait
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