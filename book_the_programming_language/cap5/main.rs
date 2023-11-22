struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someuser@exemple.com"),
        sign_in_count: 1,
    };


    let user2 = User {
        email: String::from("anotheremail@example.com"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User{
        active: true,
        username,
        email,
        sign_in_count: 1,
    }

}
