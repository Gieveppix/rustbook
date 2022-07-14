/* 
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        //active: user1.active,
        //username: user1.username,
        email: String::from("another@example.com"),
        //sign_in_count: user1.sign_in_count,
        ..user1
    };

    println!("{}", user2.username);
}
*/

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0); 
}