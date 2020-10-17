mod frontend;
mod wav;

use crate::frontend::OctaveFrontend;

fn main() {
    let mut octave = OctaveFrontend::new(90);
    octave.read("31323334".to_string());
    octave.finish();
}
