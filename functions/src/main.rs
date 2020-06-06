fn main() {
    println!("Hello, world!");
    function();
    function_x(11);
    let sum = sum(10, 20);
    println!("Sum = {}", sum);
}

fn function() {
    println!("Function");
}

fn function_x(x: i32) {
    println!("The value of X = {}", x);
}

fn sum(x: i32, y: i32) -> i32 {
    let sum = x + y;
    sum 
    // return sum; // another way to return
}
