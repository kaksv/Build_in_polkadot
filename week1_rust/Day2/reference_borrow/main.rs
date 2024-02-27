
fn main() {
    let  mut s1 = String::from("hello");
    change(&mut s1);
    //$
}
fn change(my_changed: &mut String) {
    my_changed.push_str("Decentracode");
}
