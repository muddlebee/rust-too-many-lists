use std::mem;
use first_test::Link::Empty;

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
    //initialize the list
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    //push new node to the front of the list
    pub fn push(&mut self, elem: i32) {
        let new_element = Box::new(
            Node {
                elem,
                next: mem::replace(&mut self.head, Empty),
            }
        );
        self.head = Link::More(new_element);
    }


    pub fn pop(&mut self) -> Option<i32> {
        let mut old_head = mem::replace(&mut self.head, Empty);
        match old_head {
            Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
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
        print!("{:?}", list);
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
