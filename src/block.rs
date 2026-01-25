use std::borrow::{Borrow, BorrowMut};

#[repr(C)]
pub struct BlockId;
impl BlockId {
    const Air: u32 = 0;
    const Stone: u32 = 1;
    const OakLog: u32 = 2;
    const BlockWithName: u32 = 60000;
}


#[repr(C)]
pub struct BlockBase {
    type_id: u32,
    block_state: u32
}

#[repr(C)]
pub struct Block {
    base: *mut BlockBase
}
impl Block {
    pub fn new<T>(a: Box<T>) -> Block {
        Block {
            base: Box::into_raw(a) as *mut BlockBase
        }
    }
    pub fn new_desirialize_from(type_id_raw: u32, state_raw: u32, mut blockentity_raw: &[u8]) -> Block {
        match type_id_raw {
            BlockId::Air => Block::new(Box::new(Air::new_from_rmpv(state_raw, rmpv::decode::read_value_ref(&mut blockentity_raw).unwrap_or(rmpv::ValueRef::Nil)))),
            BlockId::Stone => Block::new(Box::new(Stone::new_from_rmpv(state_raw, rmpv::decode::read_value_ref(&mut blockentity_raw).unwrap_or(rmpv::ValueRef::Nil)))),
            BlockId::OakLog => Block::new(Box::new(OakLog::new_from_rmpv(state_raw, rmpv::decode::read_value_ref(&mut blockentity_raw).unwrap_or(rmpv::ValueRef::Nil)))),
            BlockId::BlockWithName => Block::new(Box::new(BlockWithName::new_from_rmpv(state_raw, rmpv::decode::read_value_ref(&mut blockentity_raw).unwrap_or(rmpv::ValueRef::Nil)))),
            _ => Block::new(Box::new(Air::new_from_rmpv(state_raw, rmpv::decode::read_value_ref(&mut blockentity_raw).unwrap_or(rmpv::ValueRef::Nil))))
        }
    }
    pub fn serialize(s: &BlockBase) -> Vec<u8> {
        match s.type_id {
            BlockId::Air => unsafe{std::mem::transmute::<&BlockBase, &Air>(s)}.serialize(),
            BlockId::Stone => unsafe{std::mem::transmute::<&BlockBase, &Stone>(s)}.serialize(),
            BlockId::OakLog => unsafe{std::mem::transmute::<&BlockBase, &OakLog>(s)}.serialize(),
            BlockId::BlockWithName => unsafe{std::mem::transmute::<&BlockBase, &BlockWithName>(s)}.serialize(),
            _ => unsafe{std::mem::transmute::<&BlockBase, &Air>(s)}.serialize()
        }
    }
}
impl Drop for Block {
    fn drop(&mut self) {
        unsafe {
            match (*self.base).type_id {
                BlockId::Air => {
                    Box::from_raw(self.base as *mut Air);
                },
                BlockId::Stone => {
                    Box::from_raw(self.base as *mut Stone);
                },
                BlockId::OakLog => {
                    Box::from_raw(self.base as *mut OakLog);
                },
                BlockId::BlockWithName => {
                    Box::from_raw(self.base as *mut BlockWithName);
                }
                _ => {
                    // Handle other block types or do nothing
                }
            }
        }
    }
}
impl Borrow<BlockBase> for Block {
    fn borrow(&self) -> &BlockBase {
        unsafe {std::mem::transmute(self.base)}
    }
}
impl BorrowMut<BlockBase> for Block {
    fn borrow_mut(&mut self) -> &mut BlockBase {
        unsafe {std::mem::transmute(self.base)}
    }
}

pub type BlockUpdateFunction = fn(&mut BlockBase, &mut crate::world::World);


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
        rmp::encode::write_map_len(&mut buf, 1).unwrap();
        rmp::encode::write_str(&mut buf, "name");
        rmp::encode::write_str(&mut buf, &self.name);
        buf.into_vec()
    }
}
