// enum Result<T, E> {
//     Ok(()),
//     Err(E),
// }

use std::fs::File;
fn main() {
    let greet_feedback = File::open("hello.txt");
    let greet_file = match greet_feedback{
        Ok(file) => file,
        Err(error)=> panic!("Problem opening file: {:?}", error),
    };
}
