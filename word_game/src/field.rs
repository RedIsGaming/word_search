use std::sync::Arc;

use crate::file::WordFile;
use crate::parse::Parse;

#[derive(Debug)]
pub struct Field;

impl Field {
    #[allow(unused)]
    pub fn new(input: String) {
        let size = Parse::new::<u8>(&input).unwrap();

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

        WordFile::read()
    }

    // fn board() -> Vec<&'static str> {
    //     let words: Vec<&str> = vec!["Casual", "Potato", "Pear", "Happy", "Great", "Tidy"];
        
    //     words.iter()
    //         .cloned()
    //         .collect::<Vec<&str>>()
    // }
}
