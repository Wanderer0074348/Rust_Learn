

fn main(){
    let mut s = String::from("Hello");

    let mut x = print_world(&mut s);
    println!("{:?}",x);
    let y = get_first_word(&mut x);
    println!("{}",y);
    let z = String::from("Value101");
    let a = &z[..3];
    println!("{:?}",a);
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