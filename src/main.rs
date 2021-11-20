fn main() {
    let a=5;
    println!("a={}",a); //5

    let mut b=5;
    println!("b={}",b); //5
    b=6;
    println!("b={} when assigned to a new value",b); //6

    const TADOKORO:i32=114514;
    println!("YATTAZE! {}",TADOKORO);

    let a=a+1; //shadowing
    println!("a={} when the original variable is hidden",a); //6
    {
        let a=a*2;
        println!("a={} when the original variable is hidden again",a); //12
    }
    println!("a={} when the secondary shadowing variable is wasted",a); //6

    let string1="悔い改めて";
    println!("string1={}",string1); //悔い改めて
    let string1=string1.len();
    println!("string1={} when the original variable is hidden",string1); //15 in unicode
    //let mut string2="いいよ来いよ";
    //string2=string2.len(); data type error
    let string2="いいよ来いよ";
    let string2_len=string2.len();
    println!("string2_len={}",string2_len);
}
