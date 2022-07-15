// fn main() {
//     let list = vec![1, 2, 3];
//     println!("Before defining closure: {:?}", list);

//     let only_borrows = || println!("From closure: {:?}", list);

//     println!("Before calling closure: {:?}", list);
//     only_borrows();
//     println!("After calling closure: {:?}", list);
// }



// fn main() {
//     let mut list = vec![1, 2, 3];
//     println!("Before defining closure: {:?}", list);

//     let mut borrows_mutably = || list.push(7);

//     //println!("Before calling closure: {:?}", list);
//     borrows_mutably();
//     println!("After calling closure: {:?}", list);
// }



// impl<T> Option<T> {
//     pub fn unwrap_or_else<F>(self, f: F) -> T
//     where
//         F: FnOnce() -> T
//     {
//         match self {
//             Some(x) => x,
//             None => f(),
//         }
//     }
// }



// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let mut list = [
//         Rectangle {
//             width: 10,
//             height: 1,
//         },
//         Rectangle {
//             width: 3,
//             height: 5,
//         },
//         Rectangle {
//             width: 7,
//             height: 12,
//         },
//     ];

//     // list.sort_by_key(|r| r.width);
//     // println!("{:#?}", list);

//     let mut num_sort_operations = 0;
//     list.sort_by_key(|r| {
//         num_sort_operations += 1;
//         r.width
//     });
//     println!("{:#?}, sorted in {num_sort_operations} operations", list);
// }

//ITERATORS

fn main() {
    // let v1 = vec![1, 2, 3];

    // let v1_iter = v1.iter();
    // println!("{:?}", v1_iter);


    
    // let v1 = vec![1, 2, 3];

    // let v1_iter = v1.iter();

    // for val in v1_iter {
    //     println!("Got: {}", val);
    // }



    // let v1: Vec<i32> = vec![1, 2, 3];
    
    // let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    
    // assert_eq!(v2, vec![2, 3, 4]);



    
}
