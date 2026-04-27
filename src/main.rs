fn main() {
    let var_check = is_even(4);
    println!("does value is even? {var_check}");

    if var_check {
        println!("fn return true")
    }
}

fn is_even(x: u32) -> bool {
    if x % 2 == 0 {
        return true;
    }
    false
}