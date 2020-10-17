mod wav;

use wav::WavStream;

fn main() {
    let mut stream = WavStream::new();

    for frequency in &[500, 600, 400, 500, 800, 900, 1000, 800, 600, 400, 300] {
        stream.wave(0.5, *frequency, 0.22);
    }

    stream.finish();
}
