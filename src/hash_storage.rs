use storage::*;
use std::collections::HashMap;
use std::ops::Deref;

pub struct HashStorage {
    elements: Box<HashMap<u16, Vec<StorageItem>>>,
}

impl Storage for HashStorage {
    fn new() -> Self {
        HashStorage {
            elements: Box::new(HashMap::new())
        }
    }

    fn push(&mut self, priority: u16, payload: Box<Vec<u8>>) {
        let storage_item = StorageItem {
            priority: priority,
            data: payload,
        };
        match self.elements.contains_key(&priority) {
            true => self.elements.get_mut(&priority).unwrap().push(storage_item),
            false => { self.elements.insert(priority, vec![storage_item]); }
        };
    }

    fn pop(&mut self) -> Option<StorageItem> {
        match self.max_priority() {
            Some(priority) => {
                let mut priority_elements = self.elements.get_mut(&priority).unwrap();
                Some(priority_elements.remove(0))
            }
            None => None,
        }
    }

    fn max_priority(&self) -> Option<u16> {
        let mut max_priority: u16 = 0;
        let mut found = false;

        for (priority, elements) in self.elements.iter() {
            if !elements.is_empty() {
                found = true;
                if *priority > max_priority { max_priority = *priority }
            }
        }

        match found {
            true => Some(max_priority),
            false => None
        }
    }

    fn dump(&self) {}
    fn load(&mut self) {}

    fn clear(&mut self) {
        self.elements.clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works_for_no_elements() {
        let mut instance = HashStorage::new();
        let opt = instance.pop();
        assert_eq!(opt.is_none(), true);
    }

    #[test]
    fn it_works_for_a_single_element() {
        let mut instance = HashStorage::new();
        instance.push(10, Box::new(vec![2]));
        let opt = instance.pop().unwrap();
        assert_eq!(opt.priority, 10);
        assert_eq!(opt.data[0], 2);
        let opt2 = instance.pop();
        assert_eq!(opt2.is_none(), true);
    }

    #[test]
    fn it_works_for_2_elements() {
        let mut instance = HashStorage::new();
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
        let mut instance = HashStorage::new();
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
        let mut instance = HashStorage::new();

        instance.push(10, Box::new(vec![1]));
        instance.push(2, Box::new(vec![1]));
        instance.push(11, Box::new(vec![1]));
        instance.push(3, Box::new(vec![1]));
        instance.push(0, Box::new(vec![1]));
        assert_eq!(instance.max_priority(), Some(11));
    }

    #[test]
    fn it_clears() {
        let mut instance = HashStorage::new();
        instance.push(10, Box::new(vec![1]));
        instance.clear();
        let opt = instance.pop();
        assert_eq!(opt.is_none(), true);
    }
}
