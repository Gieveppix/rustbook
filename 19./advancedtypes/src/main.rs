// fn main() {
//     type Kilometers = i32;

//     let x: i32 = 5;
//     let y: Kilometers = 5;

//     println!("x + y = {}", x + y);
// }



// fn main() {
//     type Thunk = Box<dyn Fn() + Send + 'static>;

//     let f: Thunk = Box::new(|| println!("hi"));

//     fn takes_long_type(f: Thunk) {
//         // --snip--
//     }

//     fn returns_long_type() -> Thunk {
//         // --snip--
//         Box::new(|| ())
//     }
// }



// fn main() {
//     print!("forever ");

//     loop {
//         print!("and ever ");
//     }
// }


//NO
// fn main() {
//     let s1: str = "Hello there!";
//     let s2: str = "How's it going?";
// }
