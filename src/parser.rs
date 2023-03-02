use std::{io::{Lines, BufReader}, fs::File, error};

use crate::obj::{header::Header, note::Notes};

#[derive(Debug)]
pub struct BPMMissingError;
impl error::Error for BPMMissingError{
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
impl std::fmt::Display for BPMMissingError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Missing Main BPM")
    }
}

pub fn header_parser(input_stream: &mut Lines<BufReader<File>>) -> Result<Header, Box<dyn error::Error>>{
    let mut header = Header::new();
    while let Some(Ok(t)) = input_stream.next(){
        if t.starts_with("#"){
            if let Some((command,arg)) = t.split_once(' '){
                if let Err(e) = header.push(command,arg){
                    println!("{}",e);
                }
            }
            else if t.contains("MAIN DATA FIELD"){
                break;
            }

        }
    }
    Ok(header)
}




pub fn body_parser(header: &Header,input_stream: &mut Lines<BufReader<File>>) -> Result<Notes, Box<dyn error::Error>>{
    let notes = Notes::new(header.bpm.ok_or(Box::new(BPMMissingError))?);
    while let Some(Ok(t)) = input_stream.next(){
        if t.starts_with("#"){
            if let Some((indicator,serial)) = t.split_once(' '){
                let measure_number = &indicator[1..4];
                let track_number = &indicator[4..6];

            }
        }
    }
    Ok(notes)

}


#[cfg(test)]
mod tests{
    use super::*;

}