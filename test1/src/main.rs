use std::fmt::{self, Formatter, Display};

#[derive(Debug)]

struct Personal {
    name: &'static str,
    age: u8,
    gender: &'static str,
}

impl Display for Personal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,"{} {} {}", self.name, self.age, self.gender)
    }
}

// #[derive(Debug)]
fn add(a: u8, b: u8) -> u8 {
    return a + b;
}

fn sub(a: u8, b: u8) -> u8 {
    return a - b;
}

fn main() {
    let greeting_text = "Hello Smart Fox";
    println!("Hello, world!");
    println!("{:?}", greeting_text);

    let personal_data = Personal{name:"Smart Fox", age: 20, gender: "man"};
    println!("without Display > {}{}{}",personal_data.name, personal_data.age, personal_data.gender);
    println!("with Display > {}",personal_data);
    println!("sum of two numbers > {:?}",add(8, 8));
    println!("sub of two numbers > {:?}",add(8, 8));
    println!("sub of two numbers > {:?}",add(8, 8));
    println!("sub of two numbers > {:?}",add(8, 8));
    println!("sub of two numbers > {:?}",add(8, 8));
    println!("sub of two numbers > {:?}",add(8, 8));
    println!("sub of two numbers > {:?}",add(8, 8));
}
