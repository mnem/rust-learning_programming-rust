fn main() {
    windows();
}

fn windows() {
    println!("# Trying windows");

    let things = vec!["a", "b", "c", "d", "e", "f",];
    for item in things.windows(3).zip(1..) {
        println!("{}: {:?}", item.1, item.0);
    }
}
