# Trait Objects

Trait objects allow for values of different types, for instance in a collection:

```rust,editable
trait Pet {
    fn name(&self) -> String;
}

struct Dog {
    name: String,
}

struct Cat;

impl Pet for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Pet for Cat {
    fn name(&self) -> String {
        String::from("The cat") // No name, cats won't respond to it anyway.
    }
}

fn main() {
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Cat),
        Box::new(Dog { name: String::from("Fido") }),
    ];
    for pet in pets {
        println!("Hello {}!", pet.name());
    }
}
```


Memory layout after allocating `pets`:

```bob
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - - - - - - - - - - - - -.
:                           :     :                                             :
:    pets                   :     :                                             :
:   +-----------+-------+   :     :   +-----+-----+                             :
:   | ptr       |   o---+---+-----+-->| o o | o o |                             :
:   | len       |     2 |   :     :   +-|-|-+-|-|-+                             :
:   | capacity  |     2 |   :     :     | |   | |   +---------------+           :
:   +-----------+-------+   :     :     | |   | '-->| name: "Fido"  |           :
:                           :     :     | |   |     +---------------+           :
`- - - - - - - - - - - - - -'     :     | |   |                                 :
                                  :     | |   |     +----------------------+    :   
                                  :     | |   '---->| "<Dog as Pet>::name" |    :
                                  :     | |         +----------------------+    : 
                                  :     | |                                     : 
                                  :     | |   +-+                               :   
                                  :     | '-->|\|                               :     
                                  :     |     +-+                               :    
                                  :     |                                       : 
                                  :     |     +----------------------+          : 
                                  :     '---->| "<Cat as Pet>::name" |          : 
                                  :           +----------------------+          :
                                  :                                             :
                                  '- - - - - - - - - - - - - - - - - - - - - - -'

```

<details>

* Types that implement a given trait may be of different sizes. This makes it impossible to have things like `Vec<Pet>` in the example above.
* `dyn Pet` is a way to tell the compiler about a dynamically sized type that implements `Pet`.
* In the example, `pets` holds *fat pointers* to objects that implement `Pet`. The fat pointer consists of two components, a pointer to the actual object and a pointer to the virtual method table for the `Pet` implementation of that particular object.
* Compare these outputs in the above example:
     ```rust,ignore
         println!("{} {}", std::mem::size_of::<Dog>(), std::mem::size_of::<Cat>());
         println!("{} {}", std::mem::size_of::<&Dog>(), std::mem::size_of::<&Cat>());
         println!("{}", std::mem::size_of::<&dyn Pet>());
         println!("{}", std::mem::size_of::<Box<dyn Pet>>());
     ```

</details>
