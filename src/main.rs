use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use std::fs::File;
use std::f32::consts::PI;
use std::i16;

const CHUNK_ID: i32 = 0x52494646;  // "RIFF"
const FORMAT: i32 = 0x57415645;  // "WAVE"
const SUB_CHUNK_1_ID: i32 = 0x666D7420;  // "fmt "
const SUB_CHUNK_2_ID: i32 = 0x64617461;  // "data"


fn main() -> std::io::Result<()> {
    let mut file = File::create("sine.wav")?;
    
    let mut pcm = MonoPcm::new(44100_i32, 16_u16);

    file.write_i32::<BigEndian>(CHUNK_ID)?;
    // Chunk Size
    // 36 + SubChunk 2 Size
    file.write_u32::<LittleEndian>(36_u32 + pcm.length)?;
    file.write_i32::<BigEndian>(FORMAT)?;
    file.write_i32::<BigEndian>(SUB_CHUNK_1_ID)?;
    // Sub Chunk 1 Size
    // PCM -> 16
    file.write_i32::<LittleEndian>(16)?;
    // Audio Format
    // PCM = 1
    file.write_u16::<LittleEndian>(1)?;
    // Num channels
    // Mono = 1, Stereo = 2
    file.write_u16::<LittleEndian>(1)?;
    // Sample Rate
    // 8000, 44100 ..
    file.write_i32::<LittleEndian>(pcm.fs)?;
    // Byte Rate
    // SampleRate * NumChannels * BitsPerSample / 8
    file.write_i32::<LittleEndian>(88200)?;
    // Block Align
    // NumChannels * BitsPerSample / 8
    file.write_u16::<LittleEndian>(2)?;
    // Bits per sample
    file.write_u16::<LittleEndian>(pcm.bits)?;
    file.write_i32::<BigEndian>(SUB_CHUNK_2_ID)?;
    // Sub Chunk 2 Size
    file.write_u32::<LittleEndian>(pcm.length)?;

    let f0: f32 = 550.0;
    let amplitude = i16::MAX as f32;

    pcm.add_data(f0, amplitude);

    for i in pcm.data.iter() {
        file.write_i16::<LittleEndian>(*i)?;
    }

    Ok(())
}

#[derive(Debug)]
struct MonoPcm {
    fs: i32,
    bits: u16,
    length: u32,
    data: Vec<i16>,
}

impl MonoPcm {
    fn new(fs: i32, bits: u16) -> MonoPcm {
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
