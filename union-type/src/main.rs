// 1.🌟 正常情况下我们无法使用 str 类型，但是可以使用 &str 来替代
// fn main() {
//     let _s: &str = "hello, world";
// }

// 🌟🌟 如果要使用 str 类型，只能配合 Box。 & 可以用来将 Box<str> 转换为 &str 类型
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(s)
}

// fn greetings(s: &str) {
//     println!("{}",s)
// }
fn greetings(s: str) {
    println!("{}",s)
}