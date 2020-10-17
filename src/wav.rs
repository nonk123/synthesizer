use std::io::{self, Write};

struct Wave {
    amplitude: i16,
    frequency: u32,
    samples: u32,
}

impl Wave {
    fn new(amplitude: i16, frequency: u32, samples: u32) -> Self {
        Self { amplitude, frequency, samples }
    }

    /// Return the sine wave value on specified sample.
    fn sample(&self, index: u32) -> i16 {
        let t = index as f32 / self.samples as f32;

        let a = self.amplitude as f32;
        let f = self.frequency as f32;

        let tau = 2.0 * std::f32::consts::PI;

        let omega = f * tau;

        let y = a * (omega * t).sin();

        y as i16
    }
}

pub struct WavStream {
    data_size: u32,
    bit_size: u16,
    sample_rate: u32,
    waves: Vec<Wave>,
}

impl WavStream {
    pub fn new() -> Self {
        Self {
            data_size: 0,
            bit_size: 16,
            sample_rate: 44100,
            waves: Vec::new(),
        }
    }

    /// Add a wave with specified amplitude and amount of samples.
    ///
    /// "Abs" stands for "absolute"; the integer parameters are arbitrary.
    pub fn wave_abs(&mut self, amplitude: i16, frequency: u32, samples: u32) {
        self.waves.push(Wave::new(amplitude, frequency, samples));
        // Assuming one channel.
        self.data_size += self.bit_size as u32 / 8 * samples;
    }

    /// Add a wave with "relative" values.
    ///
    /// `amplitude` is a fraction of maximum amplitude.
    /// Amount of samples is calculated from `seconds`.
    pub fn wave(&mut self, amplitude: f32, frequency: u32, seconds: f32) {
        let samples = (seconds * self.sample_rate as f32) as u32;

        let max_amplitude = 2u32.pow(self.bit_size as u32 - 1) as f32;
        let amplitude = (max_amplitude * amplitude) as i16;

        self.wave_abs(amplitude, frequency, samples);
    }

    /// Write `bytes` to stdout.
    fn write(&self, bytes: &[u8]) {
        match io::stdout().write_all(bytes) {
            Err(error) => match error.kind() {
                // Don't panic when e.g. `aplay` closes the pipe.
                io::ErrorKind::BrokenPipe => {}
                _ => panic!(error),
            },
            _ => {}
        };
    }

    /// Write `chars` as ASCII to stdout.
    fn write_chars(&self, chars: &[char]) {
        let to_ascii = |c: &char| {
            if c.is_ascii() {
                *c as u8
            } else {
                panic!("Not an ASCII character: {}", c);
            }
        };

        let ascii: Vec<u8> = chars.iter().map(to_ascii).collect();
        self.write(&ascii);
    }

    /// Write `n` as 32-bit unsigned integer.
    fn write_u32(&self, n: u32) {
        self.write(&n.to_le_bytes());
    }

    /// Write `n` as 16-bit unsigned integer.
    fn write_u16(&self, n: u16) {
        self.write(&n.to_le_bytes());
    }

    /// Write `n` as 16-bit signed integer.
    fn write_i16(&self, n: i16) {
        self.write(&n.to_le_bytes());
    }

    /// Finish building the Wav file and print it to stdout.
    pub fn finish(self) {
        let header_size = 44;
        let file_size = self.data_size + header_size;

        // Header.
        self.write_chars(&['R', 'I', 'F', 'F']);
        self.write_u32(file_size - 8);
        self.write_chars(&['W', 'A', 'V', 'E']);
        self.write_chars(&['f', 'm', 't', ' ']);
        self.write_u32(16);
        self.write_u16(1); // PCM format.
        self.write_u16(1); // Mono.
                           // TODO: add channel count to calculations. It's currently fixed at 1.
        self.write_u32(self.sample_rate);
        self.write_u32(self.sample_rate * self.bit_size as u32 / 8);
        self.write_u16(self.bit_size / 8);
        self.write_u16(self.bit_size);

        // Data chunk.
        self.write_chars(&['d', 'a', 't', 'a']);
        self.write_u32(self.data_size);

        for wave in &self.waves {
            // Assuming 16-bit chunks and one channel.
            for i in 0..wave.samples {
                self.write_i16(wave.sample(i));
            }
        }

        // Padding.
        if self.data_size % 2 == 1 {
            self.write(&[0]);
        }
    }
}
