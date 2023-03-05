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

#[derive(Debug)]
pub struct RNGMissingError;
impl error::Error for RNGMissingError{
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
impl std::fmt::Display for RNGMissingError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Missing RNG Generator")
    }
}

pub struct Parser{
    pub input_stream: Lines<BufReader<File>>,
    pub rng: Option<fn(u16) -> u16>,
}

impl Parser{

    pub fn parse_header(&mut self) -> Result<Header, Box<dyn error::Error>>{
        let mut header = Header::new();
        while let Some(Ok(t)) = self.input_stream.next(){
            if t.starts_with("#"){
                if let Some((command,arg)) = t.split_once(' '){
                    header.push(command,arg)?;
                }

            }
            else if t.contains("MAIN DATA FIELD"){
                break;
            }
        }
        Ok(header)
    }




    pub fn parse_body(&mut self, header: &Header) -> Result<Notes, Box<dyn error::Error>>{
        let mut notes = Notes::new(header.bpm.ok_or(Box::new(BPMMissingError::MainBPM))?);
        let mut rng_range:u16 = 0;
        'a: while let Some(Ok(t)) = self.input_stream.next(){
            if !t.starts_with("#"){
                continue;
            }

            if t=="#ENDIF"{
                let mut flag = false;
                while let Some(Ok(line)) = self.input_stream.next(){
                    if let Some((clause,num)) = t.split_once(' '){
                        if clause=="#IF" && num.parse::<u16>()? == rng_range{
                            flag = true;
                        }
                    }
                    if flag && line=="#ENDIF"{
                        break;
                    }
                }
            }
            if let Some((indicator,serial)) = t.split_once(':'){
                let measure_number = (&indicator[1..4]).parse()?;
                let track_number = &indicator[4..6];
                if track_number=="02"{
                    notes.push(measure_number, track_number, serial)?;
                    continue;
                }
                for i in (0..serial.len()).step_by(2){
                    let info = &serial[i..i+2];
                    if info == "00"{
                        continue;
                    }
                    if track_number =="08"{
                        let bpm = format!("{}",header.bpm_options
                            .get(&u16::from_str_radix(info, 36)?)
                            .ok_or(BPMMissingError::BPMOption(info.to_string()))?);
                        notes.push(measure_number+i as f64/serial.len() as f64, track_number, &bpm)?;
                        continue;
                    }
                    notes.push(measure_number+i as f64/serial.len() as f64, track_number, info)?;
                }
            }
            if let Some((indicator,serial)) = t.split_once(' '){
                if indicator == "#RANDOM"{
                    rng_range= serial.parse()?;
                    let number = self.rng.ok_or(RNGMissingError)?(rng_range);
                    while let Some(Ok(line)) = self.input_stream.next(){
                        if let Some((clause,num)) = line.split_once(' '){
                            if clause=="#IF" && num.parse::<u16>()? == number{
                                continue 'a;
                            }
                        }
                    }
                }
            }
            
        }
        notes.update_time();
        Ok(notes)

    }

}


