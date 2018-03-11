pub struct StorageItem {
    pub priority: u16,
    pub data: Box<Vec<u8>>,
}

pub struct Storage {
    elements: Box<Vec<StorageItem>>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            elements: Box::new(vec![]),
        }
    }

    pub fn push(&mut self, priority: u16, payload: Box<Vec<u8>>) {
        self.elements.push(StorageItem {
            priority: priority,
            data: payload,
        })
    }

    pub fn pop(&mut self) -> Option<StorageItem> {
        let mp = self.max_priority();
        match mp {
            None => None,
            Some(priority) => {
                let item_index = self.elements
                    .iter()
                    .position(|elem| elem.priority == priority)
                    .unwrap();

                Some(self.elements.remove(item_index))
            }
        }
    }

    pub fn max_priority(&self) -> Option<u16> {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut instance = Storage::new();
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
        let mut instance = Storage::new();
        instance.push(10, Box::new(vec![1]));
        instance.push(2, Box::new(vec![1]));
        instance.push(11, Box::new(vec![1]));
        instance.push(3, Box::new(vec![1]));
        instance.push(0, Box::new(vec![1]));
        assert_eq!(instance.max_priority(), Some(11));
    }
}
