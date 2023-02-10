use std::mem;

#[derive(Debug)]
//struct List with generic type T
pub struct List<T> {
    head: Link<T>,
}

#[derive(Debug)]
//struct Node with generic type T
struct Node<T> {
    elem: T,
    next: Link<T>,
}
type Link<T> = Option<Box<Node<T>>>;


//impl List with generic type T
impl <T> List<T> {
    //initialize the list
    pub fn new() -> Self {
        List { head: None }
    }

    //push new node to the front of the list
    pub fn push(&mut self, elem: T) {
        let new_element = Box::new(
            Node {
                elem,
                next: self.head.take(),
            }
        );
        self.head = Some(new_element);
    }

/*    //pop the first node from the list
    pub fn pop(&mut self) -> Option<T> {
        let mut old_head = self.head.take();
        match old_head {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }*/

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    //mutable version of peek
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
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
        assert_eq!(list.peek(), Some(&3));

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
