fn main() {
    let x = add(5, 4);

    println!("x value is {x}");

    let new = min(10, 4);
    println!("new value is {new}");

    let var_check = is_even(5);
    println!("does value is even? {var_check}");
}

fn add(x: u32, y: u32) -> u32 {
    return x + y;
}

fn min(x: u32, y: u32) -> u32 {
    x - y
}

fn is_even(x: u32) -> bool {
    if x % 2 == 0 {
        return true;
    }
    false
}