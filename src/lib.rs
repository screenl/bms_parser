use std::{error, io::BufRead};


pub mod obj;

pub mod parser;



struct BMS{
    pub header: obj::header::Header,
    pub notes: obj::note::Notes,
}

impl BMS{
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn error::Error>>{
        let mut lines = std::io::BufReader::new(std::fs::File::open(file_path)?).lines();
        while let Some(Ok(t)) = lines.next(){
            if t.contains("HEADER FIELD"){
                break;
            }
        }
        let header = parser::header_parser(&mut lines)?;
        let notes = parser::body_parser(1.0,"".to_string());
        Ok(BMS { header: header, notes: notes })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn pass(){
        
    }
}
