fn multiply(x: i128, y: i128) -> i128 {
    x * y
}

fn main() {
    let x: i128 = 15;
    let y: bool = false;

    println!("{x} * {y} = {}", multiply(x.into(), y.into()));
}