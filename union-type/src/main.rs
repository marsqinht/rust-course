// 1.🌟 正常情况下我们无法使用 str 类型，但是可以使用 &str 来替代
// fn main() {
//     let _s: &str = "hello, world";
// }

// 🌟🌟 如果要使用 str 类型，只能配合 Box。 & 可以用来将 Box<str> 转换为 &str 类型
// fn main() {
//     let s: Box<str> = "hello, world".into();
//     greetings(s)
// }

// // fn greetings(s: &str) {
// //     println!("{}",s)
// // }
// fn greetings(s: str) {
//     println!("{}",s)
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {:?}", rect1);
// }

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();
    // 读取控制台的输出
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
