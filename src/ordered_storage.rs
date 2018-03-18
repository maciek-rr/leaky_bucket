extern crate bincode;

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

impl Storage for OrderedStorage {
    fn push(&mut self, priority: u16, payload: Box<Vec<u8>>) {
        let mut index = 0;
        {
            let lesser_priority_element = self.elements.iter().find(|storage_item| {
                index += 1;
                storage_item.priority < priority
            });

            match lesser_priority_element {
                Some(elem) => index -= 1,
                None => {}
            }
        }

        self.elements.insert(
            index,
            StorageItem {
                priority: priority,
                data: payload,
            },
        );
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
            Some(element) => Some(element.priority),
            None => None,
        }
    }

    fn dump(&self) {}
    fn load(&mut self) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works_for_no_elements() {
        let mut instance = OrderedStorage::new();
        let opt = instance.pop();
        assert_eq!(opt.is_none(), true);
    }

    #[test]
    fn it_works_for_a_single_element() {
        let mut instance = OrderedStorage::new();
        instance.push(10, Box::new(vec![2]));
        let opt = instance.pop().unwrap();
        assert_eq!(opt.priority, 10);
        assert_eq!(opt.data[0], 2);
        let opt2 = instance.pop();
        assert_eq!(opt2.is_none(), true);
    }

    #[test]
    fn it_works_for_2_elements() {
        let mut instance = OrderedStorage::new();
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
    fn it_preserves_order_for_the_same_priority() {
        let mut instance = OrderedStorage::new();
        instance.push(2, Box::new(vec![1]));
        instance.push(2, Box::new(vec![2]));
        instance.push(2, Box::new(vec![3]));
        instance.push(4, Box::new(vec![4]));

        let opt = instance.pop().unwrap();
        assert_eq!(opt.priority, 4);
        assert_eq!(opt.data[0], 4);

        let opt2 = instance.pop().unwrap();
        assert_eq!(opt2.priority, 2);
        assert_eq!(opt2.data[0], 1);

        let opt3 = instance.pop().unwrap();
        assert_eq!(opt3.priority, 2);
        assert_eq!(opt3.data[0], 2);

        let opt4 = instance.pop().unwrap();
        assert_eq!(opt4.priority, 2);
        assert_eq!(opt4.data[0], 3);
    }

    #[test]
    fn it_returns_correct_max_priority() {
        let mut instance = OrderedStorage::new();
        instance.push(10, Box::new(vec![1]));
        instance.push(2, Box::new(vec![1]));
        instance.push(11, Box::new(vec![1]));
        instance.push(3, Box::new(vec![1]));
        instance.push(0, Box::new(vec![1]));
        assert_eq!(instance.max_priority(), Some(11));
    }
}
