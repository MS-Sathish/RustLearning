fn main() {
    let mut num = 6;
    println!("The value of number is {num}");
    num = 7;
    println!("The value of number is {num}");

    //integer
    let a: i32 = 23;
    println!("{a}");
    let a: isize = 23;
    println!("{a}");

    //float 
    let b: f64 = 2.34;
    println!("{b}");

    //bool
    let c:bool = false;
    println!("{c}");

    const arr:[i32;5] = [1,2,3,4,4];
    let first = arr[0];
    println!("{}",first);


    let x:(i32,f64,u8) = (500, 6.4, 1);
    let five = x.0;
    println!("{}",five);

    for number in (1..4) {
        println!("{number}!");
    }

}
