use std::fmt;

struct Field(u8, u8);

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl Field {
    fn new(x: u8, y: u8) -> () {
        let mut vec: Vec<Vec<u8>> = vec![vec![1; y.into()]; x.into()];

        for x in vec.iter_mut() {
            for y in x.iter() {
                println!("The field to print: {:?}{}", x, y)
            }
        }
    }
}

fn main() {
    let _field = Field::new(3, 3);
}
