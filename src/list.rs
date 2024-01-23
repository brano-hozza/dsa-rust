#[derive(Debug)]
pub struct Node {
    pub data: i32,
    pub next: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct List {
    head: Option<Box<Node>>,
    length: u32,
}

impl List {
    pub fn new() -> Self {
        List {
            head: None,
            length: 0,
        }
    }

    pub fn push(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data: data,
            next: self.head.take(),
        });

        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.length -= 1;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&i32> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn peek_mut(&mut self) -> Option<&mut i32> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    pub fn get(&self, index: u32) -> Option<&i32> {
        if index >= self.length {
            return None;
        }

        let mut current = &self.head;
        for _ in 0..index {
            current = &current.as_ref().unwrap().next;
        }

        current.as_ref().map(|node| &node.data)
    }

    pub fn get_mut(&mut self, index: u32) -> Option<&mut i32> {
        if index >= self.length {
            return None;
        }

        let mut current = &mut self.head;
        for _ in 0..index {
            current = &mut current.as_mut().unwrap().next;
        }

        current.as_mut().map(|node| &mut node.data)
    }
}

impl Iterator for List {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[cfg(test)]
mod tests {

    fn setup() -> super::List {
        let mut list = super::List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);
        list
    }

    #[test]
    fn test_push() {
        let list = setup();
        assert_eq!(list.length, 5);
    }

    #[test]
    fn test_pop() {
        let mut list = setup();
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut list = setup();
        assert_eq!(list.peek(), Some(&5));
        assert_eq!(list.peek(), Some(&5));
        list.pop();
        assert_eq!(list.peek(), Some(&4));
    }

    #[test]
    fn test_peek_mut() {
        let mut list = setup();
        assert_eq!(list.peek_mut(), Some(&mut 5));
        assert_eq!(list.peek_mut(), Some(&mut 5));
        let item = list.peek_mut().unwrap();
        *item = 42;
        assert_eq!(list.peek(), Some(&42));
    }

    #[test]
    fn test_get() {
        let list = setup();
        assert_eq!(list.get(0), Some(&5));
        assert_eq!(list.get(1), Some(&4));
        assert_eq!(list.get(2), Some(&3));
        assert_eq!(list.get(3), Some(&2));
        assert_eq!(list.get(4), Some(&1));
        assert_eq!(list.get(5), None);
    }

    #[test]
    fn test_get_mut() {
        let mut list = setup();
        assert_eq!(list.get_mut(0), Some(&mut 5));
        let item = list.get_mut(0).unwrap();
        *item = 42;
        assert_eq!(list.get(0), Some(&42));
    }
}
