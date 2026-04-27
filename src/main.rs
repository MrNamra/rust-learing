fn main() {
    let mut num = 1;
    loop {
        println!("Value of number is {}", num);
        
        if num == 10 {
            break;
        }

        num += 1;
    }

    println!("Loop Ended");

    loop1();

    loop2();
}

fn loop1() {
    let mut n = 3;

    while n != 0 {
        println!("{n}");

        n -=1;
    }

    println!("end while loop");

    let arr = [1,2,3,4,5,6];

    let mut index = 0;

    while index < 6 {
        println!("i: {} and v:{}", index, arr[index]);
        index +=1;
    }

}

fn loop2() {
    let arr = [1,2,3,4,5];

    for x in arr {
        println!("x = {}", x)
    }
}