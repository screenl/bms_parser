use std::{error, io::BufRead};


pub mod obj;
pub mod specs;
pub mod parser;




pub struct BMS{
    pub header: obj::header::Header,
    pub notes: obj::note::Notes,
}

impl BMS{
    pub fn load_from_file(file_path: &str) -> Result<Self, Box<dyn error::Error>>{
        let mut lines = std::io::BufReader::new(std::fs::File::open(file_path)?).lines();
        let header = parser::header_parser(&mut lines)?;
        let notes = parser::body_parser(&header,&mut lines)?;
        Ok(BMS { header: header, notes: notes })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn header_test(){
        let t = BMS::load_from_file("testfiles/observer_spa.bms").unwrap();
        println!("{:#?}",t.header);
    }
    #[test]
    fn notes_test(){
        let t = BMS::load_from_file("testfiles/observer_spa.bms").unwrap();
        println!("{:#?}",t.notes);
    }
}
