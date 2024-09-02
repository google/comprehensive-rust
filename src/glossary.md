<!-- i18n:comment Please keep { #glossary } untranslated. -->

# Glossary { #glossary }

The following is a glossary which aims to give a short definition of many Rust
terms. For translations, this also serves to connect the term back to the
English original.

<style>
h1#glossary ~ ul {
    list-style: none;
    padding-inline-start: 0;
}

h1#glossary ~ ul > li {
    /* Simplify with "text-indent: 2em hanging" when supported:
       https://caniuse.com/mdn-css_properties_text-indent_hanging */
    padding-left: 2em;
    text-indent: -2em;
}

h1#glossary ~ ul > li:first-line {
    font-weight: bold;
}
</style>

<!-- i18n:comment Please add the English term in italic after your -->
<!-- i18n:comment translated term. Also, please keep the hard line -->
<!-- i18n:comment breaks to ensure a nice formatting. -->

- allocate:\
  Dynamic memory allocation on [the heap](memory-management/review.md).
- argument:\
  Information that is passed into a function or method.
- associated type:\
  A type associated with a specific trait. Useful for defining the relationship
  between types.
- Bare-metal Rust:\
  Low-level Rust development, often deployed to a system without an operating
  system. See [Bare-metal Rust](bare-metal.md).
- block:\
  See [Blocks](control-flow-basics/blocks-and-scopes.md) and _scope_.
- borrow:\
  See [Borrowing](borrowing/shared.md).
- borrow checker:\
  The part of the Rust compiler which checks that all borrows are valid.
- brace:\
  `{` and `}`. Also called _curly brace_, they delimit _blocks_.
- build:\
  The process of converting source code into executable code or a usable
  program.
- call:\
  To invoke or execute a function or method.
- channel:\
  Used to safely pass messages [between threads](concurrency/channels.md).
- Comprehensive Rust 🦀:\
  The courses here are jointly called Comprehensive Rust 🦀.
- concurrency:\
  The execution of multiple tasks or processes at the same time.
- Concurrency in Rust:\
  See [Concurrency in Rust](concurrency/welcome.md).
- constant:\
  A value that does not change during the execution of a program.
- control flow:\
  The order in which the individual statements or instructions are executed in a
  program.
- crash:\
  An unexpected and unhandled failure or termination of a program.
- enumeration:\
  A data type that holds one of several named constants, possibly with an
  associated tuple or struct.
- error:\
  An unexpected condition or result that deviates from the expected behavior.
- error handling:\
  The process of managing and responding to errors that occur during program
  execution.
- exercise:\
  A task or problem designed to practice and test programming skills.
- function:\
  A reusable block of code that performs a specific task.
- garbage collector:\
  A mechanism that automatically frees up memory occupied by objects that are no
  longer in use.
- generics:\
  A feature that allows writing code with placeholders for types, enabling code
  reuse with different data types.
- immutable:\
  Unable to be changed after creation.
- integration test:\
  A type of test that verifies the interactions between different parts or
  components of a system.
- keyword:\
  A reserved word in a programming language that has a specific meaning and
  cannot be used as an identifier.
- library:\
  A collection of precompiled routines or code that can be used by programs.
- macro:\
  Rust macros can be recognized by a `!` in the name. Macros are used when
  normal functions are not enough. A typical example is `format!`, which takes a
  variable number of arguments, which isn't supported by Rust functions.
- `main` function:\
  Rust programs start executing with the `main` function.
- match:\
  A control flow construct in Rust that allows for pattern matching on the value
  of an expression.
- memory leak:\
  A situation where a program fails to release memory that is no longer needed,
  leading to a gradual increase in memory usage.
- method:\
  A function associated with an object or a type in Rust.
- module:\
  A namespace that contains definitions, such as functions, types, or traits, to
  organize code in Rust.
- move:\
  The transfer of ownership of a value from one variable to another in Rust.
- mutable:\
  A property in Rust that allows variables to be modified after they have been
  declared.
- ownership:\
  The concept in Rust that defines which part of the code is responsible for
  managing the memory associated with a value.
- panic:\
  An unrecoverable error condition in Rust that results in the termination of
  the program.
- parameter:\
  A value that is passed into a function or method when it is called.
- pattern:\
  A combination of values, literals, or structures that can be matched against
  an expression in Rust.
- payload:\
  The data or information carried by a message, event, or data structure.
- program:\
  A set of instructions that a computer can execute to perform a specific task
  or solve a particular problem.
- programming language:\
  A formal system used to communicate instructions to a computer, such as Rust.
- receiver:\
  The first parameter in a Rust method that represents the instance on which the
  method is called.
- reference counting:\
  A memory management technique in which the number of references to an object
  is tracked, and the object is deallocated when the count reaches zero.
- return:\
  A keyword in Rust used to indicate the value to be returned from a function.
- Rust:\
  A systems programming language that focuses on safety, performance, and
  concurrency.
- Rust Fundamentals:\
  Days 1 to 4 of this course.
- Rust in Android:\
  See [Rust in Android](android.md).
- Rust in Chromium:\
  See [Rust in Chromium](chromium.md).
- safe:\
  Refers to code that adheres to Rust's ownership and borrowing rules,
  preventing memory-related errors.
- scope:\
  The region of a program where a variable is valid and can be used.
- standard library:\
  A collection of modules providing essential functionality in Rust.
- static:\
  A keyword in Rust used to define static variables or items with a `'static`
  lifetime.
- string:\
  A data type storing textual data. See [Strings](references/strings.html) for
  more.
- struct:\
  A composite data type in Rust that groups together variables of different
  types under a single name.
- test:\
  A Rust module containing functions that test the correctness of other
  functions.
- thread:\
  A separate sequence of execution in a program, allowing concurrent execution.
- thread safety:\
  The property of a program that ensures correct behavior in a multithreaded
  environment.
- trait:\
  A collection of methods defined for an unknown type, providing a way to
  achieve polymorphism in Rust.
- trait bound:\
  An abstraction where you can require types to implement some traits of your
  interest.
- tuple:\
  A composite data type that contains variables of different types. Tuple fields
  have no names, and are accessed by their ordinal numbers.
- type:\
  A classification that specifies which operations can be performed on values of
  a particular kind in Rust.
- type inference:\
  The ability of the Rust compiler to deduce the type of a variable or
  expression.
- undefined behavior:\
  Actions or conditions in Rust that have no specified result, often leading to
  unpredictable program behavior.
- union:\
  A data type that can hold values of different types but only one at a time.
- unit test:\
  Rust comes with built-in support for running small unit tests and larger
  integration tests. See [Unit Tests](testing/unit-tests.html).
- unit type:\
  Type that holds no data, written as a tuple with no members.
- unsafe:\
  The subset of Rust which allows you to trigger _undefined behavior_. See
  [Unsafe Rust](unsafe-rust/unsafe.md).
- variable:\
  A memory location storing data. Variables are valid in a _scope_.
