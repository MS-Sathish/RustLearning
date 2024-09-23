fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); 
    println!("{s}"); 

    let x = 5;
    let y = x;
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world!");
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

}
