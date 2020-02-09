#![feature(const_fn)]
#![feature(const_if_match)]

mod dynamic_types;

fn main() {
    println!("{:?}", dynamic_types::filter_ids_by_tag(dynamic_types::Tag::CanBeBurn));
    println!("{:?}", dynamic_types::filter_ids_by_tag(dynamic_types::Tag::RedStonePowerSource));
    println!("{:?}", dynamic_types::filter_ids_by_tag(dynamic_types::Tag::Wood));
    println!("{:?}", dynamic_types::filter_ids_by_tag(dynamic_types::Tag::Stone));
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct BlockPostion {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

type EntityUUID = [u8;16];

struct EnitiyPosition {
    pub x: fixed::types::U60F4,
    pub y: fixed::types::U60F4,
    pub z: fixed::types::U60F4,
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
    entities: std::collections::HashMap<EntityUUID, (EnitiyPosition, dynamic_types::DynamicValue)>,
    blocks: std::collections::HashMap<BlockPostion, (dynamic_types::DynamicValue, u8)>,
    biomes: std::collections::HashMap<StructureCoordinate, dynamic_types::TypeID<'static>>,
    structures: std::collections::HashMap<StructureCoordinate, dynamic_types::DynamicValue>,
    event_queue: std::collections::VecDeque<Event>,
}

impl MapConnection {
    fn new(file_name: &str) -> Self {
        MapConnection {
            conn: rusqlite::Connection::open(file_name).expect("Open Fail!"),
            players: std::collections::HashMap::new(),
            entities: std::collections::HashMap::new(),
            blocks: std::collections::HashMap::new(),
            biomes: std::collections::HashMap::new(),
            structures: std::collections::HashMap::new(),
            event_queue: std::collections::VecDeque::new()
        }
    }

    fn step(&mut self) {

    }

    fn save(&mut self) {

    }
}