///空实现
use super::Value;
use super::IDModule;
use super::Tag;

pub enum Module {}
impl IDModule for Module {
    const TAG_LIST: &'static [Tag] = &[];
    type BlockValue = ();
    type EntityValue = ();
    type ItemValue = ();
}