use num::complex::Complex;
use std::ops::{Range, RangeInclusive};

fn _greet_world() {
    let southern_germany = "GE";
    let chinese = "ni hao";
    let english = "hello world";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region)
    }
}

fn _penguin_info() {
    // data to be handled
    let penguin_data = "\
   common name,length (cm)
   Little penguin,33
   Yellow-eyed penguin,65
   Fiordland penguin,60
   Invalid,data
   ";
    let records = penguin_data.lines();
    // println!("{}", penguin_data);
    // println!("{}", &penguin_data)
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }
        // use elastic array
        // it is record oriented programming
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        if cfg!(debug_assertiions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }
        let common_name = fields[0];
        if let Ok(height) = fields[1].parse::<f32>() {
            println!("{}, {}cm", common_name, height)
        }
    }
}

fn _print_boolean() {
    let (a, mut b) = (true, false);
    println!("{:?}, {:?}", a, b);

    b = true;
    assert_eq!(a, b);
}

// struct Struct {
//     e: i32,
// }

// fn match_mode() {
//     let (a, b, c, d, e);
//     (a, b) = (1, 2);
//     [c, .., d, _] = [1, 2, 3, 4, 5];
//     Struct { e } = Struct { e: 5 };
//     assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
// }

fn _warpping_handling() {
    let a: u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b);
}

fn _floating_experiment() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
    println!("abc (f32)");
    println!("    0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("          0.3: {:x}", (abc.2).to_bits());
    println!();
    println!("xyz (f64)");
    println!("    0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("          0.3: {:x}", (xyz.2).to_bits());
    println!();
    assert!(abc.0 + abc.1 == abc.2);
    assert!(xyz.0 + xyz.1 == xyz.2);
    assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.001);
}

fn _nan_experiment() {
    let x = (-42_f32).sqrt();
    // assert_eq!(x, x);
    if x.is_nan() {
        println!("undefined math operation")
    }
}

fn _cal_experiment() {
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;
    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));
    let forty_twos = [42.0, 42f32, 42.0f32];
    println!("{:.2}", forty_twos[0]);
}

fn _bitwise_experiment() {
    // 00000010
    let a: i32 = 2;
    // 00000011
    let b: i32 = 3;
    println!("(a & b) value is {}", a & b);
    println!("(a | b) value is {}", a | b);
    println!("(a ^ b) value is {}", a ^ b);
    println!("(!b) value is {}", !b);
    println!("(a << b) value is {}", a << b);
    println!("(a >> b) value is {}", a >> b);
    let mut a = a;
    a <<= b;
    println!("(a << b) value is {}", a);
}

fn _range_experiment() {
    for i in 1..=5 {
        println!("{}", i);
    }
    for i in 'a'..='z' {
        println!("{}", i);
    }
    for c in 'a'..'z' {
        println!("{}", c as u8);
    }
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
}

fn _complex_experiment() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}

fn _unicode_mem_size() {
    let x = 'z';
    println!("The memory size for 'z' is {}", std::mem::size_of_val(&x));
}

fn _unit_type_experiment() {
    let unit: () = ();
    assert!(std::mem::size_of_val(&unit) == 0);
    assert_eq!(unit, _implicitly_ret_unit());
}
fn _implicitly_ret_unit() {
    println!("I will return a ()");
}

fn _statement_expression_experiment() {
    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        x_cube + x_squared + x
    };
    // let z = {
    //     2 * x;
    // };
    println!("x is {:?}", x);
    println!("y is {:?}", y);
    // println!("z is {:?}", z);
}

fn _print() -> () {
    println!("hello,world");
}

fn _never_return_impl1() -> ! {
    panic!("I return nothing!")
}

fn _never_return_impl2() -> ! {
    loop {
        println!("I return nothing");
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}

fn _get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    _never_return_impl3()
}

fn _never_return_impl3() -> ! {
    // panic!()
    // todo!();
    loop {
        println!("I return nothing");
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}

fn _stack_heap_experiment() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn auto_copy() {
    let x: &str = "hello, world";
    let y = x;
    println!("{},{}", x, y);
}

// fn ownership_experiment() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("{}, world!", s1);
// }

fn main() {
    // greet_world();
    // penguin_info();
    // print_boolean();
    // match_mode();
    // warpping_handling();
    // floating_experiment();
    // nan_experiment();
    // cal_experiment();
    // bitwise_experiment();
    // range_experiment();
    // complex_experiment()
    // unicode_mem_size();
    // unit_type_experiment();
    // statement_expression_experiment();
    // print();
    // never_return_impl1();
    // never_return_impl2();
    // get_option(2);
    // stack_heap_experiment();
    auto_copy();
}
