fn main() {
    let a = 2020;

    if a < 10 {
        println!("Less than 10.")
    } else {
        println!("More then 10.")
    }

    // leap year check
    if a % 400 == 0 || (a % 4 == 0 && a % 100 != 0) {
        println!("Leap Year!");
    } else {
        println!("Not Leap Year.");
    }
}
