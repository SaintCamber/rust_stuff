// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }



// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }


// fn main() {
//     let x = 2.0; // f64

//     let y: f32 = 3.0; // f32

//     println!("{x}:x, y:{y}");
// }


// fn main() {
//     // addition
//     let sum = 5 + 10;

//     // subtraction
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let product = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3; // Results in -1

//     // remainder
//     let remainder = 43 % 5;
// }


// fn main(){
//     let t = true;

//     let f: bool = false;

//     println!("{t}{f}")
// }



// strings of type char in rust are denoted with singe quotes and are 4 bytes so
// able to rep more than just ascii
// error i noted: variables are prefered to be snakecase so start with a
// lowercase letter!

fn main(){
    let c = 'C';

    let z: char = 'Z';

    println!("{c} comes before {Z}");
}