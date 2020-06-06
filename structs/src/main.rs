struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("moni@abc.com"),
        username: String::from("moni"),
        active: true,
        sign_in_count: 1,
    };

    println!("Email: {}", user1.email);

    let user2: User = build_user("abc@abc.com".to_string(), "Mk".to_string());

    println!("Email: {}", user2.email);
    
    let user3 = User{
        email: String::from("mk@abc.com"),
        ..user1
    };
    println!("Email: {}", user3.email);
    println!("Email: {}", user3.username);
    println!("Email: {}", user3.active);
    println!("Email: {}", user3.sign_in_count);

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
