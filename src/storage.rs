extern crate serde;

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageItem {
    pub priority: u16,
    pub data: Box<Vec<u8>>,
}

pub trait Storage {
    fn push(&mut self, priority: u16, payload: Box<Vec<u8>>);
    fn pop(&mut self) -> Option<StorageItem>;
    fn dump(&self);
    fn load(&mut self);
    fn max_priority(&self) -> Option<u16>;
}
