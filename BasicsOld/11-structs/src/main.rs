struct User {
     username: String,
     email: String,
     sign_in_count: u64,
     active: bool   
}

struct Point(f32, f32, f32);

fn main() {
    // ==========================================
    // Creating an immutable instance
    let user1 = User {
        username:  String::from("user1"),
        email: String::from("user!@test.com"),
        active: true,
        sign_in_count: 1,
    };

    // Accessing to struct instance field
    println!("User1 name = {}", user1.username);

    // ==========================================
    // Creating a mutable struct instance
    let mut user = User {
        username:  String::from("user1"),
        email: String::from("user!@test.com"),
        active: true,
        sign_in_count: 1,
    };

    // Mutating struct instance field
    user.username = String::from("othername");
    println!("User1 updated name = {}", user.username);


    // ==========================================
    // Creating a new instance from method using shothand
    let user2 = build_user_strct_update(String::from("fake"), String::from("test@test.io"));

    // ==========================================
    // Structure update syntax
    let mut user3 = User {
        email: String::from("test@email.com"),
        username: String::from("theuser"),
        ..user2 // Using structure update syntax
    };

    println!("user3 name = {}", user3.active);

    // ==========================================
    // Tuple struct
    // Accessing fields using tuple index
    let origin = Point(0.0,0.0,0.0);
    println!("Point ({}, {}, {})", origin.0, origin.1, origin.2);
    // Destructuring a tuple struct
    let Point(x,y,z) = origin;
    println!("Point ({}, {}, {})", x, y , x);

}

fn build_user_simple(username:String, email:String) -> User{
        // Using all field names
        User {
            username:  username,
            email: email,
            active: true,
            sign_in_count: 1,
        }
}

fn build_user_strct_update(username: String, email: String ) -> User {
    // Using field init shorthand
    User {
        username,
        email,
        active: true,
        sign_in_count: 1
    }
}
