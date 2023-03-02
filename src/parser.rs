use std::{io::{Lines, BufReader}, fs::File, error};

use crate::obj::{header::Header, note::{Note, Notes}};



pub fn header_parser(input_stream: &mut Lines<BufReader<File>>) -> Result<Header, Box<dyn error::Error>>{
    let first: String;
    let mut header = Header::new();
    while let Some(Ok(t)) = input_stream.next(){
        if t.starts_with("#"){
            if let Some((command,arg)) = t.split_once(' '){
                header.push(command,arg);
            }
            else if t.contains(":"){
                break;
            }

        }
    }
    Ok(header)
}




pub fn body_parser(main_bpm: f64,start:String) -> Notes{
    Notes::new(main_bpm)

}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn header_test(){

    }
}