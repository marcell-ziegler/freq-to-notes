use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

#[derive(Debug, Clone)]
pub struct Note {
    pub from_freq: f64,
    pub freq: f64,
    pub note: String,
    pub midi: u8,
}
impl Note {
    const NAMES: [&str; 12] = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "C",
    ];
    pub fn from_freq(freq: f64) -> Note {
        let midi = freq_to_midi(freq);
        let note = Note::NAMES[(midi % 12) as usize];
        let octave = midi / 12 - 1;
        let note_str = format!("{}{}", note, octave);
        Note {
            freq: midi_to_std_freq(midi),
            from_freq: freq,
            note: note_str,
            midi,
        }
    }
    pub fn from_midi(midi: u8) -> Note {
        let note = Note::NAMES[(midi % 12) as usize];
        let octave = midi / 12 - 1;
        let note_str = format!("{}{}", note, octave);
        Note {
            note: note_str,
            freq: midi_to_std_freq(midi),
            from_freq: midi_to_std_freq(midi),
            midi,
        }
    }
    // pub fn from_name(name: &str) -> Note {
    //     if name.len() >= 3 {
    //         let note = &name[..2];
    //         let octave: f64 = name[2..].parse().expect("Octave must be a number");
    //
    //     } else {
    //         panic!("Wrong input to Note::from_name");
    //     }
    // }
}
impl Default for Note {
    fn default() -> Self {
        Note {
            from_freq: 440.0,
            freq: 440.0,
            note: String::from("A4"),
            midi: 69,
        }
    }
}
impl PartialEq for Note {
    // Compes Notes by their midi value
    fn eq(&self, other: &Self) -> bool {
        self.from_freq == other.from_freq
    }
}
impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.from_freq.partial_cmp(&other.from_freq)
    }
}
impl Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.note)
    }
}

// Convert a frequency in Hz to midi note
fn freq_to_midi(freq: f64) -> u8 {
    (69.0 + 12.0 * (freq / 440.0).log2()).round() as u8
}

fn midi_to_std_freq(midi: u8) -> f64 {
    440f64 * 2f64.powf((midi as i32 - 69) as f64 / 12f64)
}
