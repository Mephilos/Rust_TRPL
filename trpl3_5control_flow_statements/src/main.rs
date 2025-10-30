// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {number}");
// }

// fn main() {
//     let condition = true;

//     let number = if condition { 5 } else { "six" };

//     println!("The value of number is: {number}");
// }
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }
// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }
// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }
// fn main() {
//     for number in (1..4).rev() {
//         println!("{number}!");
//     }
//     println!("LIFTOFF!!!");
// }
// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {element}");
//     }
// }
use std::io;
fn main() {
    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("n번째 피보나치");

    let n: usize = n.trim().parse().expect("스트링에서 변환");

    if n == 0 {
        println!("0");
    }
    let mut pibo: u128 = 0;
    let mut pibo2: u128 = 1;
    let mut pibo_counter = 0;
    loop {
        let temp: u128 = pibo;
        pibo = pibo2;
        pibo2 = temp + pibo;

        pibo_counter += 1;

        if pibo_counter == n {
            break;
        }
    }
    println!("{pibo}");
}
