fn greet_world(){
    let southern_germany = "GE";
    let chinese = "ni hao";
    let english = "hello world";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region)
    }
}

fn main() {
    greet_world();
}
