# Visibility

Modules are a privacy boundary:

> Modules are a privacy boundary:
> * Module items are private by default (hides implementation details).
> * Parent and sibling items are always visible.

```rust,editable
mod outer {
    fn private() {
        println!("outer::private");
    }

    pub fn public() {
        println!("outer::public");
    }

    mod inner {
        fn private() {
            println!("outer::inner::private");
        }

        pub fn public() {
            println!("outer::inner::public");
            super::private();
        }
    }
}

fn main() {
    outer::public();
}
```
