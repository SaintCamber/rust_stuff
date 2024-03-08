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



// char literals in rust are denoted with singe quotes and are 4 bytes so
// able to rep more than just ascii
// error i noted: variables are prefered to be snakecase so start with a
// lowercase letter!

// string literals use "" double quotes!

// fn main(){
//     let c = 'C';

//     let z: char = 'Z';

//     println!("{c} comes before {Z}");
// }


//rust allows compounds such as tuples as well and has a rather intuitive
//destructuring method in that variables can be assigned as tuple.index and the
//compiler will correctly place the vales from tuple in the slots as shown below.
// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;

//     let six_point_four = x.1;

//     let one = x.2;

//     println!("five_hundred is {five_hundred},six_point_four is {six_point_four}, one is {one}")
// }

//this works as well! it will complain about x and z not being used so they
// //should be used or labled _x/_y to denote the intentional non use.
// fn main() {
//     let tup = (500, 6.4, 1);

//     let (_x, y, _z) = tup;

//     println!("The value of y is: {y}");
// }


// coming from python and js this next bit is probably going to be one that is
// rough for a while...
// arrays in rust have fixed lengths and must be the same data type across all
// values.

// fn main(){
//     let arr = [1,2,3,4,5];

//     println!("{}", arr[3]);
// }

// few observations arrays are indexed in a similar fashion to previous
// languages i've worked with however string interpolation from an array seems
// odd, i had to place empty bracket and list the array value after a comma
// inside the println statement, need to test using multiple values so will do
// that below.

fn main(){
    let arr = [1,2,3,4,5];

    println!("{},{}", arr[3],arr[4]);
}

//comma seperated values worked as intuited!