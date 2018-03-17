extern crate bincode;

use bincode::{serialize, deserialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use storage::*;

const DUMP_NAME: &'static str = "dump.bin";

pub struct OrderedStorage {
    elements: Box<Vec<StorageItem>>,
}

impl OrderedStorage {
    pub fn new() -> Self {
        OrderedStorage {
            elements: Box::new(vec![]),
        }
    }
}

// elems = [{priority:3}, {priority: 2}, {priority: 1}]
// elems.find { |el| el[:priority] < 2 }


impl Storage for OrderedStorage {
    fn push(&mut self, priority: u16, payload: Box<Vec<u8>>) {
        let mut index = 0;
        self.elements.iter().find(|storage_item| {
            index += 1;
            storage_item.priority < priority
        });
        if index > 0 { index -= 1 }

        self.elements.insert(index, StorageItem {
            priority: priority,
            data: payload
        });
    }

    fn pop(&mut self) -> Option<StorageItem> {
        if self.elements.is_empty() {
            None
        } else {
            Some(self.elements.remove(0))
        }
    }

    fn max_priority(&self) -> Option<u16> {
        match self.elements.first() {
            Some(element) => { Some(element.priority) }
            None => { None }
        }
    }

    fn dump(&self) {}
    fn load(&mut self) {}
}