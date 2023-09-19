// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// ANCHOR: solution
// ANCHOR: setup
struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// Implement the methods below. Update the `self` parameter to
// indicate the method's required level of ownership over the object:
//
// - `&self` for shared read-only access,
// - `&mut self` for unique and mutable access,
// - `self` for unique access by value.
impl Library {
    // ANCHOR_END: setup

    // ANCHOR: Library_new
    fn new() -> Library {
        // ANCHOR_END: Library_new
        Library { books: Vec::new() }
    }

    // ANCHOR: Library_len
    //fn len(self) -> usize {
    //    todo!("Return the length of `self.books`")
    //}
    // ANCHOR_END: Library_len
    fn len(&self) -> usize {
        self.books.len()
    }

    // ANCHOR: Library_is_empty
    //fn is_empty(self) -> bool {
    //    todo!("Return `true` if `self.books` is empty")
    //}
    // ANCHOR_END: Library_is_empty
    fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    // ANCHOR: Library_add_book
    //fn add_book(self, book: Book) {
    //    todo!("Add a new book to `self.books`")
    //}
    // ANCHOR_END: Library_add_book
    fn add_book(&mut self, book: Book) {
        self.books.push(book)
    }

    // ANCHOR: Library_print_books
    //fn print_books(self) {
    //    todo!("Iterate over `self.books` and print each book's title and year")
    //}
    // ANCHOR_END: Library_print_books
    fn print_books(&self) {
        for book in &self.books {
            println!("{}, published in {}", book.title, book.year);
        }
    }

    // ANCHOR: Library_oldest_book
    //fn oldest_book(self) -> Option<&Book> {
    //    todo!("Return a reference to the oldest book (if any)")
    //}
    // ANCHOR_END: Library_oldest_book
    fn oldest_book(&self) -> Option<&Book> {
        // Using a closure and a built-in method:
        // self.books.iter().min_by_key(|book| book.year)

        // Longer hand-written solution:
        let mut oldest: Option<&Book> = None;
        for book in self.books.iter() {
            if oldest.is_none() || book.year < oldest.unwrap().year {
                oldest = Some(book);
            }
        }

        oldest
    }
}

// ANCHOR: main
// This shows the desired behavior. Uncomment the code below and
// implement the missing methods. You will need to update the
// method signatures, including the "self" parameter! You may
// also need to update the variable bindings within main.
fn main() {
    let library = Library::new();

    //println!("The library is empty: library.is_empty() -> {}", library.is_empty());
    //
    //library.add_book(Book::new("Lord of the Rings", 1954));
    //library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    //
    //println!("The library is no longer empty: library.is_empty() -> {}", library.is_empty());
    //
    //
    //library.print_books();
    //
    //match library.oldest_book() {
    //    Some(book) => println!("The oldest book is {}", book.title),
    //    None => println!("The library is empty!"),
    //}
    //
    //println!("The library has {} books", library.len());
    //library.print_books();
}
// ANCHOR_END: main

#[test]
fn test_library_len() {
    let mut library = Library::new();
    assert_eq!(library.len(), 0);
    assert!(library.is_empty());

    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    assert_eq!(library.len(), 2);
    assert!(!library.is_empty());
}

#[test]
fn test_library_is_empty() {
    let mut library = Library::new();
    assert!(library.is_empty());

    library.add_book(Book::new("Lord of the Rings", 1954));
    assert!(!library.is_empty());
}

#[test]
fn test_library_print_books() {
    let mut library = Library::new();
    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    // We could try and capture stdout, but let us just call the
    // method to start with.
    library.print_books();
}

#[test]
fn test_library_oldest_book() {
    let mut library = Library::new();
    assert!(library.oldest_book().is_none());

    library.add_book(Book::new("Lord of the Rings", 1954));
    assert_eq!(
        library.oldest_book().map(|b| b.title.as_str()),
        Some("Lord of the Rings")
    );

    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    assert_eq!(
        library.oldest_book().map(|b| b.title.as_str()),
        Some("Alice's Adventures in Wonderland")
    );
}
