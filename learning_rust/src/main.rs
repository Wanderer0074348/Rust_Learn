use std::fmt::{self};

#[derive(Debug)]
struct User {
    name: &'static str,
    sign_in_count: i64
}

impl fmt::Display for User{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}:{}",self.name,self.sign_in_count)?;

        write!(f, "\nHello World")
    }
} 


fn main() {
    let user:User = User{sign_in_count: 56, name: "Tanay"};
    // user.sign_in_count = 5;
    println!("{}",user);
    println!("{:?}", user);
}
