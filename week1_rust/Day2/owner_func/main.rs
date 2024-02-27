//Ownership In Functions
fn main() {
    let s = String::from("enjoy");
    takes_ownership(s);

    let t1 =gives_owner("myname".to_string());

    let x =8;
    copy_me(x);


}
fn takes_ownership(my_string: String) {
    println!("{}, on.", my_string);
}
fn copy_me(num:i32) {
    println!("{}",num);
}
// ->
fn gives_owner(declared:String) ->String {
    let string_returned = String::from("here");
    string_returned
}


