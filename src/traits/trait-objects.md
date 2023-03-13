# Trait Objects

We've seen how a function can take arguments which implement a trait:

```rust
trait Greet {
    fn say_hello(&self);
}

struct Dog {
    name: String,
}

struct Cat;  // No name, cats won't respond to it anyway.

impl Greet for Dog {
    fn say_hello(&self) {
        println!("Wuf, my name is {}!", self.name);
    }
}

impl Greet for Cat {
    fn say_hello(&self) {
        println!("Miau!");
    }
}

fn main() {
    let pets: Vec<Box<dyn Greet>> = vec![
        Box::new(Dog { name: String::from("Fido") }),
        Box::new(Cat),
    ];
    for pet in pets {
        pet.say_hello();
    }
}
```


Memory layout after allocating `pets`:

```bob
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - - - - - - - - - - - - - - - -.
:                           :     :                                                   :
:    xs                     :     :                                                   :
:   +-----------+-------+   :     :   +-----+-----+                                   :
:   | ptr       |   o---+---+-----+-->| o o | o o |                                   :
:   | len       |     2 |   :     :   +-|-|-+-|-|-+                                   :
:   | capacity  |     2 |   :     :     |     | |   +----+----+---+----+              :
:   +-----------+-------+   :     :     |     | '-->| F  | i  | d | o  |              :
:                           :     :     |     |     +----+----+---+----+              :
`- - - - - - - - - - - - - -'     :     |     |                                       :
                                  '- - - - - - - - - - - - - - - - - - - - - - - - - -'
                                        |     |     +-----------------------------+    
                                        |     '---->| "<Dog as Greet>::say_hello" |    
                                        |           +-----------------------------+    
                                        |                                              
                                        |                                              
                                        |                                              
                                        |                                              
                                        |                                              
                                        |     +-----------------------------+          
                                        '---->| "<Cat as Greet>::say_hello" |          
                                              +-----------------------------+         
                                  

```