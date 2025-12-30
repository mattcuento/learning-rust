type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
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


pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() } // as_ref() converts Option<Box<Node<T>>> to
        // Option<&Node<T>>
    }
}

// lifetimes like 'a indicate how long references are valid. This can be necessary for types and
// methods
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem // return a reference to the element
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() } // as_mut() converts Option<Box<Node<T>>> to
        // Option<&mut Node<T>>
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

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

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter_mut = list.iter_mut();
        assert_eq!(iter_mut.next(), Some(&mut 3));
        assert_eq!(iter_mut.next(), Some(&mut 2));
        assert_eq!(iter_mut.next(), Some(&mut 1));
        assert_eq!(iter_mut.next(), None);
    }
}

pub fn main() {
    let mut list = List::new();

    println!("Checking empty list behaves right");
    println!("Pop from empty list: {:?}", list.pop());

    println!("Populating list");
    list.push(1);
    list.push(2);
    list.push(3);

    println!("Check normal removal");
    println!("Pop from list: {:?}", list.pop());
    println!("Pop from list: {:?}", list.pop());


    println!("Push some more just to make sure nothing's corrupted");
    list.push(4);
    list.push(5);

    println!("Check normal removal");
    println!("Pop from list: {:?}", list.pop());
    println!("Pop from list: {:?}", list.pop());

    println!("Check exhaustion");
    println!("Pop from list: {:?}", list.pop());
    println!("Pop from list: {:?}", list.pop());

    println!("\n--- Testing peeks ---");
    let mut list2 = List::new();
    println!("Peek on empty list: {:?}", list2.peek());
    list2.push(1); list2.push(2); list2.push(3);

    println!("Peek on list with 3, 2, 1: {:?}", list2.peek());

    if let Some(value) = list2.peek_mut() {
        *value += 1;
    }
    println!("Peek after adding 1 to head: {:?}", list2.peek());

    println!("\n--- Testing into_iter ---");
    let mut list3 = List::new();
    list3.push(1); list3.push(2); list3.push(3);
    let mut iter = list3.into_iter();
    println!("into_iter next(): {:?}", iter.next());
    println!("into_iter next(): {:?}", iter.next());
    println!("into_iter next(): {:?}", iter.next());
    println!("into_iter next(): {:?}", iter.next());

    println!("\n--- Testing iter ---");
    let mut list4 = List::new();
    list4.push(1); list4.push(2); list4.push(3);
    let mut iter2 = list4.iter();
    println!("iter next(): {:?}", iter2.next());
    println!("iter next(): {:?}", iter2.next());
    println!("iter next(): {:?}", iter2.next());
    println!("iter next(): {:?}", iter2.next());

    println!("\n--- Testing iter_mut ---");
    let mut list5 = List::new();
    list5.push(1); list5.push(2); list5.push(3);
    let mut iter_mut = list5.iter_mut();
    println!("iter_mut next(): {:?}", iter_mut.next());
    println!("iter_mut next(): {:?}", iter_mut.next());
    println!("iter_mut next(): {:?}", iter_mut.next());
    println!("iter_mut next(): {:?}", iter_mut.next());
}
