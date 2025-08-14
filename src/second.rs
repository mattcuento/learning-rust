type Link<T> = Option<Box<Node<T>>>; 

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct List<T> { // pub used for visibility - avoid leaking internals
    head: Link<T>,
}

// unimplemented!() // placeholder for unimplemented code - ! indicates a macro
//
impl<T> List<T> { // impl is like a constructor?
    pub fn new() -> Self { // Self refers to the type referred to by impl
        List { head: None } // init logic
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn peek(&self) -> Option<&T> { // &self is a shared reference to the instance
        self.head.as_ref().map(|node| &node.elem) // as_ref() converts Option<Box<Node<T>>> to
        // Option<&Node<T>>
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> { // &mut self is a mutable reference to the instance
        self.head.as_mut().map(|node| &mut node.elem) // as_mut() converts Option<Box<Node<T>>> to
        // Option<&mut Node<T>>
    }

    pub fn push(&mut self, elem: T) { // &mut self allows modifying the instance
        let new_node = Node {
            elem,
            next: self.head.take(), // take() replaces the value with None and returns the old
            // value
        };
        self.head = Some(Box::new(new_node)); // wrap in Box to allocate on heap
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next; // update head to the next node
            node.elem // return the element of the current node
        })
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T; // associated type for the iterator

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop() // call pop on the List to get the next element
    }
}

impl<T> Drop for List<T> { 
    fn drop(&mut self) {
        let mut cur_link = self.head.take(); // take() replaces head with None and returns the old
        // value
        while let Some(_boxed_node) = cur_link {
            cur_link = self.head.take(); // replace head with None and return the old value
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

    #[test]
    fn peeks() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| *value += 1); // modify the value pointed to by peek_mut
        assert_eq!(list.peek(), Some(&4)); // check if the value was modified
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}
