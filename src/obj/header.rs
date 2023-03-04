use std::{collections::HashMap, path::PathBuf};

use crate::specs::*;

#[derive(Debug)]
pub struct Header{
    pub player: Option<PlayMode>,
    pub genre: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub artist: Option<String>,
    pub subartist: Option<String>,
    pub bpm: Option<f64>,
    pub play_level: Option<u8>,
    pub rank: Option<JudgeRank>,
    pub difficulty: Option<Difficulty>,
    pub total: Option<f64>,
    pub ln_type: Option<LnType>,
    pub stage_file: Option<PathBuf>,
    pub banner: Option<PathBuf>,
    pub wav_files: HashMap<u16, String>,
    pub bmp_files: HashMap<u16, String>,
    pub bpm_options: HashMap<u16, f64>,

}

#[derive(Default, Debug)]
pub struct ParseCommandError(String);
impl std::fmt::Display for ParseCommandError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Can't parse command: `{}`",self.0)
    }
}
impl std::error::Error for ParseCommandError{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}



impl Header{
    pub fn new() -> Self{
        Header { player: None, genre: None, title: None, subtitle: None, artist: None, subartist: None, bpm: None, play_level: None, rank: None, difficulty: None, total: None, ln_type: None, stage_file: None, banner: None, wav_files: HashMap::new(), bmp_files: HashMap::new(), bpm_options: HashMap::new() }
    }

    pub fn push(&mut self,command:&str,arg:&str) -> Result<(),ParseCommandError>{
        if command.starts_with("#BPM") && command != "#BPM"{
            println!("{}",command);
            let id = match u16::from_str_radix(&command[4..6], 36){
                Ok(a) => a,
                _ => {return Err(ParseCommandError(format!("{} {}",command,arg)))}
            };
            self.bpm_options.insert(id, arg.parse().map_err(|_| ParseCommandError(format!("{} {}",command,arg)))?);
            return Ok(())
        }

        if command.starts_with("#BMP"){
            let id = match u16::from_str_radix(&command[4..6], 36){
                Ok(a) => a,
                _ => {return Err(ParseCommandError(format!("{} {}",command,arg)))}
            };
            self.bmp_files.insert(id, arg.to_string());
            return Ok(())
        }

        if command.starts_with("#WAV"){
            let id = match u16::from_str_radix(&command[4..6], 36){
                Ok(a) => a,
                _ => {return Err(ParseCommandError(format!("{} {}",command,arg)))}
            };
            let name = arg.rsplit_once(".");
            self.wav_files.insert(id, name.ok_or(ParseCommandError(format!("{} {}",command,arg)))?.0.to_string());
            return Ok(())
        }

        let arg = arg.to_string();
        match command{
            "#TITLE" => self.title = Some(arg),
            "#GENRE" => self.genre = Some(arg),
            "#PLAYER" => self.player = Some(match arg.as_str(){
                "1" => PlayMode::SinglePlay,
                "2" => PlayMode::CouplePlay,
                "3" => PlayMode::DoublePlay,
                "4" => PlayMode::BattlePlay,
                _ => {return Err(ParseCommandError(format!("{} {}",command,arg)))}
            }),
            "#SUBTITLE" => self.subtitle = Some(arg),
            "#ARTIST" => self.artist = Some(arg),
            "#SUBARTIST" => self.subartist = Some(arg),
            "#BPM" => self.bpm = match arg.parse(){
                Ok(f) => Some(f),
                Err(_) => {return Err(ParseCommandError(format!("{} {}",command,arg)))}
            },
            "#RANK" => self.rank = Some(match arg.as_str(){
                "0" => JudgeRank::VeryHard,
                "1" => JudgeRank::Hard,
                "2" => JudgeRank::Normal,
                "3" => JudgeRank::Easy,
                "4" => JudgeRank::VeryEasy,
                _ => {return Err(ParseCommandError(format!("{} {}",command,arg)))}
            }),
            "#DIFFICULTY" => self.difficulty = Some(match arg.as_str(){
                "1" => Difficulty::Beginner,
                "2" => Difficulty::Normal,
                "3" => Difficulty::Hyper,
                "4" => Difficulty::Another,
                "5" => Difficulty::Insane,
                _ => {return Err(ParseCommandError(format!("{} {}",command,arg)))}
            }),
            "#LNTYPE" => self.ln_type = Some(match arg.as_str(){
                "1" => LnType::Rdm,
                "2" => LnType::Mgq,
                _ => {return Err(ParseCommandError(format!("{} {}",command,arg)))}
            }),
            "#TOTAL" => self.total = Some(match arg.parse(){
                Ok(f) => f,
                _ => {return Err(ParseCommandError(format!("{} {}",command,arg)))} 
            }),
            "#PLAYLEVEL" => self.play_level = Some(match arg.parse() {
                Ok(u) => u,
                _ => {return Err(ParseCommandError(format!("{} {}",command,arg)))} 
            }),
            _ => {}
        }
        //match command
        Ok(())
    }
}

