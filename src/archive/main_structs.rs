fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut _user1: User = User{
        active: true,
        username: ("Dak07").to_string(),
        email: ("dak07@m.com").to_string(),
        sign_in_count: 2,
    };

    _user1.email = "dak0705@m.com".to_string();

    println!("Email of the user: {}",_user1.email);

}
