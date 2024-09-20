#[derive(Debug)]
struct Student{
    id:u32,
    name: String,
    physics: u32,
    math: u32,
    chemistry: u32,
    biology: u32,
}

impl Student{
    fn display(&self) -> (){
        println!("ID: {}\nName: {}\nTotal score: {}/100",self.id,self.name,self.score());
    }

    fn score(&self) -> u32{
        (self.biology+self.chemistry+self.math+self.physics)/4
    }
    
    fn update(&mut self,subject:&str,score:u32) -> (){
        if subject == "physics"{
            self.physics = score;
        }
        else if subject == "math"{
            self.math = score;
        }
        else if subject == "chemistry"{
            self.chemistry = score;
        }
        else if subject == "biology"{
            self.biology = score;
        }
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
    let mut s = details(5, &mut name,88,99,77,67);

    println!("{:?}",s);

    s.display();

    s.update("math",0);
    s.display()
}


fn details(id:u32,s: &mut String, physics:u32,biology:u32,math:u32,chemistry:u32,)->Student{
    Student { id, name: s.to_string(),physics,math,chemistry,biology}
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