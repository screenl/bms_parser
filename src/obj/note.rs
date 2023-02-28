mod note_time;

#[derive(Debug)]
pub enum BGAType{
    Base,
    Layer,
    Poor,
}
#[derive(Debug)]
pub enum NoteAction{
    BGAChange {bga_type: BGAType,id:u16},
    BPMChange {bpm: f64},
    SigChange {length: f64},
    BGM {id: u16},
    Note {id: u16, track: u16},
    LongNote {id: u32, track: u32, end_time: note_time::NoteTime}

}

#[derive(Debug)]
pub struct Note{
    pub note_action: NoteAction,
    pub note_time: note_time::NoteTime,
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn time_test(){
        let mut ntm = note_time::NoteTimeMap::new(280.0,2.0);
        let mut notes = Vec::new();
        notes.push(Note{ note_action: NoteAction::SigChange { length: 0.5 }, note_time: note_time::NoteTime { measure_notation: 1.0, real_time_ms: -1.0 } });
        notes.push(Note{ note_action: NoteAction::SigChange { length: 4.0 }, note_time: note_time::NoteTime { measure_notation: 1.0, real_time_ms: -1.0 } });
        notes.push(Note{ note_action: NoteAction::BPMChange { bpm: 230.0 }, note_time: note_time::NoteTime{measure_notation: 2.0, real_time_ms: -1.0} });
        notes.push(Note{ note_action: NoteAction::SigChange { length: 0.2 }, note_time: note_time::NoteTime { measure_notation: 3.0, real_time_ms: -1.0 } });
        for i in &notes{ntm.push(&i);}
        println!("{:?}",&ntm);
        let time = ntm.get(&note_time::NoteTime { measure_notation: 3.5, real_time_ms: -1.0 });
        println!("{}",time);
    }



}


