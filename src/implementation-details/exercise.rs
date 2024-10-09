#![allow(clippy::non_minimal_cfg, dead_code)]

// --- Exercise

#[cfg(all())]
#[derive(Debug)]
enum List<T> {
    /// A non-empty list: first element and the rest of the list.
    Element(T, Box<List<T>>),
    /// An empty list.
    Nil,
}

#[cfg(all())]
impl<T: Copy> List<T> {
    fn new(items: &[T]) -> Self {
        let mut list = List::Nil;
        for item in items.iter().rev() {
            list = List::Element(*item, Box::new(list));
        }
        list
    }

    fn size_of(&self) -> usize {
        let mut iter = self;
        let mut size = std::mem::size_of_val(self);
        while let List::Element(_, child) = iter {
            iter = child;
            size += std::mem::size_of_val(iter);
        }
        size
    }

    fn len(&self) -> usize {
        let mut iter = self;
        let mut len = 0;
        while let List::Element(_, child) = iter {
            len += 1;
            iter = child;
        }
        len
    }
}

// --- Solution

#[cfg(any())]
#[derive(Debug)]
struct ListNode<T> {
    value: T,
    child: List<T>,
}

#[cfg(any())]
#[derive(Debug)]
struct List<T>(Option<Box<ListNode<T>>>);

#[cfg(any())]
impl<T: Copy> List<T> {
    fn new(items: &[T]) -> Self {
        let mut list = List(None);
        for value in items.iter().rev() {
            list = List(Some(Box::new(ListNode { value: *value, child: list })));
        }
        list
    }

    fn size_of(&self) -> usize {
        let mut iter = self;
        let mut size = 0;
        while let Some(boxed_node) = &iter.0 {
            size += std::mem::size_of_val(boxed_node.as_ref());
            iter = &boxed_node.child;
        }
        size
    }

    fn len(&self) -> usize {
        let mut iter = self;
        let mut len = 0;
        while let Some(boxed_node) = &iter.0 {
            len += 1;
            iter = &boxed_node.child;
        }
        len
    }
}

// --- Main

fn main() {
    let list = List::new(&[1u64, 2, 3, 4, 5, 6, 7]);
    println!("List<T> has size {}", std::mem::size_of::<List<u64>>());
    println!(
        "{:?}: {} bytes, should be no more than {}",
        list,
        list.size_of(),
        16 * list.len()
    );
}
