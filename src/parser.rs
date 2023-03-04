use std::{io::{Lines, BufReader}, fs::File, error};

use crate::obj::{header::Header, note::Notes};

#[derive(Debug)]
pub enum BPMMissingError{
    MainBPM,
    BPMOption(String),
}
impl error::Error for BPMMissingError{
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
impl std::fmt::Display for BPMMissingError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            BPMMissingError::MainBPM => write!(f,"Missing Main BPM"),
            BPMMissingError::BPMOption(i) => write!(f, "Missing BPM Option {}", i)
        }
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

        }
        else if t.contains("MAIN DATA FIELD"){
            break;
        }
    }
    Ok(header)
}




pub fn body_parser(header: &Header,input_stream: &mut Lines<BufReader<File>>) -> Result<Notes, Box<dyn error::Error>>{
    let mut notes = Notes::new(header.bpm.ok_or(Box::new(BPMMissingError::MainBPM))?);
    while let Some(Ok(t)) = input_stream.next(){
        if t.starts_with("#"){
            if let Some((indicator,serial)) = t.split_once(':'){
                let measure_number = (&indicator[1..4]).parse()?;
                let track_number = &indicator[4..6];
                if track_number=="02"{
                    notes.push(measure_number, track_number, serial)?;
                }
                else if track_number=="08"{
                    notes.push(
                        measure_number, 
                        track_number, 
                        &format!("{}",header.bpm_options
                            .get(&u16::from_str_radix(serial, 36)?)
                            .ok_or(BPMMissingError::BPMOption(serial.to_string()))?))?;
                }   
                else{
                    for i in (0..serial.len()).step_by(2){
                        let info = &serial[i..i+2];
                        if info != "00"{
                            notes.push(measure_number+i as f64/serial.len() as f64, track_number, info)?;
                        }
                    }
                }
                
                
            }
        }
    }
    notes.update_time();
    Ok(notes)

}

