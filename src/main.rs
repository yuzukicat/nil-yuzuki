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

fn main() {
    // greet_world();
    // penguin_info();
    // print_boolean();
    // match_mode();
    // warpping_handling();
    // floating_experiment();
    // nan_experiment();
    // cal_experiment();
}
