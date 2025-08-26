#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
}

enum UserError {
    Ok(User),
    Error(String),
}


fn main() {

    let mut users: Vec<User> = Vec::new();
    
    match build_user() {
        UserError::Ok(u)=> users.push(u),
        UserError::Error(e)=> panic!("Error creating user: {}", e),
    } 

    println!("{:?}",users);
}


fn build_user() -> UserError {

    let user = User {
        username:"jlorian".to_string(),
        email:"jlorian@gmail.com".to_string(),
        active: true,
    };
    
    return  UserError::Ok(user);
}

