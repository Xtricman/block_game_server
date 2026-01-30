use core::panic;
use std::ops::Deref;

#[repr(C)]
pub struct BlockId;
impl BlockId {
    pub const Air: u32 = 0;
    pub const Stone: u32 = 1;
    pub const OakLog: u32 = 2;
    pub const BlockWithName: u32 = 60000;
}


#[repr(C)]
pub struct BlockBase {
    pub type_id: u32,
    pub block_state: u32
}

#[repr(C)]
pub struct Block {
    pub pointer: *mut BlockBase
}
impl Block {
    pub fn new<T>(a: T) -> Block {
        Block {
            pointer: Box::into_raw(Box::<T>::new(a)) as *mut BlockBase
        }
    }
    pub fn new_desirialize_from(type_id_raw: u32, state_raw: u32, mut blockentity_raw: &[u8]) -> Block {
        let blockentity= rmpv::decode::read_value_ref(&mut blockentity_raw).unwrap_or(rmpv::ValueRef::Nil);
        match type_id_raw {
            BlockId::Air => Block::new(Air::new_from_rmpv(state_raw, blockentity)),
            BlockId::Stone => Block::new(Stone::new_from_rmpv(state_raw, blockentity)),
            BlockId::OakLog => Block::new(OakLog::new_from_rmpv(state_raw, blockentity)),
            BlockId::BlockWithName => Block::new(BlockWithName::new_from_rmpv(state_raw, blockentity)),
            _ => Block::new(Air::new_from_rmpv(state_raw, blockentity))
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        let tmp = unsafe{std::ptr::read(self.pointer)};
        let d = tmp.type_id;
        std::mem::forget(tmp);
        match d {
            BlockId::Air => unsafe{std::mem::transmute_copy::<Block, &Air>(self)}.serialize(),
            BlockId::Stone => unsafe{std::mem::transmute_copy::<Block, &Stone>(self)}.serialize(),
            BlockId::OakLog => unsafe{std::mem::transmute_copy::<Block, &OakLog>(self)}.serialize(),
            BlockId::BlockWithName => unsafe{std::mem::transmute_copy::<Block, &BlockWithName>(self)}.serialize(),
            _ => panic!("Serializing an unknown type_id!")
        }
    }
}
impl Drop for Block {
    fn drop(&mut self) {
        unsafe {
            let tmp = std::ptr::read(self.pointer);
            let d = tmp.type_id;
            std::mem::forget(tmp);
            match d {
                BlockId::Air => {
                    drop(Box::from_raw(self.pointer as *mut Air));
                },
                BlockId::Stone => {
                    drop(Box::from_raw(self.pointer as *mut Stone));
                },
                BlockId::OakLog => {
                    drop(Box::from_raw(self.pointer as *mut OakLog));
                },
                BlockId::BlockWithName => {
                    drop(Box::from_raw(self.pointer as *mut BlockWithName));
                }
                _ => {
                    // Handle other block types or do nothing
                }
            }
        }
    }
}

pub type BlockUpdateFunction = fn(&mut Block, &mut crate::world::World);





#[repr(C)]
pub struct Air {
    base: BlockBase
}
impl Air {
    fn new_from_rmpv(state: u32, blockentity: rmpv::ValueRef) -> Self {
        Air {
            base: {
                BlockBase { type_id: BlockId::Air, block_state: 0 }
            }
        }
    }
    fn serialize(&self) -> Vec<u8> {
        Vec::new()
    }
}




#[repr(C)]
pub struct Stone {
    base: BlockBase
}
impl Stone {
    fn new_from_rmpv(state: u32, blockentity: rmpv::ValueRef) -> Self {
        Stone {
            base: BlockBase {
                type_id: BlockId::Stone,
                block_state: 0
            }
        }
    }
    fn serialize(&self) -> Vec<u8> {
        Vec::new()
    }
}




#[repr(C)]
pub struct OakLog {
    base: BlockBase
}
impl OakLog {
    const LOG_X_AXIS: u32 = 0;
    const LOG_Y_AXIS: u32 = 1;
    const LOG_Z_AXIS: u32 = 2;
    fn new_from_rmpv(state: u32, blockentity: rmpv::ValueRef) -> Self {
        OakLog { 
        base: BlockBase {
            type_id: BlockId::OakLog,
            block_state: if state<=2 {state} else {0}
            }
        }
    }
    fn serialize(&self) -> Vec<u8> {
        Vec::new()
    }
}




#[repr(C)]
pub struct BlockWithName {
    base: BlockBase,
    name: String
}
impl BlockWithName {
    fn new_from_rmpv(state: u32, blockentity: rmpv::ValueRef) -> Self {
        let mut r = BlockWithName {
            base: BlockBase {
                type_id: BlockId::BlockWithName,
                block_state: 0
            },
            name: String::new()
        };
        match blockentity {
            rmpv::ValueRef::Map(map) => {
                for (key, value) in map {
                    if let rmpv::ValueRef::String(k) = key {
                        if k.as_str().unwrap_or_default() == "name" {
                            if let rmpv::ValueRef::String(v) = value {
                                r.name = v.into_string().unwrap_or_default();
                            }
                        }
                    };
                }
            }
            _ => ()
        };
        r
    }
    fn serialize(&self) -> Vec<u8> {
        let mut buf = rmp::encode::ByteBuf::new();
        rmp::encode::write_map_len(&mut buf, 1);
        rmp::encode::write_str(&mut buf, "name");
        rmp::encode::write_str(&mut buf, &self.name);
        let r = buf.into_vec();
        r
    }
}
