mod block;
mod world;

use std::collections::HashMap;

fn main() {
    let mut block_update_funtions: HashMap<block::BlockId, block::BlockUpdateFunction> = HashMap::new();
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct BlockPostion {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

type EntityUUID = [u8;16];

struct EnitiyPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct StructureCoordinate {
    pub x: u64,
    pub y: u64,
    pub z: u64,
    pub x_length: u64,
    pub y_length: u64,
    pub z_length: u64,
}

struct PlayerState;

struct Event;

struct MapConnection {
    conn: rusqlite::Connection,
    players: std::collections::HashMap<EntityUUID, PlayerState>,
    blocks: std::collections::HashMap<BlockPostion, block::Block>,
    entities: std::collections::HashMap<EntityUUID, (EnitiyPosition, ())>,
    structures: std::collections::HashMap<StructureCoordinate, ()>,
    globals: std::collections::HashMap<String, Vec<u8>>,
    event_queue: std::collections::VecDeque<Event>,
}

impl MapConnection {
    fn new(file_name: &str) -> Self {
        MapConnection {
            conn: rusqlite::Connection::open(file_name).expect("Open Fail!"),
            players: std::collections::HashMap::new(),
            blocks: std::collections::HashMap::new(),
            entities: std::collections::HashMap::new(),
            structures: std::collections::HashMap::new(),
            globals: std::collections::HashMap::new(),
            event_queue: std::collections::VecDeque::new()
        }
    }

    fn step(&mut self) {

    }

    fn save(&mut self) {

    }
}