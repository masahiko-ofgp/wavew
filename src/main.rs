use std::fs::File;
use std::f32::consts::PI;
use std::i16;

mod chunks;
use chunks::{RiffChunk, FmtChunk, DataChunk};


fn main() -> std::io::Result<()> {
    let mut file = File::create("wavefiles/sine.wav")?;
    
    let mut pcm = MonoPcm::new(44100_u32, 16_i16);

    let riff = RiffChunk::new(36_u32 + pcm.length);
    riff.write(&mut file)?;

    let fmt = FmtChunk::new(16, 1, 1, pcm.fs, 88200, 2, 16);
    fmt.write(&mut file)?;

    let mut dt = DataChunk::new(pcm.length);
    
    let f0: f32 = 550.0;
    let amplitude = i16::MAX as f32;

    pcm.add_data(f0, amplitude);

    for d in pcm.data.iter() {
        dt.add_data(*d);
    }
    
    dt.write(&mut file)?;

    Ok(())
}

#[derive(Debug)]
struct MonoPcm {
    fs: u32,
    bits: i16,
    length: u32,
    data: Vec<i16>,
}

impl MonoPcm {
    fn new(fs: u32, bits: i16) -> MonoPcm {
        MonoPcm {
            fs: fs,
            bits: bits,
            length: (fs * 1) as u32,
            data: Vec::new(),
        }
    }
    fn add_data(&mut self, f0: f32, amplitude: f32) {
        let sample = |t: f32| (t * f0 * 2.0 * PI).sin();

        for t in (0 .. self.length).map(|x| x as f32 / 44100.0) {
            self.data.push((sample(t) * amplitude) as i16);
        }
    }
}
