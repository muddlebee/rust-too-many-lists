use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};

pub struct List<T> {
	head: Link<T>,
	tail: Link<T>,
	length: i32
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
	elem: T,
	prev: Link<T>,
	next: Link<T>,
}

impl<T> Node<T>{
	fn new(elem: T) -> Rc<RefCell<Self>>{
		Rc::new(RefCell::new(Node {
			elem: elem,
			prev: None,
			next: None,
		}))
	}
}

impl<T> List<T> {
	fn new() -> Self{
		List { head:None, tail: None , length: 0 }
	} 

	fn push_front<>(& mut self, elem: T){
		let new_head = Node::new(elem);

		match self.head.take() {
			Some(elem) => 
			{
				elem.borrow_mut().prev = Some(new_head.clone());
				new_head.borrow_mut().next = Some(elem);
				self.head = Some(new_head);

			}, 
			None => {
				self.head = Some(new_head.clone());
				self.tail = Some(new_head);
			}
		}
		self.length += 1;
	}
	
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
		let mut list: List<i32> = List::new();
		list.push_front(2);
		list.push_front(3);
		eprintln!("list = {:?}", list.length);
	}

}