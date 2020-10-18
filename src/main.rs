mod frontend;
mod wav;

use crate::frontend::OctaveFrontend;

fn main() {
    let mut octave = OctaveFrontend::new(93);
    octave.read("313-333528_52313-13-3508_51".to_string());
    octave.finish();
}
