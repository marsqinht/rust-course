// fn main() {
//     // 断言0.1 + 0.2与0.3相等
//     // assert!(0.1 + 0.2 == 0.3);
// }
  
// fn main() {
//     let x = (-42.0_f32).sqrt();
//     if x.is_nan() {
//         println!("未定义的数学行为")
//     }
// }

// fn main() {
//     // 二进制为00000010
//     let a:i32 = 2;
//     // 二进制为00000011
//     let b:i32 = 3;

//     println!("(a & b) value is {}", a & b);

//     println!("(a | b) value is {}", a | b);

//     println!("(a ^ b) value is {}", a ^ b);

//     println!("(!b) value is {} ", !b);

//     println!("(a << b) value is {}", a << b);

//     println!("(a >> b) value is {}", a >> b);

//     let mut a = a;
//     // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
//     a <<= b;
//     println!("(a << b) value is {}", a);
// }
// use num::complex::Complex;

// fn main() {
//     let a = Complex { re: 2.1, im: -1.2 };
//     let b = Complex::new(11.1, 22.2);
//     let result = a + b;
 
//     println!("{} + {}i", result.re, result.im)
//   }

// fn main() {
//     let x = '中';
//     println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));
// }

// fn main () {
//     let x: &str = "hello, world";
//     let y = x;
//     println!("{},{}",x,y);
// }

// fn main() {
//     let s = String::from("hello");  // s 进入作用域

//     takes_ownership(s);             // s 的值移动到函数里 ...
//                                     // ... 所以到这里不再有效

//     let x = 5;                      // x 进入作用域

//     makes_copy(x);                  // x 应该移动函数里，
//                                     // 但 i32 是 Copy 的，所以在后面可继续使用 x

// } // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
//   // 所以不会有特殊操作

// fn takes_ownership(some_string: String) { // some_string 进入作用域
//     println!("{}", some_string);
// } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

// fn makes_copy(some_integer: i32) { // some_integer 进入作用域
//     println!("{}", some_integer);
// } // 这里，some_integer 移出作用域。不会有特殊操作

// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }


fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}