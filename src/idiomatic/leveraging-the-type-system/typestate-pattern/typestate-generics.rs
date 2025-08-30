// ANCHOR: Complete
use std::fmt::Write as _;

// ANCHOR: Serializer-def
struct Serializer<S> {
    // [...]
    indent: usize,
    buffer: String,
    state: S,
}
// ANCHOR_END: Serializer-def

// ANCHOR: Root-def
struct Root;
// ANCHOR_END: Root-def

// ANCHOR: Struct-def
struct Struct<S>(S);
// ANCHOR_END: Struct-def

// ANCHOR: List-def
struct List<S>(S);
// ANCHOR_END: List-def

// ANCHOR: Property-def
struct Property<S>(S);
// ANCHOR_END: Property-def

// ANCHOR: Root-impl
impl Serializer<Root> {
    fn new() -> Self {
        // [...]
        Self { indent: 0, buffer: String::new(), state: Root }
    }

    fn serialize_struct(mut self, name: &str) -> Serializer<Struct<Root>> {
        // [...]
        writeln!(self.buffer, "{name} {{").unwrap();
        Serializer {
            indent: self.indent + 1,
            buffer: self.buffer,
            state: Struct(self.state),
        }
    }

    fn finish(self) -> String {
        // [...]
        self.buffer
    }
}
// ANCHOR_END: Root-impl

// ANCHOR: Struct-impl
impl<S> Serializer<Struct<S>> {
    fn serialize_property(mut self, name: &str) -> Serializer<Property<Struct<S>>> {
        // [...]
        write!(self.buffer, "{}{name}: ", " ".repeat(self.indent * 2)).unwrap();
        Serializer {
            indent: self.indent,
            buffer: self.buffer,
            state: Property(self.state),
        }
    }

    fn finish_struct(mut self) -> Serializer<S> {
        // [...]
        self.indent -= 1;
        writeln!(self.buffer, "{}}}", " ".repeat(self.indent * 2)).unwrap();
        Serializer { indent: self.indent, buffer: self.buffer, state: self.state.0 }
    }
}
// ANCHOR_END: Struct-impl

// ANCHOR: Property-impl
impl<S> Serializer<Property<Struct<S>>> {
    fn serialize_struct(mut self, name: &str) -> Serializer<Struct<Struct<S>>> {
        // [...]
        writeln!(self.buffer, "{name} {{").unwrap();
        Serializer {
            indent: self.indent + 1,
            buffer: self.buffer,
            state: Struct(self.state.0),
        }
    }

    fn serialize_list(mut self) -> Serializer<List<Struct<S>>> {
        // [...]
        writeln!(self.buffer, "[").unwrap();
        Serializer {
            indent: self.indent + 1,
            buffer: self.buffer,
            state: List(self.state.0),
        }
    }

    fn serialize_string(mut self, value: &str) -> Serializer<Struct<S>> {
        // [...]
        writeln!(self.buffer, "{value},").unwrap();
        Serializer { indent: self.indent, buffer: self.buffer, state: self.state.0 }
    }
}
// ANCHOR_END: Property-impl

// ANCHOR: List-impl
impl<S> Serializer<List<S>> {
    fn serialize_struct(mut self, name: &str) -> Serializer<Struct<List<S>>> {
        // [...]
        writeln!(self.buffer, "{}{name} {{", " ".repeat(self.indent * 2)).unwrap();
        Serializer {
            indent: self.indent + 1,
            buffer: self.buffer,
            state: Struct(self.state),
        }
    }

    fn serialize_string(mut self, value: &str) -> Self {
        // [...]
        writeln!(self.buffer, "{}{value},", " ".repeat(self.indent * 2)).unwrap();
        self
    }

    fn finish_list(mut self) -> Serializer<S> {
        // [...]
        self.indent -= 1;
        writeln!(self.buffer, "{}]", " ".repeat(self.indent * 2)).unwrap();
        Serializer { indent: self.indent, buffer: self.buffer, state: self.state.0 }
    }
}
// ANCHOR_END: List-impl

// ANCHOR: main
fn main() {
    #[rustfmt::skip]
    let serializer = Serializer::new()
        .serialize_struct("Foo")
            .serialize_property("bar")
            .serialize_struct("Bar")
                .serialize_property("baz")
                .serialize_list()
                    .serialize_string("abc")
                    .serialize_struct("Baz")
                        .serialize_property("partial")
                        .serialize_string("def")
                        .serialize_property("empty")
                        .serialize_struct("Empty")
                        .finish_struct()
                    .finish_struct()
                .finish_list()
            .finish_struct()
        .finish_struct();

    let output = serializer.finish();

    println!("{output}");

    // These will all fail at compile time:

    // Serializer::new().serialize_list();
    // Serializer::new().serialize_string("foo");
    // Serializer::new().serialize_struct("Foo").serialize_string("bar");
    // Serializer::new().serialize_struct("Foo").serialize_list();
    // Serializer::new().serialize_property("foo");
}
// ANCHOR_END: main
