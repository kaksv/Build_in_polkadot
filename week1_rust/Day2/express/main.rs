fn main() {
    let x = five();
    println!("x value: {}", x);
    let y = plus_one(10);
    println!("Y value: {}", y);
}

fn five() -> i32 {
    10
}
fn plus_one(my_num: i32) -> i32 {
    my_num + 1
}
