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

fn main() {
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));
}