// UNSAFE FEATURES
// Dereference a raw pointer
// Call an unsafe function or method
// Access or modify a mutable static variable
// Implement an unsafe trait
// Access fields of unions

// fn main() {
//     let mut num = 5;
//     // mutable and imuttable raw pointer
//     let r1 = &num as *const i32;
//     let r2 = &mut num as *mut i32;
// }



// fn main() {
//     let address = 0x012345usize;
//     let r = address as *const i32;
// }



// fn main() {
//     let mut num = 5;

//     let r1 = &num as *const i32;
//     let r2 = &mut num as *mut i32;

//     unsafe {
//         println!("r1 is: {}", *r1);
//         println!("r2 is: {}", *r2);
//     }
// }



// fn main() {
//     unsafe fn dangerous() {}

//     unsafe {
//         dangerous();
//     }
// }



// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5, 6];

//     let r = &mut v[..];

//     let (a, b) = r.split_at_mut(3);

//     assert_eq!(a, &mut [1, 2, 3]);
//     assert_eq!(b, &mut [4, 5, 6]);
// }
