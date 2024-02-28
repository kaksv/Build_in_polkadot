// struct User {
//     username: String,
//     email: String,
//     active: bool,
//     sign_in_count: u64,
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    // let user1 = User{
    //     username: String::from("decentracode"),
    //     email: String::from("kaks@decentracode.xyz"),
    //     active: true,
    //     sign_in_count: 7,
    // };
    
    let rect1 = Rectangle{
        width: 40,
        height: 50
    };
    println!("The area is {:?}",rect1);
    
    
}
