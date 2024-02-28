// enum Option<T> {
//     None,
//     Some(T),
// }

fn main() {
   let five = Some(5);
   let six = plus_one(five);
   let none = plus_one(None);

    // Match 

}
fn plus_one(x:Option<i32>) ->Option<i32> {
    match x {
        None =>None,
        Some(x) =>Some(x+1),
    }
}
