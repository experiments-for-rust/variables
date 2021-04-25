fn main() {
    //let x = 5;
    let mut x = 5;
    println!("{}",x);
    x = 6;
    println!("{}",x);

    const MAX: i32 = 100_000;

    let x = 5; //shadowing
    let x = x + 1; //shadowing again
    let x = x * 2;

    let spaces = "   ";
    let spaces = spaces.len();

    //let mut spaces = "   ";
    //spaces = spaces.len(); variable type changed, not allowed

    let var:i32 = "42".parse().expect("Not a number");
    //let var = "42".parse().expect("Not a number"); target type unknown

    

}
