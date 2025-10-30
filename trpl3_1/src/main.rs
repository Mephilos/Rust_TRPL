fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = 5;

    let x = x + 1;
    let spaces = "   ";

    {
        let x = x * 2;
        println!("The value of x in the inne{spaces}r scope is: {x}");
    }
    let spaces = spaces.len();

    println!("The value o{spaces}f x is: {x}");
}
