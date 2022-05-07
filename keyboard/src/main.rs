use std::io;

fn main() {
    println!("Ingresa el nombre de usuario");
    let mut username = String::new(); // Static method
    // Result -> Success or Error
    io::stdin().read_line(&mut username);// Reference, read and write sharing
    let username = username.trim();// Renives enter characters
    println!("Ingresa la edad del usuario");
    let mut age = String::new();
    io::stdin().read_line(&mut age);
    let age = age.trim();
    // Result, parse handles the parse type
    let age: i32 = age.parse().unwrap();
    println!("{} tiene {} años",username,age);
}
