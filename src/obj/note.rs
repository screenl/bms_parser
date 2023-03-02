mod note_time;

#[derive(Debug,Clone)]
pub enum BGAType{
    Base,
    Layer,
    Poor,
}
#[derive(Debug,Clone)]
pub enum NoteAction{
    BGAChange {bga_type: BGAType,id:u16},
    BPMChange {bpm: f64},
    SigChange {length: f64},
    BGM {id: u16},
    Note {id: u16, track: u16},
    LongNote {id: u32, track: u32, end_time: note_time::NoteTime}

}

#[derive(Debug,Clone)]
pub struct Note{
    pub note_action: NoteAction,
    pub note_time: note_time::NoteTime,
}

#[derive(Debug)]
pub struct Notes{
    notes: Vec<Note>,
    note_time_map: note_time::NoteTimeMap,
}



impl Notes{
    pub fn new(main_bpm: f64) -> Self{
        Notes { notes: vec![] ,note_time_map: note_time::NoteTimeMap::new(main_bpm,1.0)}
    }
    
    pub fn get_timesig_changes(&self) -> Vec<&Note>{
        let mut res = vec![];
        for i in &self.notes{
            if let NoteAction::SigChange { length: _ } = i.note_action {
                res.push(i);
            }
        }
        return res;
    }

    pub fn get_bpm_changes(&self) -> Vec<&Note>{
        let mut res = vec![];
        for i in &self.notes{
            if let NoteAction::BPMChange { bpm: _ } = i.note_action {
                res.push(i);
            }
        }
        return res;
    }

    fn calculate_time_map(&mut self){
        let mut time_changes = vec![];
        for t in self.get_timesig_changes(){
            let mut new = t.clone();
            new.note_time.measure_notation+=1.0;
            if let NoteAction::SigChange {length: i} = t.note_action{
                new.note_action = NoteAction::SigChange { length: 1.0/i };
            }
            time_changes.push(t.clone());
            time_changes.push(new);
        }
        for t in self.get_bpm_changes(){
            time_changes.push(t.clone());
        }
        time_changes.sort_unstable_by(|x,y| x.note_time.partial_cmp(&y.note_time).unwrap());
        for t in time_changes{
            self.note_time_map.push(&t);
        }
    }

    pub fn update_time(&mut self){
        self.calculate_time_map();
        for i in 0..self.notes.len(){
            self.notes[i].note_time.real_time_ms = self.note_time_map.get(&self.notes[i].note_time);
        }
    }

}

impl IntoIterator for Notes{
    type Item = Note;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.notes.into_iter()
    }
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

    #[test]
    fn test_slice(){
        
        
    }
    


}


