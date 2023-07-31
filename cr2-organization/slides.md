# SLIDES

## hello-world/welcome - Welcome
 * _Existing Content_: welcome-day-1.md

## hello-world/what-is-rust - What is Rust?
 * _Existing Content_: welcome-day-1/what-is-rust.md
 * _Notes_: Rust is a modern safe programming language used by Google and more broadly in the industry. This section will mention some success stories to excite the audience.

## hello-world/hello-world - Hello, World
 * _Existing Content_: hello-world.md, hello-world/small-example.md

## hello-world/benefits - Benefits of Rust
 * _Existing Content_: why-rust.md, why-rust/compile-time.md, why-rust/runtime.md, why-rust/modern.md
 * _Notes_: This section aims to give an overview of features in Rust that set it apart from other languages (e.g safety, modern language features like pattern matching, package ecosystem etc.). We will also mention the borrow checker briefly since it is crucial for safety.

## hello-world/example - An Example in C
 * _Existing Content_: why-rust/an-example-in-c.md

## hello-world/playground - Playground
 * _Notes_: Point students toward the playground, and look at some of its capabilities

## types-and-values/variables - Variables
 * _Existing Content_: basic-syntax/variables.md

## types-and-values/values - Values
 * _Existing Content_: basic-syntax/scalar-types.md
 * _Notes_: integers, Booleans, floats, unit, chars

## types-and-values/arithmetic - Arithmetic
 * _Notes_: Arithmetic expressions, talk about handling of overflow

## types-and-values/strings - Strings
 * _Notes_: String, &str (without getting into references), utf-8 validity

## types-and-values/inference - Type Inference
 * _Existing Content_: basic-syntax/type-inference.md
 * _Notes_: Inference from initialization (`let x = 10u8`) and by later usage (`let x; .. x = 10u8`). Detail on unitialized variables.

## types-and-values/exercise - Exercise: Fibonacci

## control-flow-basics/conditionals - Conditionals
 * _Existing Content_: basic-syntax.md, control-flow.md, control-flow/if-expressions.md

## control-flow-basics/loops - Loops
 * _Existing Content_: control-flow/while-expressions.md, control-flow/for-expressions.md, control-flow/loop-expressions.md

## control-flow-basics/break-continue - `break` and `continue`
 * _Existing Content_: control-flow/break-continue.md

## control-flow-basics/blocks-and-scopes - Blocks and Scopes
 * _Existing Content_: basic-syntax/scopes-shadowing.md, control-flow/blocks.md
 * _Notes_: Mutable and immutable variables, scopes, shadowing, block values, expression values (e.g., value of an if expression)

## control-flow-basics/functions - Functions
 * _Existing Content_: basic-syntax/functions.md, basic-syntax/rustdoc.md, basic-syntax/methods.md, basic-syntax/functions-interlude.md
 * _Notes_: Simple functions and the return statement

## control-flow-basics/macros - Macros

## control-flow-basics/exercise - Exercise: Collatz conjecture
 * _Existing Content_: exercises/day-1/morning.md, exercises/day-1/afternoon.md, exercises/day-2/morning.md, exercises/day-2/afternoon.md, exercises/day-3/morning.md, exercises/day-3/afternoon.md

```
Compute number of steps before an integer `n` becomes `1` following two rules:

- If `n` is even, set `n = n/2`
- If `n` is odd, set `n  = 3 * n + 1`

Given

fn collatz(n: i32) -> u32 {
  todo!("Implement")
}

#[test]
fn test_collatz() {
  assert_eq!(collatz(1), 0);
  assert_eq!(collatz(5), 5);
  assert_eq!(collatz(50), 24);
}

fill in the collatz function.
```


## tuples-and-arrays/tuples-and-arrays - Tuples and Arrays
 * _Existing Content_: basic-syntax/compound-types.md

## tuples-and-arrays/iteration - Array Iteration
 * _Notes_: Early preview of `for` iterating over collections

## tuples-and-arrays/match - Pattern Matching
 * _Existing Content_: control-flow/match-expressions.md, pattern-matching.md, pattern-matching/match-guards.md
 * _Notes_: Match as statement / expression, matching on ranges, guard expressions

## tuples-and-arrays/destructuring - Destructuring
 * _Existing Content_: pattern-matching/destructuring-arrays.md
 * _Notes_: Destructuring arrays with `match`

## tuples-and-arrays/exercise - Exercise: Nested Arrays
 * _Existing Content_: exercises/day-1/for-loops.md
 * _Notes_: Simplify existing exercise, and drop bonus question

## references/shared - Shared References
 * _Existing Content_: basic-syntax/references.md, basic-syntax/references-dangling.md
 * _Notes_: First-pass introduction to references, without owernship, borrow checking, etc. Very informal coverage of lifetimes.

## references/exclusive - Exclusive References
 * _Notes_: Distinguish 'let v: &mut T` from `let mut v: &T`. Very informal coverage of aliasing.

## references/exercise - Exercise: Geometry
 * _Existing Content_: exercises/day-3/points-polygons.md
 * _Notes_: A few utility functions like dot product, magnitude, normalize

## user-defined-types/named-structs - Named Structs
 * _Existing Content_: structs.md, structs/field-shorthand.md
 * _Notes_: Overview of type names, naming conventions, field shorthand, `..` notation

## user-defined-types/tuple-structs - Tuple Structs
 * _Existing Content_: structs/tuple-structs.md
 * _Notes_: Tuple structs, newtype wrappers, unit-like structs, including initialization syntax

## user-defined-types/enums - Enums
 * _Existing Content_: enums.md, enums/variant-payloads.md, enums/sizes.md
 * _Notes_: Including enums with payloads

## user-defined-types/deriving - Deriving
 * _Existing Content_: traits/deriving-traits.md
 * _Notes_: Just cover deriving Debug; other traits will be introduced later

## user-defined-types/aliases - Type Aliases

## user-defined-types/destructuring - Destructuring
 * _Existing Content_: pattern-matching/destructuring-structs.md, pattern-matching/destructuring-enums.md

## user-defined-types/let-control-flow - Let Control Flow
 * _Existing Content_: control-flow/novel.md, control-flow/if-let-expressions.md, control-flow/while-let-expressions.md
 * _Notes_: Presented as shorthands to match expressions

## user-defined-types/exercise - Exercise: Expression Evaluation
 * _Existing Content_: exercises/day-1/pattern-matching.md

```
enum Op { Add, Subtract, Multiply, Divide }

enum Input {
  Number(i64),
  Op(Op)
}

Vec<Input>
```


## methods-and-traits/welcome - Welcome
 * _Existing Content_: welcome-day-2.md

## methods-and-traits/methods - Methods
 * _Existing Content_: methods.md, methods/receiver.md, methods/example.md
 * _Notes_: Methods, associated functions, constructors

## methods-and-traits/traits - Traits
 * _Existing Content_: traits.md, traits/default-methods.md
 * _Notes_: Defining, implementing, and using traits, including provided methods

## methods-and-traits/trait-objects - Trait Objects
 * _Existing Content_: traits/trait-objects.md
 * _Notes_: How and when to use `dyn Trait`

## methods-and-traits/exercise - Exercise: GUI Library
 * _Existing Content_: exercises/day-3/simple-gui.md
 * _Notes_: This will need to be simplified to fit the time

## generics/generic-functions - Generic Functions
 * _Existing Content_: generics.md, generics/methods.md, generics/monomorphization.md
 * _Notes_: Cover monomorphization, too

## generics/generic-structs - Generic Structs
 * _Existing Content_: generics/data-types.md

## generics/trait-bounds - Trait Bounds
 * _Existing Content_: traits/trait-bounds.md, traits/impl-trait.md
 * _Notes_: Generic bounds and impl Trait` in argument and return position

## generics/exercise - Exercise: Generic `min`

## std-types/std - Standard Library
 * _Existing Content_: std.md

## std-types/option - Option
 * _Existing Content_: std/option-result.md
 * _Notes_: Note that Result is addressed in a lot more detail in day 3

## std-types/result - Result
 * _Existing Content_: error-handling/result.md

## std-types/string - String
 * _Existing Content_: std/string.md

## std-types/vec - Vec
 * _Existing Content_: std/vec.md

## std-types/hashmap - HashMap
 * _Existing Content_: std/hashmap.md

## std-types/exercise - Exercise: Counter
 * _Existing Content_: exercises/day-2/book-library.md

## std-traits/equality - Eq, PartialEq, and Ord
 * _Existing Content_: traits/important-traits.md

## std-traits/operators - Operators
 * _Existing Content_: traits/operators.md

## std-traits/into-and-from - Into and From
 * _Existing Content_: traits/from-into.md

## std-traits/casting - Casting
 * _Notes_: Use of `as` to convert types, as a substitute for From/Into

## std-traits/read-and-write - Read and Write
 * _Existing Content_: traits/read-write.md

## std-traits/default - `Default`, struct update syntax
 * _Existing Content_: traits/default.md

## std-traits/closures - Closures
 * _Existing Content_: traits/closures.md

## std-traits/exercise - Exercise: Integer Conversions
 * _Existing Content_: exercises/day-1/implicit-conversions.md
 * _Notes_: Simplify to fit 15 minutes, with more specific instructions

## iterators/iterators - Iterators
 * _Existing Content_: traits/iterator.md
 * _Notes_: The Iterator trait and basic usage

## iterators/ownership - Iterator Ownership
 * _Existing Content_: exercises/day-2/iterators-and-ownership.md
 * _Notes_: Ownership of iterators and any underlying collection, and the FromIterator trait

## iterators/fromiterator - FromIterator
 * _Existing Content_: traits/from-iterator.md
 * _Notes_: The FromIterator trait and the collect method.

## iterators/exercise - Exercise: Iterator Method Chaining
 * _Existing Content_: exercises/day-2/strings-iterators.md
 * _Notes_: Something that involves a long-ish method chain (`someiter.foo().bar().bing().collect()`)

## modules/modules - Modules
 * _Existing Content_: modules.md, modules/filesystem.md
 * _Notes_: Organizing the code within a crate and across crates.

## modules/visibility - Visibility
 * _Existing Content_: modules/visibility.md

## modules/use-super-self - use, super, self
 * _Existing Content_: modules/paths.md
 * _Notes_: Include re-exports as well

## modules/exercise - Exercise: Modules for the GUI Library
 * _Notes_: Converting the GUI Library exercise solution into modules. Use the filesystem rather than the playground.

## testing/test-modules - Test Modules
 * _Existing Content_: testing.md, testing/test-modules.md, testing/unit-tests.md

## testing/other - Other Types of Tests
 * _Existing Content_: testing/integration-tests.md, testing/doc-tests.md, testing/useful-crates.md
 * _Notes_: Testing in more detail: Integration tests (separate crate with tests), documentation tests, test utilities like googletest, proptest, rstest

## testing/exercise - Exercise: Luhn Algorithm
 * _Existing Content_: exercises/day-1/luhn.md
 * _Notes_: Give all of the test cases, to give students a sense for how nice TDD is in Rust

## memory-management/welcome - Welcome
 * _Existing Content_: welcome-day-3.md

## memory-management/approaches - Approaches to Memory Management
 * _Existing Content_: memory-management.md, memory-management/stack-vs-heap.md, memory-management/stack.md, memory-management/manual.md, memory-management/scope-based.md, memory-management/garbage-collection.md, memory-management/rust.md
 * _Notes_: Short summary of how different languages handle memory management

## memory-management/ownership - Ownership
 * _Existing Content_: ownership.md
 * _Notes_: Ownership and how it ties to destructors

## memory-management/move - Move semantics
 * _Existing Content_: ownership/move-semantics.md, ownership/moved-strings-rust.md, ownership/double-free-modern-cpp.md, ownership/moves-function-calls.md
 * _Notes_: Using a non-copyable type (String) explore how values are moved in assignment and function calls.

## memory-management/clone - Clone
 * _Notes_: Quick mention of the `Clone` trait, performing deep/expensive copies when necessary

## memory-management/copy-types - Copy Types
 * _Existing Content_: ownership/copy-clone.md
 * _Notes_: Present Copy as added functionality on top of the default move semantics: with Copy, the old value does not become invalid; Can derive Copy for a type if it implements Clone

```
#[derive(Debug)] // Copy has been removed
struct Person {
  age: u8,
}

fn celebrate_birthday(person: Person) -> Person {
  person.age += 1;
  println!("Hurray you're now {} years old!", person.age);
  person
}

fn main() {
  let peter = Person { age: 40 };
  celebrate_birthday(peter);
  celebrate_birthday(peter);
}
```


## memory-management/exercise - Exercise: Builder Type
 * _Notes_: A simple struct containing some Strings and with a partially-completed Builder pattern (with builder functions taking `self`) implemented. Students fill in a few `todo!()`s

## smart-pointers/box - Box<T>
 * _Existing Content_: std/box.md, std/box-recursive.md, std/box-niche.md
 * _Notes_: Extending ownership into the heap. Use a Box<SomeStruct> as an example

## smart-pointers/rc - Rc
 * _Existing Content_: std/rc.md

## smart-pointers/drop - Drop
 * _Existing Content_: traits/drop.md

## smart-pointers/exercise - Exercise: Binary Tree
 * _Notes_: See https://github.com/google/comprehensive-rust/pull/1084

## developer-conveniences/docs - Language Docs
 * _Notes_: Use language docs to look at methods on integers

## developer-conveniences/static-and-const - Static and Const
 * _Existing Content_: basic-syntax/static-and-const.md
 * _Notes_: try to make this short!

## developer-conveniences/lints - Compiler lints and Clippy

## refs/shared - Borrowing a Value
 * _Existing Content_: ownership/borrowing.md, ownership/shared-unique-borrows.md

## refs/borrowck - Borrow Checking

## refs/interior-mutability - Interior Mutability
 * _Existing Content_: std/cell.md
 * _Notes_: Introduce the concept, with an example based on Mutex showing an `&self` method doing mutation; reference Cell/RefCell without detail.

## refs/exercise - Exercise: Health Statistics
 * _Existing Content_: exercises/day-2/health-statistics.md

## slices-and-lifetimes/slices - Slices: `&[T]`
 * _Existing Content_: basic-syntax/slices.md, basic-syntax/string-slices.md

## slices-and-lifetimes/str - String References
 * _Notes_: Including `&str` as a way of representing a slice of valid utf-8

## slices-and-lifetimes/lifetime-annotations - Lifetime Annotations
 * _Existing Content_: ownership/lifetimes.md, ownership/lifetimes-data-structures.md

## slices-and-lifetimes/lifetime-elision - Lifetime Elision
 * _Existing Content_: ownership/lifetimes-function-calls.md

## slices-and-lifetimes/struct-lifetimes - Struct Lifetimes

## slices-and-lifetimes/exercise - Exercise: Packet Parsing

```
Given

# TODO: adjust this to take and return byte slices

fn src(header: [u8; 20]) -> [u8; 4] {
  todo!("Extract source address")
}

fn dst(header: [u8; 20]) -> [u8; 4] {
  todo!("Extract source address")
}

fn main() {
    let header: [u8; 20] = [
        0x45, 0x00, 0x00, 0x3e, 0x00, 0x00, 0x40, 0x00, 0x64, 0x04, 0x94, 0x6b, 0xc0, 0xa8, 0x00,
        0x01, 0xc0, 0xa8, 0x00, 0xff,
    ];
    println!("Source address: {:?}", src(header));
    println!("Destination address: {:?}", dst(header));
}

Print the source and destination IPv4 addresses.

Packet generated with:

fn main() {
    let packet = etherparse::Ipv4Header::new(42, 100, 4, [192, 168, 0, 1], [192, 168, 0, 255]);
    let mut wire = Vec::new();
    packet.write(&mut wire).unwrap();
    println!("packet: {:#04x?}", &wire);
}
```


## error-handling/recovery - Recoverable & Irrecoverable Errors
 * _Existing Content_: error-handling.md
 * _Notes_: Introduce recoverable (Result) and non-recoverable errors (panic!)

## error-handling/panics - Panics
 * _Existing Content_: error-handling/panics.md, error-handling/panic-unwind.md
 * _Notes_: Students only need to know that it's possible, but unusual, to catch panics

## error-handling/try - Try operator
 * _Existing Content_: error-handling/try-operator.md, error-handling/converting-error-types.md, error-handling/converting-error-types-example.md
 * _Notes_: Basic usage of `?` along with its implied use of From

## error-handling/error - Error Trait
 * _Existing Content_: error-handling/deriving-error-enums.md, error-handling/dynamic-errors.md
 * _Notes_: Defining your own error type manually, as well as `Box<dyn Error>`

## error-handling/thiserror-and-anyhow - thiserror and anyhow
 * _Existing Content_: error-handling/error-contexts.md
 * _Notes_: Quick demo of using `thiserror` and `anyhow` to handle errors, including adding context

## error-handling/exercise - Exercise: Rewriting with `?`

```
Use the try operator (?) to simplify the error handling in this code:

use std::fs;
use std::io::{self, Read};

fn read_username(path: &str) -> Result<String, io::Error> {
    let username_file_result = fs::File::open(path);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn main() {
    //fs::write("config.dat", "alice").unwrap();
    let username = read_username("config.dat");
    println!("username or error: {username:?}");
}
```


## unsafe-rust/unsafe - Unsafe
 * _Existing Content_: unsafe.md

## unsafe-rust/dereferencing - Dereferencing Raw Pointers
 * _Existing Content_: unsafe/raw-pointers.md

## unsafe-rust/mutable-static - Mutable Static Variables
 * _Existing Content_: unsafe/mutable-static-variables.md

## unsafe-rust/unions - Unions
 * _Existing Content_: unsafe/unions.md

## unsafe-rust/unsafe-functions - Unsafe Functions
 * _Existing Content_: unsafe/calling-unsafe-functions.md, unsafe/writing-unsafe-functions.md, unsafe/extern-functions.md

## unsafe-rust/unsafe-traits - Unsafe Traits
 * _Existing Content_: unsafe/unsafe-traits.md

## unsafe-rust/exercise - Exercise: FFI Wrapper
 * _Existing Content_: exercises/day-3/safe-ffi-wrapper.md
