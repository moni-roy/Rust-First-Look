fn main() {
    let mut s = String::from("Moni");
    s.push_str(" Roy");
    println!("S = {}", s);

    let ss = s;
    // println!("S = {}", s); // s can't acess here, s value moved to ss
    println!("SS ={}", ss);
}
