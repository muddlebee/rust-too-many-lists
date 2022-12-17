use std::mem;

#[derive(Debug)]
pub struct List {
    head: Link,
}
#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}
#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, item: i32) {
        let new_node = Box::new(Node {
            elem: item,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let result;

        //move self.head --> old_head by replacing the original reference with Link::Empty
        let old_head = mem::replace(&mut self.head, Link::Empty);
        match old_head {
            Link::Empty => {
                result = None;
            }
            Link::More(node) => {
                self.head = node.next;
                result = Some(node.elem);
            }
        };
        result
    }
}

impl Drop for List {
    fn drop(&mut self) {}
}

mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);
        println!("{:?}", list);


        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}