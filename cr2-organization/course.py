import textwrap
import sys
from math import ceil
import re
import os
import shutil

BREAK_DURATION = 10
LUNCH_DURATION = 60
START_HOUR = 9
MINUTES_PER_DAY = 6 * 60

COLLATZ_EX = """
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
"""

RPN_EXPRESSION_EX = """
enum Op { Add, Subtract, Multiply, Divide }

enum Input {
  Number(i64),
  Op(Op)
}

Vec<Input>"""

TRY_OPERATOR_EX = """Use the try operator (?) to simplify the error handling in this code:

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
"""

COPY_MOVE_EXAMPLE = """
#[derive(Debug)] // Copy has been removed
struct Person {
  age: u8,
}

fn celebrate_birthday(person: Person) -> Person {
  person.age += 1;
  println!("Hurray you\'re now {} years old!", person.age);
  person
}

fn main() {
  let peter = Person { age: 40 };
  celebrate_birthday(peter);
  celebrate_birthday(peter);
}"""


# " XXmin"
def fdur(d):
    if hasattr(d, "duration"):
        d = d.duration()
    return f"{d} min"


# "HH:MM" (based on START_HOUR)
def ftime(time):
    h = START_HOUR + time // 60
    if h > 12:
        h -= 12
    m = time % 60
    return f"{h:02}:{m:02}"


def slugify(name):
    "Convert a name into a filename-safe name"
    name = re.sub(r"[^\w\s-]", "", name.lower())
    return re.sub(r"[-\s]+", "-", name).strip("-_")


class Slide:
    def __init__(
        self, *, name=None, title, minutes, existing=None, content=None, notes=None
    ):
        self.name = name or slugify(title)
        self.title = title
        self.existing = existing or []
        self.minutes = minutes
        self.content = content
        self.notes = notes
        self.is_exercise = title.startswith("Exercise")

    def duration(self):
        return self.minutes or 0

    def show(self, day, segment, file):
        print(f"\n## {segment.name}/{self.name} - {self.title}", file=file)
        existing = None
        if self.existing:
            existing = ", ".join([n.path for n in self.existing])
        for title, value in [
            ("Existing Content", existing),
            ("Notes", self.notes),
        ]:
            if value:
                value = value.strip()
                if "\n" in value:
                    print(f" * _{title}_:", file=file)
                    print("   ```", file=file)
                    print(textwrap.indent(value, "   "), file=file)
                    print("   ```", file=file)
                    print(file=file)
                else:
                    print(f" * _{title}_: {value}", file=file)
        if self.content:
            print("\n```", file=file)
            print(self.content.strip(), file=file)
            print("```\n", file=file)

    def redirects(self, segment, file):
        if not self.existing:
            return
        for existing in self.existing:
            old_path = existing.path.replace(".md", ".html")
            new_path = f"{segment.name}/{self.name}.html"
            if old_path == new_path:
                # this file replaced itself, so no problem..
                continue
            dots_to_root = "../" * old_path.count("/")
            new_path = f"{dots_to_root}{new_path}"
            print(f"{repr(old_path)} = {repr(new_path)}", file=file)

    def generate_page(self, old_dir, segment, new_dir):
        def use_existing(existing, include_source):
            if not existing.path:
                return
            if include_source:
                print(f"\n<!-- From v1 file {existing.path}: -->\n", file=file)

            # exercises typically refer to an .rs file, which we will find
            # and rename just below.
            include_re = re.compile(r"{{#include [a-z0-9-]*.rs")
            include_repl = "{{#include %s.rs" % (self.name,)
            with open(os.path.join(old_dir, existing.path)) as old:
                for line in old:
                    line = include_re.sub(
                        include_repl,
                        line.rstrip(),
                    )
                    print(line, file=file)

            # if the page had a paired `.rs` file, just copy that over
            rs_filename = os.path.join(old_dir, existing.path.replace(".md", ".rs"))
            if os.path.exists(rs_filename):
                shutil.copy(
                    rs_filename, os.path.join(new_dir, segment.name, f"{self.name}.rs")
                )

        try:
          with open(os.path.join(new_dir, segment.name, f"{self.name}.md"), "w") as file:
              print("---", file=file)
              print(f"minutes: {self.minutes}", file=file)
              if self.existing:
                  print(f"existing course material:", file=file)
                  for existing in self.existing:
                      print(f"- {existing.path}", file=file)
              print("---", file=file)
              print(file=file)

              for existing in self.existing:
                  if not existing.path:
                      # this occurs for grayed-out bits in SUMMARY.md
                      continue

              # if this is basically a copy of the old file, just use it.
              if not self.content and not self.notes and len(self.existing) == 1:
                  use_existing(self.existing[0], False)
                  return

              if self.notes:
                  print(f"<!-- NOTES:\n{self.notes}\n-->", file=file)

              print(f"# {self.title}", file=file)
              print(file=file)
              if self.content:
                  print(self.content, file=file)
              found_rs_file = False
              for existing in self.existing:
                  use_existing(existing, False)
        finally:
          if self.is_exercise:
              with open(os.path.join(new_dir, segment.name, f"solution.md"), "w") as file:
                  print("# Solution", file=file)
                  print("", file=file)
                  print("```rust,editable", file=file)
                  print("{{#include exercise.rs:solution}}", file=file)
                  print("```", file=file)
              rs_filename = os.path.join(new_dir, segment.name, f"{self.name}.rs")
              if not os.path.exists(rs_filename):
                  with open(rs_filename, "w") as file:
                      print("// TODO, including license", file=file)


class Segment:
    def __init__(self, *, name=None, title, slides):
        self.name = name or slugify(title)
        self.title = title
        self.slides = slides

    def duration(self):
        minutes = sum(slide.duration() for slide in self.slides)
        # round up to next multiple of 5 minutes
        return 5 * ceil(minutes / 5)

    def exercise_slide(self):
        for slide in self.slides:
            if slide.is_exercise:
                return slide

    def redirects(self, file):
        for slide in self.slides:
            slide.redirects(self, file)

    def generate_pages(self, old_dir, new_dir):
        seg_dir = os.path.join(new_dir, self.name)
        if not os.path.exists(seg_dir):
            os.makedirs(seg_dir)
        with open(os.path.join(new_dir, f"{self.name}.md"), "w") as file:
            print(f"# {self.title}", file=file)
            print(file=file)
            print(f"In this segment:", file=file)
            print(file=file)
            for slide in self.slides:
                print(f"* [{slide.title}]({self.name}/{slide.name}.md)", file=file)
                slide.generate_page(old_dir, self, new_dir)


class Day:
    def __init__(self, title, segments):
        self.title = title
        self.segments = segments

    def duration(self):
        duration = sum(segment.duration() for segment in self.segments)
        # add a break between each segment
        duration += (len(self.segments) - 1) * BREAK_DURATION
        # and replace one of those with lunch
        duration += LUNCH_DURATION - BREAK_DURATION
        return duration

    def lunch_after(self):
        "Calculate the segment after which the class should break for lunch"
        seg_duration = sum(segment.duration() for segment in self.segments)
        for lunch_after in range(len(self.segments)):
            before_lunch = sum(s.duration() for s in self.segments[: lunch_after + 1])
            before_lunch += lunch_after * BREAK_DURATION
            if before_lunch > 150:
                break
        return lunch_after

    def outline(self, prefix="", file=None):
        lunch_after = self.lunch_after()
        time = 0

        def line(name, duration, highlight=False, exercise=None):
            nonlocal time
            durstr = f"({fdur(duration)})" if duration else ""

            def hl(s):
                if highlight:
                    return f"_{s}_"
                else:
                    return s

            exercise_suffix = ""
            if exercise:
                exercise_suffix = f" ({exercise.title})"
            print(
                f"{prefix}{ftime(time)}: {hl(name)} {durstr}{exercise_suffix}",
                file=file,
            )
            time += duration

        for i, segment in enumerate(self.segments):
            exercise = segment.exercise_slide()
            line(segment.title, segment.duration(), True, exercise=exercise)
            if i == lunch_after:
                line("lunch", LUNCH_DURATION)
            elif i + 1 < len(self.segments):
                line("break", BREAK_DURATION)
            else:
                line("end", 0)

    def summary(self, file=None):
        lunch_after = self.lunch_after()
        time = 0

        print(f"----\n\n# {self.title}: Morning\n", file=file)

        def line(name, duration, highlight=False, exercise=None):
            nonlocal time
            durstr = f"({fdur(duration)})" if duration else ""

            def hl(s):
                if highlight:
                    return f"_{s}_"
                else:
                    return s

            exercise_suffix = ""
            if exercise:
                exercise_suffix = f" ({exercise.title})"
            print(
                f"{prefix}{ftime(time)}: {hl(name)} {durstr}{exercise_suffix}",
                file=file,
            )
            time += duration

        for i, segment in enumerate(self.segments):
            print(f"- [{segment.title}]({segment.name}.md)", file=file)
            for slide in segment.slides:
                print(f"  - [{slide.title}]({segment.name}/{slide.name}.md)", file=file)
                if slide.is_exercise:
                    print(f"    - [Solution]({segment.name}/solution.md)", file=file)

            if i == lunch_after:
                print(f"\n# {self.title}: Afternoon\n", file=file)

    def redirects(self, file):
        for segment in self.segments:
            segment.redirects(file)

    def generate_pages(self, old_dir, new_dir):
        for segment in self.segments:
            segment.generate_pages(old_dir, new_dir)


class Course:
    def __init__(self, days):
        self.days = days

    def duration(self):
        return sum(day.duration() for day in self.days)

    def outline(self, prefix="* ", file=None):
        for n, day in enumerate(self.days, start=1):
            print(file=file)
            print(
                f"{prefix}{day.title}: {fdur(day)} (expected: {fdur(MINUTES_PER_DAY)})",
                file=file,
            )
            day.outline("  " + prefix, file=file)

        print(file=file)
        print(
            f"Total: {fdur(self)} (expected: {fdur(MINUTES_PER_DAY*len(self.days))})",
            file=file,
        )

    def slides(self, file):
        for day in self.days:
            for segment in day.segments:
                for slide in segment.slides:
                    slide.show(day, segment, file=file)

    def summary(self, file):
        for day in self.days:
            day.summary(file)
            print("", file=file)

    def redirects(self, file):
        print("# redirects from course v1 to v2", file=file)
        for day in self.days:
            day.redirects(file)
        print("", file=file)

    def generate_pages(self, old_dir, new_dir):
        for day in self.days:
            day.generate_pages(old_dir, new_dir)


class SummaryTreeNode:
    def __init__(self, order, path, title):
        self.order = order  # sort key
        self.path = path
        self.title = title
        self.seen = False  # seen in v2.0 content
        self.children = []


class Summary:
    """Represent the Fundamentals portion of src/SUMMARY.md"""

    def __init__(self):
        self.parse_summary()

    def unseen(self, file=None):
        return [n for n in self.nodes.values() if not n.seen]

    def ref(self, path, seen_recursively=False):
        """Reference the named references from this slide."""
        node = self.nodes[path]
        if node.seen:
            raise RuntimeError(f"Node {path} referenced more than once")
        node.seen = True
        rv = [node]
        if seen_recursively:
            for child in node.children:
                rv.extend(self.ref(child.path, seen_recursively))
        return rv

    def parse_summary(self):
        skipping = True
        self.root = SummaryTreeNode([], "index.md", "Root")
        self.nodes = {}
        parents = [self.root]
        link_re = re.compile(r"^( *)- \[([^\]]*)\]\(([^)]*)\)")
        for line in open("../src/SUMMARY.md"):
            if line.startswith("#"):
                skipping = not line.startswith("# Day")
                continue
            if skipping:
                continue
            if mo := link_re.match(line):
                (prefix, title, path) = mo.groups()
                order = [len(p.children) for p in parents]
                node = SummaryTreeNode(order, path, title)
                self.nodes[path] = node

                prefix = len(prefix)
                assert prefix % 2 == 0
                prefix = prefix // 2 + 1
                parents = parents[:prefix]
                parents[-1].children.append(node)
                parents.append(node)


SUMMARY = Summary()
COURSE = Course(
    [
        Day(
            "Day 1",
            [
                Segment(
                    title="Hello, World",
                    slides=[
                        Slide(
                            title="Welcome",
                            minutes=5,
                            existing=SUMMARY.ref("welcome-day-1.md"),
                        ),
                        Slide(
                            title="What is Rust?",
                            existing=SUMMARY.ref("welcome-day-1/what-is-rust.md"),
                            minutes=10,
                            notes="Rust is a modern safe programming language used by Google and more broadly in the industry. This section will mention some success stories to excite the audience.",
                        ),
                        Slide(
                            title="Hello, World",
                            minutes=5,
                            existing=SUMMARY.ref("hello-world.md")
                            + SUMMARY.ref("hello-world/small-example.md")
                        ),
                        Slide(
                            title="Benefits of Rust",
                            name="benefits",
                            existing=SUMMARY.ref("why-rust.md") + SUMMARY.ref("why-rust/compile-time.md") + SUMMARY.ref("why-rust/runtime.md") + SUMMARY.ref("why-rust/modern.md"),
                            minutes=3,
                            notes="This section aims to give an overview of features in Rust that set it apart from other languages (e.g safety, modern language features like pattern matching, package ecosystem etc.). We will also mention the borrow checker briefly since it is crucial for safety.",
                        ),
                        Slide(
                            title="An Example in C",
                            name="example",
                            existing=SUMMARY.ref("why-rust/an-example-in-c.md"),
                            minutes=5,
                        ),
                        Slide(
                            title="Playground",
                            minutes=2,
                            notes="Point students toward the playground, and look at some of its capabilities",
                        )
                    ],
                ),
                Segment(
                    title="Types and Values",
                    slides=[
                        Slide(
                            title="Variables",
                            existing=SUMMARY.ref("basic-syntax/variables.md"),
                            minutes=5,
                        ),
                        Slide(
                            title="Values",
                            existing=SUMMARY.ref("basic-syntax/scalar-types.md"),
                            minutes=10,
                            notes="integers, Booleans, floats, unit, chars",
                        ),
                        Slide(
                            title="Arithmetic",
                            minutes=5,
                            notes="Arithmetic expressions, talk about handling of overflow",
                        ),
                        Slide(
                            title="Strings",
                            minutes=10,
                            notes="String, &str (without getting into references), utf-8 validity",
                        ),
                        Slide(
                            title="Type Inference",
                            name="inference",
                            existing=SUMMARY.ref("basic-syntax/type-inference.md"),
                            minutes=5,
                            notes="Inference from initialization (`let x = 10u8`) and by later usage (`let x; .. x = 10u8`). Detail on unitialized variables.",
                        ),
                        Slide(title="Exercise: Fibonacci", name="exercise", minutes=15),
                    ],
                ),
                Segment(
                    title="Control Flow Basics",
                    slides=[
                        Slide(
                            title="Conditionals",
                            existing=SUMMARY.ref("basic-syntax.md")
                            + SUMMARY.ref("control-flow.md")
                            + SUMMARY.ref("control-flow/if-expressions.md"),
                            minutes=5,
                        ),
                        Slide(
                            title="Loops",
                            existing=SUMMARY.ref("control-flow/while-expressions.md")
                            + SUMMARY.ref("control-flow/for-expressions.md")
                            + SUMMARY.ref("control-flow/loop-expressions.md"),
                            minutes=5,
                        ),
                        Slide(
                            title="`break` and `continue`",
                            name="break-continue",
                            existing=SUMMARY.ref("control-flow/break-continue.md"),
                            minutes=5,
                        ),
                        Slide(
                            title="Blocks and Scopes",
                            minutes=10,
                            existing=SUMMARY.ref("basic-syntax/scopes-shadowing.md")
                            + SUMMARY.ref("control-flow/blocks.md"),
                            notes="Mutable and immutable variables, scopes, shadowing, block values, expression values (e.g., value of an if expression)",
                        ),
                        Slide(
                            title="Functions",
                            existing=SUMMARY.ref("basic-syntax/functions.md", True),
                            minutes=3,
                            notes="Simple functions and the return statement",
                        ),
                        Slide(title="Macros", minutes=2),
                        Slide(
                            title="Exercise: Collatz conjecture",
                            name="exercise",
                            content=COLLATZ_EX,
                            minutes=15,
                            # Send all of the tables-of-contents for the old
                            # exercises here, just to have somewhere to go.
                            existing=SUMMARY.ref("exercises/day-1/morning.md")
                            + SUMMARY.ref("exercises/day-1/afternoon.md")
                            + SUMMARY.ref("exercises/day-2/morning.md")
                            + SUMMARY.ref("exercises/day-2/afternoon.md")
                            + SUMMARY.ref("exercises/day-3/morning.md")
                            + SUMMARY.ref("exercises/day-3/afternoon.md"),
                        ),
                    ],
                ),
                Segment(
                    title="Tuples and Arrays",
                    slides=[
                        Slide(
                            title="Tuples and Arrays",
                            existing=SUMMARY.ref("basic-syntax/compound-types.md"),
                            minutes=10,
                        ),
                        Slide(
                            title="Array Iteration",
                            name="iteration",
                            minutes=3,
                            notes="Early preview of `for` iterating over collections",
                        ),
                        Slide(
                            title="Pattern Matching",
                            name="match",
                            notes="Match as statement / expression, matching on ranges, guard expressions",
                            existing=SUMMARY.ref("control-flow/match-expressions.md")
                            + SUMMARY.ref("pattern-matching.md")
                            + SUMMARY.ref("pattern-matching/match-guards.md"),
                            minutes=10,
                        ),
                        Slide(
                            title="Destructuring",
                            notes="Destructuring arrays with `match`",
                            existing=SUMMARY.ref(
                                "pattern-matching/destructuring-arrays.md"
                            ),
                            minutes=5,
                        ),
                        Slide(
                            title="Exercise: Nested Arrays",
                            name="exercise",
                            existing=SUMMARY.ref("exercises/day-1/for-loops.md"),
                            notes="Simplify existing exercise, and drop bonus question",
                            minutes=15,
                        ),
                    ],
                ),
                Segment(
                    title="References",
                    slides=[
                        Slide(
                            title="Shared References",
                            name="shared",
                            existing=SUMMARY.ref("basic-syntax/references.md", True),
                            notes="First-pass introduction to references, without owernship, borrow checking, etc. Very informal coverage of lifetimes.",
                            minutes=10,
                        ),
                        Slide(
                            title="Exclusive References",
                            name="exclusive",
                            notes="Distinguish 'let v: &mut T` from `let mut v: &T`. Very informal coverage of aliasing.",
                            minutes=10,
                        ),
                        Slide(
                            title="Exercise: Geometry",
                            name="exercise",
                            notes="A few utility functions like dot product, magnitude, normalize",
                            existing=SUMMARY.ref("exercises/day-3/points-polygons.md"),
                            minutes=15,
                        ),
                    ],
                ),
                Segment(
                    title="User-Defined Types",
                    slides=[
                        Slide(
                            title="Named Structs",
                            notes="Overview of type names, naming conventions, field shorthand, `..` notation",
                            name="named-structs",
                            existing=SUMMARY.ref("structs.md")
                            + SUMMARY.ref("structs/field-shorthand.md"),
                            minutes=10,
                        ),
                        Slide(
                            title="Tuple Structs",
                            notes="Tuple structs, newtype wrappers, unit-like structs, including initialization syntax",
                            existing=SUMMARY.ref("structs/tuple-structs.md"),
                            minutes=10,
                        ),
                        Slide(
                            title="Enums",
                            notes="Including enums with payloads",
                            existing=SUMMARY.ref("enums.md", True),
                            minutes=5,
                        ),
                        Slide(
                            title="Deriving",
                            notes="Just cover deriving Debug; other traits will be introduced later",
                            existing=SUMMARY.ref("traits/deriving-traits.md"),
                            minutes=5,
                        ),
                        Slide(title="Type Aliases", name="aliases", minutes=2),
                        Slide(
                            title="Destructuring",
                            name="destructuring",
                            existing=SUMMARY.ref(
                                "pattern-matching/destructuring-structs.md"
                            )
                            + SUMMARY.ref("pattern-matching/destructuring-enums.md"),
                            minutes=10,
                        ),
                        Slide(
                            title="Let Control Flow",
                            name="let-control-flow",
                            notes="Presented as shorthands to match expressions",
                            existing=SUMMARY.ref("control-flow/novel.md")
                            + SUMMARY.ref("control-flow/if-let-expressions.md")
                            + SUMMARY.ref("control-flow/while-let-expressions.md"),
                            minutes=3,
                        ),
                        Slide(
                            title="Exercise: Expression Evaluation",
                            name="exercise",
                            content=RPN_EXPRESSION_EX,
                            existing=SUMMARY.ref("exercises/day-1/pattern-matching.md"),
                            minutes=15,
                        ),
                    ],
                ),
            ],
        ),
        Day(
            "Day 2",
            [
                Segment(
                    title="Methods and Traits",
                    slides=[
                        Slide(
                            title="Welcome",
                            minutes=5,
                            existing=SUMMARY.ref("welcome-day-2.md"),
                        ),
                        Slide(
                            title="Methods",
                            notes="Methods, associated functions, constructors",
                            name="methods",
                            existing=SUMMARY.ref("methods.md", True),
                            minutes=10,
                        ),
                        Slide(
                            title="Traits",
                            notes="Defining, implementing, and using traits, including provided methods",
                            existing=SUMMARY.ref("traits.md")
                            + SUMMARY.ref("traits/default-methods.md"),
                            minutes=10,
                        ),
                        Slide(
                            title="Trait Objects",
                            notes="How and when to use `dyn Trait`",
                            existing=SUMMARY.ref("traits/trait-objects.md"),
                            minutes=10,
                        ),
                        Slide(
                            title="Exercise: GUI Library",
                            name="exercise",
                            existing=SUMMARY.ref("exercises/day-3/simple-gui.md"),
                            notes="This will need to be simplified to fit the time",
                            minutes=15,
                        ),
                    ],
                ),
                Segment(
                    title="Generics",
                    slides=[
                        Slide(
                            title="Generic Functions",
                            existing=SUMMARY.ref("generics.md")
                            + SUMMARY.ref("generics/monomorphization.md"),
                            notes="Cover monomorphization, too",
                            minutes=5,
                        ),
                        Slide(
                            title="Generic Data types",
                            name="generic-data",
                            existing=SUMMARY.ref("generics/data-types.md")
                            + SUMMARY.ref("generics/methods.md"),
                            minutes=15,
                        ),
                        Slide(
                            title="Trait Bounds",
                            existing=SUMMARY.ref("traits/trait-bounds.md"),
                            minutes=10,
                        ),
                        Slide(
                            title="Impl Trait",
                            existing=SUMMARY.ref("traits/impl-trait.md"),
                            minutes=5,
                        ),
                        Slide(
                            title="Exercise: Generic `min`",
                            name="exercise",
                            minutes=5,
                        ),
                    ],
                ),
                Segment(
                    title="Standard Library Types",
                    name="std-types",
                    slides=[
                        Slide(
                            title="Standard Library",
                            minutes=3,
                            name="std",
                            existing=SUMMARY.ref("std.md"),
                        ),
                        Slide(
                            title="Option",
                            minutes=3,
                            existing=SUMMARY.ref("std/option-result.md"),
                            notes="Note that Result is addressed in a lot more detail in day 3",
                        ),
                        Slide(
                            title="Result",
                            minutes=3,
                            existing=SUMMARY.ref("error-handling/result.md"),
                        ),
                        Slide(
                            title="String",
                            minutes=3,
                            existing=SUMMARY.ref("std/string.md"),
                        ),
                        Slide(
                            title="Vec", minutes=3, existing=SUMMARY.ref("std/vec.md")
                        ),
                        Slide(
                            title="HashMap",
                            minutes=3,
                            existing=SUMMARY.ref("std/hashmap.md"),
                        ),
                        Slide(
                            title="Exercise: Counter",
                            name="exercise",
                            existing=SUMMARY.ref("exercises/day-2/book-library.md"),
                            minutes=15,
                        ),
                    ],
                ),
                Segment(
                    title="Standard Library Traits",
                    name="std-traits",
                    slides=[
                        Slide(
                            title="Comparisons",
                            minutes=5,
                            existing=SUMMARY.ref("traits/important-traits.md"),
                        ),
                        Slide(
                            title="Operators",
                            minutes=5,
                            existing=SUMMARY.ref("traits/operators.md"),
                        ),
                        Slide(
                            title="From and Into",
                            minutes=5,
                            existing=SUMMARY.ref("traits/from-into.md"),
                        ),
                        Slide(
                            title="Casting",
                            name="casting",
                            notes="Use of `as` to convert types, as a substitute for From/Into",
                            minutes=3,
                        ),
                        Slide(
                            title="Read and Write",
                            minutes=5,
                            existing=SUMMARY.ref("traits/read-write.md"),
                        ),
                        Slide(
                            title="`Default`, struct update syntax",
                            name="default",
                            existing=SUMMARY.ref("traits/default.md"),
                            minutes=5,
                        ),
                        Slide(
                            title="Closures",
                            existing=SUMMARY.ref("traits/closures.md"),
                            minutes=15,
                        ),
                        Slide(
                            title="Exercise: ROT13",
                            name="exercise",
                            existing=SUMMARY.ref(
                                "exercises/day-1/implicit-conversions.md"
                            ),
                            minutes=15,
                        ),
                    ],
                ),
                Segment(
                    title="Iterators",
                    slides=[
                        Slide(
                            title="Iterators",
                            minutes=5,
                            existing=SUMMARY.ref("traits/iterator.md"),
                            notes="The Iterator trait and basic usage",
                        ),
                        Slide(
                            title="IntoIterator",
                            name="intoiterator",
                            minutes=5,
                            existing=SUMMARY.ref(
                                "exercises/day-2/iterators-and-ownership.md"
                            ),
                        ),
                        Slide(
                            title="FromIterator",
                            name="fromiterator",
                            minutes=5,
                            existing=SUMMARY.ref("traits/from-iterator.md"),
                            notes="The FromIterator trait and the collect method.",
                        ),
                        Slide(
                            title="Exercise: Iterator Method Chaining",
                            name="exercise",
                            existing=SUMMARY.ref(
                                "exercises/day-2/strings-iterators.md"
                            ),
                            minutes=15,
                            notes="Something that involves a long-ish method chain (`someiter.foo().bar().bing().collect()`)",
                        ),
                    ],
                ),
                Segment(
                    title="Modules",
                    slides=[
                        Slide(
                            title="Modules",
                            existing=SUMMARY.ref("modules.md"),
                            minutes=5,
                            notes="Organizing the code within a crate and across crates.",
                        ),
                        Slide(
                            title="Filesystem Hierarchy",
                            name="filesystem",
                            existing=SUMMARY.ref("modules/filesystem.md"),
                            minutes=5,
                            notes="Organizing modules into multiple files.",
                        ),
                        Slide(
                            title="Visibility",
                            existing=SUMMARY.ref("modules/visibility.md"),
                            minutes=5,
                        ),
                        Slide(
                            title="use, super, self",
                            name="paths",
                            existing=SUMMARY.ref("modules/paths.md"),
                            notes="Include re-exports as well",
                            minutes=10,
                        ),
                        Slide(
                            title="Exercise: Modules for the GUI Library",
                            name="exercise",
                            minutes=10,
                            notes="Converting the GUI Library exercise solution into modules. Use the filesystem rather than the playground.",
                        ),
                    ],
                ),
                Segment(
                    title="Testing",
                    slides=[
                        Slide(
                            title="Language Docs",
                            name="docs",
                            notes="Use language docs to look at methods on integers",
                            minutes=5,
                        ),
                        Slide(
                            title="Test Modules",
                            existing=SUMMARY.ref("testing.md")
                            + SUMMARY.ref("testing/test-modules.md")
                            + SUMMARY.ref("testing/unit-tests.md"),
                            minutes=5,
                        ),
                        Slide(
                            title="Other Types of Tests",
                            name="other",
                            existing=SUMMARY.ref("testing/integration-tests.md")
                            + SUMMARY.ref("testing/doc-tests.md"),
                            minutes=20,
                            notes="Testing in more detail: Integration tests (separate crate with tests), documentation tests"
                        ),
                        Slide(
                            title="Useful Crates",
                            name="useful-crates",
                            existing=SUMMARY.ref("testing/useful-crates.md"),
                            minutes=3,
                        ),
                        Slide(
                            title="Compiler lints and Clippy", name="lints", 
                            minutes=5,
                        ),
                        Slide(
                            title="Exercise: Luhn Algorithm",
                            name="exercise",
                            minutes=15,
                            existing=SUMMARY.ref("exercises/day-1/luhn.md"),
                            notes="Give all of the test cases, to give students a sense for how nice TDD is in Rust",
                        ),
                    ],
                ),
            ],
        ),
        Day(
            "Day 3",
            [
                Segment(
                    title="Memory Management",
                    slides=[
                        Slide(
                            title="Welcome",
                            minutes=3,
                            existing=SUMMARY.ref("welcome-day-3.md"),
                        ),
                        Slide(
                            title="Review of Program Memory",
                            name="review",
                            existing=SUMMARY.ref("memory-management/stack-vs-heap.md")
                            +SUMMARY.ref("memory-management/stack.md"),
                            notes="Short summary of memory: stack vs. heap, allocations, etc.",
                            minutes=5,
                        ),
                        Slide(
                            title="Approaches to Memory Management",
                            name="approaches",
                            existing=SUMMARY.ref("memory-management.md")
                            +SUMMARY.ref("memory-management/manual.md")
                            +SUMMARY.ref("memory-management/scope-based.md")
                            +SUMMARY.ref("memory-management/garbage-collection.md"),
                            notes="Short summary of how different languages handle memory management",
                            minutes=10,
                        ),
                        Slide(
                            title="Ownership",
                            notes="Ownership and how it ties to destructors",
                            existing=SUMMARY.ref("ownership.md")
                            +SUMMARY.ref("memory-management/rust.md"),
                            minutes=5,
                        ),
                        Slide(
                            title="Move semantics",
                            name="move",
                            existing=SUMMARY.ref("ownership/move-semantics.md")
                            + SUMMARY.ref("ownership/moved-strings-rust.md", True)
                            + SUMMARY.ref("ownership/moves-function-calls.md"),
                            minutes=10,
                            notes="Using a non-copyable type (String) explore how values are moved in assignment and function calls.",
                        ),
                        Slide(
                            title="Clone",
                            notes="Quick mention of the `Clone` trait, performing deep/expensive copies when necessary",
                            minutes=2,
                        ),
                        Slide(
                            title="Copy Types",
                            content=COPY_MOVE_EXAMPLE,
                            notes="Present Copy as added functionality on top of the default move semantics: with Copy, the old value does not become invalid; Can derive Copy for a type if it implements Clone",
                            existing=SUMMARY.ref("ownership/copy-clone.md"),
                            minutes=5,
                        ),
                        Slide(
                            title="Drop",
                            name="drop",
                            existing=SUMMARY.ref("traits/drop.md"),
                            minutes=10,
                        ),
                        Slide(
                            title="Exercise: Builder Type",
                            name="exercise",
                            notes="A simple struct containing some Strings and with a partially-completed Builder pattern (with builder functions taking `self`) implemented. Students fill in a few `todo!()`s",
                            minutes=10,
                        ),
                    ],
                ),
                Segment(
                    title="Smart Pointers",
                    slides=[
                        Slide(
                            title="Box<T>",
                            name="box",
                            existing=SUMMARY.ref("std/box.md", True),
                            minutes=10,
                            notes="Extending ownership into the heap. Use a Box<SomeStruct> as an example",
                        ),
                        Slide(
                            title="Rc",
                            name="rc",
                            minutes=5,
                            existing=SUMMARY.ref("std/rc.md"),
                        ),
                        Slide(
                            title="Exercise: Binary Tree",
                            name="exercise",
                            minutes=15,
                            notes="See https://github.com/google/comprehensive-rust/pull/1084",
                        ),
                    ],
                ),
                Segment(
                    title="Borrowing",
                    name="borrowing",
                    slides=[
                        Slide(
                            title="Borrowing a Value",
                            name="shared",
                            existing=SUMMARY.ref("ownership/borrowing.md", True),
                            minutes=10,
                        ),
                        Slide(
                            title="Borrow Checking",
                            name="borrowck",
                            minutes=10,
                        ),
                        Slide(
                            title="Interior Mutability",
                            minutes=10,
                            notes="Introduce the concept, with an example based on Mutex showing an `&self` method doing mutation; reference Cell/RefCell without detail.",
                            existing=SUMMARY.ref("std/cell.md"),
                        ),
                        Slide(
                            title="Exercise: Health Statistics",
                            name="exercise",
                            existing=SUMMARY.ref("exercises/day-2/health-statistics.md"),
                            minutes=15,
                        ),
                    ],
                ),
                Segment(
                    title="Error Handling",
                    slides=[
                        Slide(
                            title="Panics",
                            existing=SUMMARY.ref("error-handling/panics.md")
                            + SUMMARY.ref("error-handling.md")
                            + SUMMARY.ref("error-handling/panic-unwind.md"),
                            notes="Students only need to know that it's possible, but unusual, to catch panics",
                            minutes=3,
                        ),
                        Slide(
                            title="Try operator",
                            name="try",
                            existing=SUMMARY.ref("error-handling/try-operator.md"),
                            minutes=5,
                        ),
                        Slide(
                            title="Try Conversions",
                            name="try-conversions",
                            existing=SUMMARY.ref("error-handling/converting-error-types.md")
                            + SUMMARY.ref(
                                "error-handling/converting-error-types-example.md"
                            ),
                            minutes=5,
                        ),
                        Slide(
                            title="Error Trait",
                            name="error",
                            existing=SUMMARY.ref(
                                "error-handling/deriving-error-enums.md"
                            ),
                            notes="Defining your own error type manually, as well as `Box<dyn Error>`",
                            minutes=5,
                        ),
                        Slide(
                            title="thiserror and anyhow",
                            existing=SUMMARY.ref("error-handling/error-contexts.md")
                            + SUMMARY.ref("error-handling/dynamic-errors.md"),
                            notes="Quick demo of using `thiserror` and `anyhow` to handle errors, including adding context",
                            minutes=5,
                        ),
                        Slide(
                            title="Exercise: Rewriting with Result",
                            name="exercise",
                            content=TRY_OPERATOR_EX,
                            minutes=10,
                        ),
                    ],
                ),
                Segment(
                    title="Slices and Lifetimes",
                    slides=[
                        Slide(
                            title="Slices: `&[T]`",
                            name="slices",
                            existing=SUMMARY.ref("basic-syntax/slices.md"),
                            minutes=10,
                        ),
                        Slide(
                            title="String References",
                            name="str",
                            notes="Including `&str` as a way of representing a slice of valid utf-8",
                            existing=SUMMARY.ref("basic-syntax/string-slices.md"),
                            minutes=10,
                        ),
                        Slide(
                            title="Lifetime Annotations",
                            existing=SUMMARY.ref("ownership/lifetimes.md"),
                            minutes=10,
                        ),
                        Slide(
                            title="Lifetime Elision",
                            existing=SUMMARY.ref(
                                "ownership/lifetimes-function-calls.md"
                            ),
                            minutes=5,
                        ),
                        Slide(
                            title="Struct Lifetimes",
                            existing=SUMMARY.ref("ownership/lifetimes-data-structures.md"),
                            minutes=5,
                        ),
                        Slide(
                            title="Exercise: Protobuf Parsing",
                            name="exercise",
                            minutes=15,
                        ),
                    ],
                ),
                Segment(
                    title="Unsafe Rust",
                    slides=[
                        Slide(
                            title="Unsafe",
                            existing=SUMMARY.ref("unsafe.md"),
                            minutes=5,
                        ),
                        Slide(
                            title="Dereferencing Raw Pointers",
                            name="dereferencing",
                            existing=SUMMARY.ref("unsafe/raw-pointers.md"),
                            minutes=3,
                        ),
                        Slide(
                            title="Static and Const",
                            existing=SUMMARY.ref("basic-syntax/static-and-const.md"),
                            minutes=5,
                            notes="try to make this short!",
                        ),
                        Slide(
                            title="Mutable Static Variables",
                            name="mutable-static",
                            existing=SUMMARY.ref("unsafe/mutable-static-variables.md"),
                            minutes=2,
                        ),
                        Slide(
                            title="Unions",
                            existing=SUMMARY.ref("unsafe/unions.md"),
                            minutes=3,
                        ),
                        Slide(
                            title="Unsafe Functions",
                            existing=SUMMARY.ref("unsafe/calling-unsafe-functions.md")
                            + SUMMARY.ref("unsafe/writing-unsafe-functions.md")
                            + SUMMARY.ref("unsafe/extern-functions.md"),
                            minutes=2,
                        ),
                        Slide(
                            title="Unsafe Traits",
                            existing=SUMMARY.ref("unsafe/unsafe-traits.md"),
                            minutes=5,
                        ),
                        Slide(
                            title="Exercise: FFI Wrapper",
                            name="exercise",
                            minutes=15,
                            existing=SUMMARY.ref("exercises/day-3/safe-ffi-wrapper.md"),
                        ),
                    ],
                ),
            ],
        ),
    ]
)


def main():
    unseen = SUMMARY.unseen()
    if unseen:
        print(f"# ORHPANED CONTENT - content in v1 not seen in v2\n")
        for node in sorted(unseen, key=lambda n: n.order):
            print(f"* `{node.path}` ({node.title})")
        sys.exit(1)

    with open("outline.md", "w") as file:
        print(f"# OUTLINE", file=file)
        COURSE.outline(file=file)

    with open("slides.md", "w") as file:
        print(f"# SLIDES", file=file)
        COURSE.slides(file=file)

    if 'CONVERT' not in os.environ:
        return

    os.chdir("..")
    os.rename("src", "oldsrc")
    os.mkdir("src")

    # copy back the stuff we're not changing
    KEEP_FILES = ["index.md"]
    for root, dirs, files in os.walk("oldsrc"):
        for filename in files:
            if filename.endswith(".md"):
                # .md files will be found via SUMMARY
                continue
            path = os.path.join(root, filename)
            path = path.replace("oldsrc/", "")
            KEEP_FILES.append(path)

    with open("src/SUMMARY.md", "w") as file:
        keeping = True
        included_new_summary = False
        ref_re = re.compile(r" *- \[.*\]\(([^)]*)\)")
        for line in open("oldsrc/SUMMARY.md"):
            line = line.rstrip()
            if line.startswith("#"):
                keeping = not line.startswith("# Day")
            if keeping:
                if mo := ref_re.match(line):
                    KEEP_FILES.append(mo.group(1))
                print(line, file=file)
            elif not included_new_summary:
                COURSE.summary(file)
                included_new_summary = True

    with open("book.toml") as file:
        book_toml = list(file)

    with open("book.toml", "w") as file:
        for line in book_toml:
            line = line.rstrip()
            if line.startswith("[output.exerciser]"):
                COURSE.redirects(file)
            print(line, file=file)

    for copyback in KEEP_FILES:
        dir = os.path.dirname(copyback)
        if dir and not os.path.exists(os.path.join("src", dir)):
            os.makedirs(os.path.join("src", dir))
        shutil.copyfile(os.path.join("oldsrc", copyback), os.path.join("src", copyback))

    COURSE.generate_pages("oldsrc", "src")


main()
