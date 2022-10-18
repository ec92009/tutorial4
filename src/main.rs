fn main() {
    let i: u32 = 4;
    let f: f64 = 4.0;
    let b: bool = true;
    let c: char = 'c';
    println!("Hello, world!, {} {} {} {}", i, f, b, c);

    let tup: (i32, bool, char) = (1, true, 's');
    println!("Hello, world!, {} {} {} ", tup.0, tup.1, tup.2);

    let mut tup1: (i32, bool, char) = (1, true, 's');
    tup1.0 = 10;
    println!("Hello, world!, {} {} {} ", tup1.0, tup1.1, tup1.2);

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array(4) = {}", arr[4]);
    arr[4] = 21;
    println!("array(4) = {}", arr[4]);
 
}
