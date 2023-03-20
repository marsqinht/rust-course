// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

// fn display_array<T: std::fmt::Debug>(arr: &[T]) {
//     println!("{:?}", arr);
// }
// fn main() {
//     let arr: [i32; 3] = [1, 2, 3];
//     display_array(&arr);

//     let arr: [i32;2] = [1,2];
//     display_array(&arr);
// }

#[derive(Debug)]
enum UiObject {
    Button,
    SelectBox,
}

fn main() {
    let objects = [
        UiObject::Button,
        UiObject::SelectBox
    ];

    for o in objects {
        draw(o)
    }
}

fn draw(o: UiObject) {
    println!("{:?}",o);
}

