use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use std::fs::File;
use std::f32::consts::PI;
use std::i16;

fn main() {
    let mut file = File::create("sine.wav").expect("Error");
    
    // Chunk ID 'RIFF'
    file.write_i32::<BigEndian>(0x52494646).unwrap();
    // Chunk Size
    file.write_i32::<LittleEndian>(88244).unwrap();
    // Format 'WAVE'
    file.write_i32::<BigEndian>(0x57415645).unwrap();
    // Sub Chunk 1 ID 'fmt '
    file.write_i32::<BigEndian>(0x666D7420).unwrap();
    // Sub Chunk 1 Size
    file.write_i32::<LittleEndian>(16).unwrap();
    // Audio Format
    file.write_u16::<LittleEndian>(1).unwrap();
    // Num channels
    file.write_u16::<LittleEndian>(2).unwrap();
    // Sample Rate
    file.write_i32::<LittleEndian>(44100).unwrap();
    // Byte Rate
    file.write_i32::<LittleEndian>(176400).unwrap();
    // Block Align
    file.write_u16::<LittleEndian>(4).unwrap();
    // Bits per sample
    file.write_u16::<LittleEndian>(16).unwrap();
    // Sub Chunk 2 ID 'data'
    file.write_i32::<BigEndian>(0x64617461).unwrap();
    // Sub Chunk 2 Size
    file.write_i32::<LittleEndian>(44100).unwrap();

    // Data
    for t in (0 ..44100).map(|x| x as f32 / 44100.0) {
        let sample = (t * 500.0 * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        file.write_i16::<LittleEndian>((sample * amplitude) as i16).unwrap();
    }
}
