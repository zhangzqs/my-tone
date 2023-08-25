use std::{collections::HashMap, ops::Deref, time::Duration};

use rodio::{OutputStream, OutputStreamHandle};

#[derive(Debug, Clone, Copy)]
pub enum NoteName {
    Do,
    DoSharp,
    Re,
    ReSharp,
    Mi,
    Fa,
    FaSharp,
    Sol,
    SolSharp,
    La,
    LaSharp,
    Si,
}

impl From<u8> for NoteName {
    fn from(x: u8) -> Self {
        match x {
            0 => NoteName::Do,
            1 => NoteName::DoSharp,
            2 => NoteName::Re,
            3 => NoteName::ReSharp,
            4 => NoteName::Mi,
            5 => NoteName::Fa,
            6 => NoteName::FaSharp,
            7 => NoteName::Sol,
            8 => NoteName::SolSharp,
            9 => NoteName::La,
            10 => NoteName::LaSharp,
            11 => NoteName::Si,
            _ => panic!("invalid note name"),
        }
    }
}

impl Into<u8> for NoteName {
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NoteType {
    C,
    Cs,
    D,
    Eb,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    Bb,
    B,
}

impl From<u8> for NoteType {
    fn from(x: u8) -> Self {
        match x {
            0 => NoteType::C,
            1 => NoteType::Cs,
            2 => NoteType::D,
            3 => NoteType::Eb,
            4 => NoteType::E,
            5 => NoteType::F,
            6 => NoteType::Fs,
            7 => NoteType::G,
            8 => NoteType::Gs,
            9 => NoteType::A,
            10 => NoteType::Bb,
            11 => NoteType::B,
            _ => panic!("invalid note type"),
        }
    }
}

impl Into<u8> for NoteType {
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Octave {
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
}

impl From<u8> for Octave {
    fn from(x: u8) -> Self {
        match x {
            0 => Octave::O1,
            1 => Octave::O2,
            2 => Octave::O3,
            3 => Octave::O4,
            4 => Octave::O5,
            5 => Octave::O6,
            6 => Octave::O7,
            7 => Octave::O8,
            _ => panic!("invalid octave"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NoteDuration {
    WholeDotted,
    Whole,
    HalfDotted,
    Half,
    QuarterDotted,
    Quarter,
    EighthDotted,
    Eighth,
    SixteenthDotted,
    Sixteenth,
    ThirtySecondDotted,
    ThirtySecond,
    SixtyFourthDotted,
    SixtyFourth,
}

impl Into<f64> for NoteDuration {
    fn into(self) -> f64 {
        match self {
            NoteDuration::WholeDotted => 3.0 / 2.0,
            NoteDuration::Whole => 1.0,
            NoteDuration::HalfDotted => 3.0 / 4.0,
            NoteDuration::Half => 1.0 / 2.0,
            NoteDuration::QuarterDotted => 3.0 / 8.0,
            NoteDuration::Quarter => 1.0 / 4.0,
            NoteDuration::EighthDotted => 3.0 / 16.0,
            NoteDuration::Eighth => 1.0 / 8.0,
            NoteDuration::SixteenthDotted => 3.0 / 32.0,
            NoteDuration::Sixteenth => 1.0 / 16.0,
            NoteDuration::ThirtySecondDotted => 3.0 / 64.0,
            NoteDuration::ThirtySecond => 1.0 / 32.0,
            NoteDuration::SixtyFourthDotted => 3.0 / 128.0,
            NoteDuration::SixtyFourth => 1.0 / 64.0,
        }
    }
}

pub trait Play {
    fn play(&self, stream_handle: &OutputStreamHandle, bar_duration: Duration);
}

impl<'a, T> Play for &'a T
where
    T: Play,
{
    fn play(&self, stream_handle: &OutputStreamHandle, bar_duration: Duration) {
        (*self).play(&stream_handle, bar_duration);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AbsulateNotePitch {
    note_type: NoteType,
    octave: Octave,
}

impl AbsulateNotePitch {
    pub fn new(note_type: NoteType, octave: Octave) -> Self {
        AbsulateNotePitch { note_type, octave }
    }

    pub fn add(self, half_tone: i32) -> AbsulateNotePitch {
        let x = self.octave as i32 * 12 + self.note_type as i32;
        let y = x + half_tone;
        let octave = y / 12;
        let note_type = y % 12;
        AbsulateNotePitch::new((note_type as u8).into(), (octave as u8).into())
    }
}

pub struct RelativePitch {
    note_name: NoteName,
    octave: Octave,
}

impl RelativePitch {
    pub fn new(note_name: NoteName, octave: Octave) -> Self {
        RelativePitch { note_name, octave }
    }

    pub fn to_absulate(self, base: AbsulateNotePitch) -> AbsulateNotePitch {
        let b = base.octave as i32 * 12 + base.note_type as i32;
        let d = self.octave as i32 * 12 + self.note_name as i32;

        let octave = (b + d) / 12;
        let note_type = (b + d) % 12;

        AbsulateNotePitch::new((note_type as u8).into(), (octave as u8).into())
    }
}

pub struct Note {
    pitch: AbsulateNotePitch,
    duration: NoteDuration,
}

impl Note {
    pub fn frequency(&self) -> f32 {
        let x = (self.pitch.octave as i32 - 3) * 12 + (self.pitch.note_type as i32 - 9);
        let freq = (2.0f64.powf(x as f64 / 12.0) * 440.0) as f32;
        println!("play freq: {}", freq);
        freq
    }

    pub fn add(self, half_tone: i32) -> Self {
        Self {
            pitch: self.pitch.add(half_tone),
            duration: self.duration,
        }
    }
}

impl Play for Note {
    fn play(&self, stream_handle: &OutputStreamHandle, bar_duration: Duration) {
        let sink = rodio::Sink::try_new(stream_handle).unwrap();
        sink.set_volume(0.5);
        let source = rodio::source::SineWave::new(self.frequency());
        sink.append(source);
        std::thread::sleep(bar_duration.mul_f64(self.duration.into()));
        sink.stop();
    }
}

pub struct Rest {
    duration: NoteDuration,
}

impl Play for Rest {
    fn play(&self, _stream_handle: &OutputStreamHandle, bar_duration: Duration) {
        std::thread::sleep(bar_duration.mul_f64(self.duration.into()));
    }
}

pub struct Player {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    bar_duration: Duration,
}

impl Player {
    pub fn new(bar_duration: Duration) -> Self {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        Player {
            _stream,
            stream_handle,
            bar_duration,
        }
    }

    pub fn from_bpm(bpm: u32) -> Self {
        Player::new(Duration::from_secs_f64(60.0 / bpm as f64) * 4)
    }

    pub fn play(&self, notes: &Vec<Box<dyn Play>>) {
        for note in notes {
            note.play(&self.stream_handle, self.bar_duration);
        }
    }
}

/// 一根琴弦上的发音
pub struct GuitarNotePitch {
    base: AbsulateNotePitch,
    position: u8,
}

impl GuitarNotePitch {
    pub fn new(base: AbsulateNotePitch, position: u8) -> Self {
        GuitarNotePitch { base, position }
    }

    fn to_absulate(&self) -> AbsulateNotePitch {
        let note_type = (self.base.note_type as u8 + self.position) % 12;
        let octave = self.base.octave as u8 + (self.base.note_type as u8 + self.position) / 12;
        AbsulateNotePitch::new(note_type.into(), octave.into())
    }
}

pub struct GuitarNote {
    pitch: GuitarNotePitch,
    duration: NoteDuration,
}

impl GuitarNote {
    pub fn to_absulate(&self) -> Note {
        Note {
            pitch: self.pitch.to_absulate(),
            duration: self.duration,
        }
    }
}

pub struct Guitar {
    base: HashMap<GuitarString, AbsulateNotePitch>,
    notes: Vec<GuitarNote>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GuitarString {
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
}

impl Guitar {
    pub fn new() -> Self {
        Guitar {
            base: HashMap::from_iter(vec![
                (
                    GuitarString::S1,
                    AbsulateNotePitch::new(NoteType::E, Octave::O4),
                ),
                (
                    GuitarString::S2,
                    AbsulateNotePitch::new(NoteType::B, Octave::O3),
                ),
                (
                    GuitarString::S3,
                    AbsulateNotePitch::new(NoteType::G, Octave::O3),
                ),
                (
                    GuitarString::S4,
                    AbsulateNotePitch::new(NoteType::D, Octave::O3),
                ),
                (
                    GuitarString::S5,
                    AbsulateNotePitch::new(NoteType::A, Octave::O2),
                ),
                (
                    GuitarString::S6,
                    AbsulateNotePitch::new(NoteType::E, Octave::O2),
                ),
            ]),
            notes: vec![],
        }
    }

    pub fn add(&mut self, s: GuitarString, position: u8, duration: NoteDuration) {
        self.notes.push(GuitarNote {
            pitch: GuitarNotePitch::new(self.base[&s], position),
            duration: duration,
        });
    }

    pub fn to_absulate_notes(&self) -> Vec<Note> {
        self.notes.iter().map(|x| x.to_absulate()).collect()
    }
}

fn play(i: i32) {
    let mut guitar = Guitar::new();
    for _ in 0..2 {
        guitar.add(GuitarString::S5, 0, NoteDuration::Quarter);
        guitar.add(GuitarString::S3, 0, NoteDuration::Quarter);
        guitar.add(GuitarString::S2, 0, NoteDuration::Quarter);
        guitar.add(GuitarString::S3, 0, NoteDuration::Quarter);
        guitar.add(GuitarString::S1, 0, NoteDuration::Quarter);
        guitar.add(GuitarString::S3, 0, NoteDuration::Quarter);
        guitar.add(GuitarString::S2, 0, NoteDuration::Quarter);
        guitar.add(GuitarString::S3, 0, NoteDuration::Quarter);
    }
    let song = guitar
        .to_absulate_notes()
        .into_iter()
        .map(|x| x.add(i))
        .map(|x| Box::new(x) as Box<dyn Play>)
        .collect();
    let np = Player::from_bpm(120);
    np.play(&song);
}

fn main() {
    for i in 0..12 {
        println!("change {}: ", i);
        play(i);
    }
}