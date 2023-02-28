use super::*;

#[derive(Debug)]
pub struct NoteTimeMap{
    pub(crate) inflection_points: Vec<(f64,f64,f64)>,
    pub(crate) main_bpm: f64
    // Tuple of (measure, ms per measure, start_ms)
}

impl NoteTimeMap{
    pub fn new(start_bpm:f64,start_time_sig:f64) -> Self{
        let mut inflection_points = Vec::new();
        inflection_points.push((0.0,60000.0/start_bpm * 4.0 * start_time_sig,0.0));
        NoteTimeMap { inflection_points , main_bpm: start_bpm}
    }
    pub fn push(&mut self,note: &Note){
        let mut insert = |shift: f64| {
            let last_point = self.inflection_points.last().unwrap();
            if note.note_time.measure_notation < last_point.0{
                panic!();
            }
            if note.note_time.measure_notation == last_point.0{
                let last_point = self.inflection_points.last_mut().unwrap();
                *last_point = (last_point.0,last_point.1*shift,last_point.2);
                return;
            }
            let now_time = last_point.2 + last_point.1 * (note.note_time.measure_notation - last_point.0);
            let now_mpm = last_point.1 * shift;
            self.inflection_points.push((note.note_time.measure_notation,now_mpm,now_time));
        };

        match note.note_action{
            NoteAction::BPMChange { bpm } => {
                insert(bpm/self.main_bpm);
            },
            NoteAction::SigChange { length } => {
                insert(length);
            },
            _ => {},
        }
    }

    pub(crate) fn search(&self, measure:f64) -> usize{
        let mut low = 0;
        let mut high = self.inflection_points.len();

        while low < high {
            let mid = low + (high - low) / 2;
            if self.inflection_points[mid].0 < measure {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        low
    }

    pub fn get(&self, note_time: &NoteTime) -> f64{
        let index = self.search(note_time.measure_notation);
        let result = self.inflection_points.get(index-1).unwrap();
        return result.2 + result.1 * (note_time.measure_notation-result.0);
    }

}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NoteTime{
    pub measure_notation: f64,
    pub real_time_ms: f64,
}

impl PartialOrd for NoteTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.measure_notation.partial_cmp(&other.measure_notation)
    }
}
