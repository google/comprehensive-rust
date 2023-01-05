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

// This makes it possible to print Book values with {}.
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}
// ANCHOR_END: setup

// ANCHOR: Library_new
impl Library {
    fn new() -> Library {
        // ANCHOR_END: Library_new
        Library { books: Vec::new() }
    }

    // ANCHOR: Library_len
    //fn len(self) -> usize {
    //    unimplemented!()
    //}
    // ANCHOR_END: Library_len
    fn len(&self) -> usize {
        self.books.len()
    }

    // ANCHOR: Library_is_empty
    //fn is_empty(self) -> bool {
    //    unimplemented!()
    //}
    // ANCHOR_END: Library_is_empty
    fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    // ANCHOR: Library_add_book
    //fn add_book(self, book: Book) {
    //    unimplemented!()
    //}
    // ANCHOR_END: Library_add_book
    fn add_book(&mut self, book: Book) {
        self.books.push(book)
    }

    // ANCHOR: Library_print_books
    //fn print_books(self) {
    //    unimplemented!()
    //}
    // ANCHOR_END: Library_print_books
    fn print_books(&self) {
        for book in &self.books {
            println!("{}", book);
        }
    }

    // ANCHOR: Library_oldest_book
    //fn oldest_book(self) -> Option<&Book> {
    //    unimplemented!()
    //}
    // ANCHOR_END: Library_oldest_book
    fn oldest_book(&self) -> Option<&Book> {
        self.books.iter().min_by_key(|book| book.year)
    }
}

// ANCHOR: main
// This shows the desired behavior. Uncomment the code below and
// implement the missing methods. You will need to update the
// method signatures, including the "self" parameter!
fn main() {
    let library = Library::new();

    //println!("Our library is empty: {}", library.is_empty());
    //
    //library.add_book(Book::new("Lord of the Rings", 1954));
    //library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    //
    //library.print_books();
    //
    //match library.oldest_book() {
    //    Some(book) => println!("My oldest book is {book}"),
    //    None => println!("My library is empty!"),
    //}
    //
    //println!("Our library has {} books", library.len());
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
