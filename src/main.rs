fn _greet_world(){
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

fn _print_boolean(){
    let (a, mut b) = (true, false);
    println!("{:?}, {:?}", a, b);

    b = true;
    assert_eq!(a, b);
}

struct Struct {
    e: i32
}

fn match_mode() {
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e,  } = Struct { e: 5};
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

fn main() {
    // greet_world();
    // penguin_info();
    // print_boolean();
    match_mode();
}
