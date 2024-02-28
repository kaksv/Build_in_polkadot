fn main() {
    println!("Hello, world!");
    //new empty vector
    let v: Vec<i32> = Vec::new();
    // Vector with values
    let v1 = vec![1,2,3,4,5,7,11];
    let fourth: &i32 = &v1[3];

    //can also access the vector using get fun
    let fourth_value:Option<&i32> = v1.get(3);
    match fourth_value {
        Some(fourth_value) =>println!("Fourth number: {fourth_value}"),
        None => println!("The number doesnt exist"),
    }

}
