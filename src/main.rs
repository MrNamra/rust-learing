fn main() {
    let mut age = 22;
    println!("My Age is {age}");

    age = 15;
    println!("My mutables Age is  {age}");

    const ONE_HOUR: u32 = 60 * 60;

    const THREE_HOURS: u32 = ONE_HOUR * 3;

    println!("Three Hours In seconds is {THREE_HOURS}")
    
}
