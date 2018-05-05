extern crate ears;
extern crate simplemad;
extern crate simplemad_sys;

use ears::{Sound, AudioController};
use simplemad::Decoder;
use simplemad::MadFixed32;

fn play_song(filename: &String) {
    let mut sound = Sound::new(filename).expect("error opening file");
    sound.play();
    while sound.is_playing() {}
}

fn main() {
    // given a filename, we want to print out all the ID3 tags, to start
    // nvm, first let's try out this libmad thingy
    // nvm, let's try out portaudio
    let path = Path::new(&String::from("/Users/rorysawyer/media/audio/toe/the book about my idle plot on a vague anxiety/C.mp3");
    let f = File::open(&path).unwrap();
    let decoder = Decoder::decode(f).unwrap();
    let mut frames: Vec<Vec<MadFixed32>> = Vec::new();
    // TODO: change this to frames.push(frame.sample)
    for result in decoder {
        match result {
            Ok(frame) => frames.push(frame.sample),
            Err(msg) => eprintln!("{:?}", msg),
        }
    }
    println!("{}", frames.len());
    println!("{}", frames[0].bit_rate);
    let sample_rate = frames[0].sample_rate as f64;
    let channels = match frames[0].mode {
        MadMode::SingleChannel => 1,
        _ => 2,
    } as i32;
    let frame_size = frames.len() as u32;
    play_song(&String::from("/Users/rorysawyer/media/audio/toe/the book about my idle plot on a vague anxiety/C.mp3"));
}
