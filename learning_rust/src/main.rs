#[derive(Debug)]
struct Student{
    id:u32,
    name: String
}

impl Student{
    fn display(&self) -> (){
        println!("ID: {}\nName: {}",self.id,self.name);
    }
}
fn main(){
    let mut s = String::from("Hello");

    // Cannot create references of the same varaibles multiple times, so you change the scope of the variable to create simultaneous references
    {
        let x = print_world(&mut s);
        println!("{:?}",x);
    }
    let y = get_first_word(&mut s);

    println!("{:?}",y);

    let mut name = String::from("User");
    let s = details(5, &mut name);

    println!("{:?}",s);

    s.display();
}


fn details(id:u32,s: &mut String)->Student{
    Student { id, name: s.to_string() }
}

fn print_world(s: &mut String)-> &mut String{
    s.push_str(" World!");
    return s
}

fn get_first_word(s: &mut String)-> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    s
}