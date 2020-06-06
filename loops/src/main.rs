fn main() {
    loop {
        println!("Loops");
        break;
    }

    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number = number - 1;
    }

    let a = [1, 2, 3, 4, 5];

    for element in a.iter() {
        print!("{} ", element);
    }
}
