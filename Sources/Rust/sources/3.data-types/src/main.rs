use rand::Rnd;

fn get_rand(start: u8, end: u8) -> u8 {
    return rand::thread_rng().gen_range(start..=end);
}

fn main() {
    // Test integer writing
    let v1 = 0xff + 1;
    let v2 = 3;
    let v3 = 0xff + v2;
    print!("{v1}; {v2}; {v3};");

    // tuples can't be indexed
    // let tuple: (str, i8) = ("test", 1);
    // print!("tuple {} number {}", tuple[0], tuple[1]);

    let arr = [get_rand(0, 10); 3];
    println!("Test Random Array {}, {}, {}", arr[0], arr[1], arr[2]);
}
