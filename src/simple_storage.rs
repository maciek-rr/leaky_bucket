extern crate bincode;

use bincode::{serialize, deserialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

use storage::*;

const DUMP_NAME: &'static str = "dump.bin";

pub struct SimpleStorage {
    elements: Box<Vec<StorageItem>>,
}

impl SimpleStorage {
    pub fn new() -> Self {
        SimpleStorage {
            elements: Box::new(vec![]),
        }
    }
}

impl Storage for SimpleStorage {
    fn push(&mut self, priority: u16, payload: Box<Vec<u8>>) {
        self.elements.push(StorageItem {
            priority: priority,
            data: payload,
        });
    }

    fn pop(&mut self) -> Option<StorageItem> {
        let mp = self.max_priority();
        match mp {
            None => None,
            Some(priority) => {
                let item_index = self.elements
                    .iter()
                    .position(|elem| elem.priority == priority)
                    .unwrap();
                let elem = self.elements.swap_remove(item_index);
                Some(elem)
            }
        }
    }

    fn max_priority(&self) -> Option<u16> {
        if self.elements.is_empty() {
            return None;
        };

        let mut max_priority: u16 = 0;
        self.elements.iter().for_each(|storage_item| {
            if storage_item.priority > max_priority {
                max_priority = storage_item.priority
            }
        });
        Some(max_priority)
    }

    fn dump(&self) {
        let serialized = serialize(&self.elements).unwrap();
        let mut dump_file = File::create(DUMP_NAME).unwrap();
        dump_file.write(&serialized).expect("Failed to dump the data");
    }

    fn load(&mut self) {
        let file = OpenOptions::new().read(true).open(DUMP_NAME);
        match file {
            Ok(mut open_file) => {
                let mut buf: &mut Vec<u8> = &mut vec!();
                let read_bytes = open_file.read_to_end(&mut buf).unwrap();
                println!("Read {} bytes from the dump file.", read_bytes);
                self.elements = deserialize(&buf[..]).unwrap();
            }
            Err(e) => {
                println!("No dump file, or error opening dump file file ({:?}", e);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut instance = SimpleStorage::new();
        instance.push(10, Box::new(vec![2]));
        instance.push(2, Box::new(vec![1]));
        let opt = instance.pop().unwrap();
        assert_eq!(opt.priority, 10);
        assert_eq!(opt.data[0], 2);
        let opt2 = instance.pop().unwrap();
        assert_eq!(opt2.priority, 2);
        assert_eq!(opt2.data[0], 1);
        let opt3 = instance.pop();
        assert_eq!(opt3.is_none(), true);
    }

    #[test]
    fn it_returns_correct_max_priority() {
        let mut instance = SimpleStorage::new();
        instance.push(10, Box::new(vec![1]));
        instance.push(2, Box::new(vec![1]));
        instance.push(11, Box::new(vec![1]));
        instance.push(3, Box::new(vec![1]));
        instance.push(0, Box::new(vec![1]));
        assert_eq!(instance.max_priority(), Some(11));
    }
}
