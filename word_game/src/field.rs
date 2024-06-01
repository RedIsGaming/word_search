use std::sync::Arc;

use crate::parse::Parse;

#[derive(Debug)]
pub struct Field;

impl Field {
    pub fn new(input: String) {
        let size = Parse::new(&input).unwrap();
        let arc: Arc<Vec<Vec<u8>>> = Arc::new(vec![vec![1; (size * 6).into()]; (size * 6).into()].into());

        arc.iter().for_each(|vec| {
            println!("{:?}", vec);
        })
    }
}
