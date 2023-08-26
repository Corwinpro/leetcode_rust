#[derive(Debug)]
pub struct LinkedList<T> {
    value: T,
    next: Option<Box<LinkedList<T>>>,
}

impl<T> LinkedList<T> {
    pub fn push_front(&mut self, data: T) {
        // Replace the self's value with `data`, keeping the old value
        let old_value = std::mem::replace(&mut self.value, data);
        // Replace the self's next with `None`, keeping the old next
        // Equivalent to self.next.take()
        let old_ref = std::mem::replace(&mut self.next, None);
        // Create a self's doppelganger as it was before the call
        let new = LinkedList {
            value: old_value,
            next: old_ref,
        };
        // Point the self to the doppelganger
        self.next = Some(Box::new(new));
    }

    pub fn push_back(&mut self, data: T) {
        match self.next.as_mut() {
            Some(boxed_value) => boxed_value.push_back(data),
            None => {
                self.next = Some(Box::new(LinkedList {
                    value: data,
                    next: None,
                }));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ll() {
        let mut list = LinkedList {
            value: 1,
            next: None,
        };
        println!("{:?}", list);
        list.push_front(2);
        println!("{:?}", list);

        list.push_front(3);
        println!("{:?}", list);

        list.push_back(4);
        println!("{:?}", list);
    }
}
