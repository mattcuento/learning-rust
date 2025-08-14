use std::mem;

// pub publishes a definition outside the module
// pub enum List {
//     Empty,
//     ElemThenEmpty(i32),
//     ElemThenNotEmpty(i32, Box<List>),
// }

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>), // todo: What does More do/is it a keyword?
}

pub struct List { // pub used for visibility - avoid leaking internals
    head: Link,
}


// unimplemented!() // placeholder for unimplemented code - ! indicates a macro
//
impl List { // impl is like a constructor?
    pub fn new() -> Self { // Self refers to the type referred to by impl
        List { head: Link::Empty } // init logic
    }

    pub fn push(&mut self, elem: i32) { // &mut self allows modifying the instance
        let new_node = Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty), // replace head with Empty 
        };
        self.head = Link::More(Box::new(new_node)); // wrap in Box to allocate on heap
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) { // replace head with Empty
            Link::Empty => return None, 
            Link::More(node) => {
                self.head = node.next;
                return Some(node.elem);
            }
        };
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty); 
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty); 
        }
    }
}

// ownership
// self - value
// &self - shared reference
// &mut self - mutable reference

#[cfg(test)]
mod test {
    use super::List; // use super to access the parent module

    #[test] // what is this syntax?
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

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
