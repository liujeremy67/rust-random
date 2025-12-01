// linked list

struct Node<T> {
    val: T,
    next: Link<T>,
}

// both to prevent types of unknown size (a linked list) and to prevent uncertainty (potentially none)
type Link<T> = Option<Box<Node<T>>>;

pub struct LinkedList<T> {
    head: Link<T>,
}

// constructor
// impl implements method for struct
impl<T> LinkedList<T> {
    pub fn new() -> Self { // -> Self returns same type as implementation
        Self {             // create a struct literal
            head: None,
        }
    }
}

// get front
impl<T> LinkedList<T> {
    pub fn get(&self) -> Option<&T>{ // list, val not modified (no mut)
        match self.head.as_ref() {
            Some(node) => Some(&node.val),
            None => None,
        }   
    }
}

// push front
// Some part of Option (the first T enum)
// req since we defined it as Link, which is Option
impl<T> LinkedList<T> {
    pub fn push(&mut self, val: T) {
        // .take() sets head to 0, returns original value
        let new_node = Box::new(Node { val, next: self.head.take() });
        self.head = Some(new_node);
    }
}

// pop front
impl<T> LinkedList<T> {
    pub fn pop(&mut self) -> Option<T> {
        if let Some(prev_head) = self.head.take() { // head is wrapped in Option
            self.head = prev_head.next; // next already wrapped, does not need Some
            Some(prev_head.val)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_works() {
        let mut list = LinkedList::new();
        assert_eq!(list.get(), None);

        list.push(10);
        assert_eq!(list.get(), Some(&10));

        list.push(20);
        assert_eq!(list.get(), Some(&20));

        assert_eq!(list.pop(), Some(20));
        assert_eq!(list.get(), Some(&10));

        assert_eq!(list.pop(), Some(10));
        assert_eq!(list.get(), None);
    }
}