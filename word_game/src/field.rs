use std::sync::Arc;

use crate::puzzle::PuzzleFile;
use crate::parse::Parse;

#[derive(Debug)]
pub struct Field;

impl Field {
    pub fn create(input: String) {
        let size = Parse::convert::<u8>(&input).unwrap_or_default();

        let arc: Arc<Vec<Vec<&str>>> = Arc::new(vec![
            vec![
                "str"; 
                (size * 6).into()
            ]; 
            (size * 6).into()
        ]);

        arc.iter().for_each(|vec| {
            println!("{:?}", vec);
        });

        PuzzleFile::read()
    }
}
