use crate::wav::WavStream;

enum Note {
    Silence,
    Offset(i32),
}

/// This front-end allows using octaves instead of frequencies.
pub struct OctaveFrontend {
    stream: WavStream,
    starting_note: f32,
    tempo: u32,
}

impl OctaveFrontend {
    pub fn new(tempo: u32) -> Self {
        Self {
            stream: WavStream::new(),
            starting_note: 440.0,
            tempo,
        }
    }

    fn wave(&mut self, frequency: f32, length: f32) {
        let measure = 60.0 * 4.0 / self.tempo as f32; // 4/4 time signature
        self.stream.wave(0.5, frequency, measure * length);
    }

    /// Add the `n`th note away from A4.
    ///
    /// `length` is a percentage of one measure in 4/4 time signature.
    pub fn note(&mut self, n: i32, length: f32) {
        self.wave(self.starting_note * 2f32.powf(n as f32 / 12.0), length);
    }

    /// Return (L, N) of a note composed of `chars`.
    ///
    /// L and N are the first and the last character in the array. The second
    /// element is the sign of N.
    fn parse_note(&self, chars: [char; 3]) -> (f32, Note) {
        let [l, sign, n] = chars;

        if !l.is_digit(16) {
            panic!("L is not a hexadecimal digit");
        }

        let sign = match sign {
            '-' => -1,
            _ => 1,
        };

        let l = (l as i32 - '0' as i32) as f32;

        let n = if n == '_' {
            Note::Silence
        } else if n.is_digit(16) {
            let n = n as i32 - '0' as i32;
            Note::Offset(n * sign)
        } else {
            panic!("L is not a hexadecimal digit nor '_'");
        };

        (l, n)
    }

    /// Read a string of notes encoded like: <L1><N1><L2><L2>...
    ///
    /// Each LN cluster produces a note N away from starting note (can be
    /// negative or '_' for silence) of length 2^(-L + 1) measures.
    pub fn read(&mut self, string: String) {
        let mut i = 0;

        while i < string.len() {
            let ch = |n| string.chars().nth(n).expect("Unexpected end of string");

            let l = ch(i);
            let n_or_sign = ch(i + 1);

            let sign = {
                if n_or_sign == '-' {
                    i += 3;
                    '-'
                } else {
                    i += 2;
                    ' '
                }
            };

            let n = ch(i - 1);

            let (l, n) = self.parse_note([l, sign, n]);

            let length = 2f32.powf(-l + 1.0);

            match n {
                Note::Offset(off) => self.note(off, length),
                Note::Silence => self.wave(0.0, length),
            }
        }

        if i > string.len() {
            panic!("Trailing characters");
        }
    }

    pub fn finish(self) {
        self.stream.finish();
    }
}

#[cfg(test)]
mod tests {
    fn r(notes: &str) {
        super::OctaveFrontend::new(120).read(notes.to_string());
    }

    #[test]
    #[should_panic]
    fn garbage() {
        r("FG");
    }

    #[test]
    #[should_panic]
    fn no_n() {
        r("1");
    }

    #[test]
    fn empty_string() {
        r("");
    }

    #[test]
    #[should_panic]
    fn one_and_a_half_notes() {
        r("101");
    }

    #[test]
    #[should_panic]
    fn weird_sign() {
        r("1-");
    }

    #[test]
    fn one_note() {
        r("10");
        r("1-3");
        r("FF");
        r("F-F");
    }

    #[test]
    fn silence() {
        r("1_");
        r("1-_");
    }

    #[test]
    fn multiple_notes() {
        r("101-1121-3");
    }
}
