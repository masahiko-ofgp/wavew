use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use std::fs::File;
use std::f32::consts::PI;
use std::i16;


fn main() -> std::io::Result<()> {
    let mut file = File::create("sine.wav")?;
    
    let fs: i32 = 44100;
    let bits: u16 = 16;
    let length: u32 = (fs * 1) as u32;

    // Chunk ID 'RIFF'
    file.write_i32::<BigEndian>(0x52494646)?;
    // Chunk Size
    // 36 + SubChunk 2 Size
    file.write_i32::<LittleEndian>(44136)?;
    // Format 'WAVE'
    file.write_i32::<BigEndian>(0x57415645)?;
    // Sub Chunk 1 ID 'fmt '
    file.write_i32::<BigEndian>(0x666D7420)?;
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
    file.write_i32::<LittleEndian>(fs)?;
    // Byte Rate
    // SampleRate * NumChannels * BitsPerSample / 8
    file.write_i32::<LittleEndian>(88200)?;
    // Block Align
    // NumChannels * BitsPerSample / 8
    file.write_u16::<LittleEndian>(2)?;
    // Bits per sample
    file.write_u16::<LittleEndian>(bits)?;
    // Sub Chunk 2 ID 'data'
    file.write_i32::<BigEndian>(0x64617461)?;
    // Sub Chunk 2 Size
    file.write_u32::<LittleEndian>(length)?;

    let f0: f32 = 550.0;
    let amplitude = i16::MAX as f32;
    
    write_data(&mut file, f0, amplitude, length)?;

    Ok(())
}


// Data
fn write_data(file: &mut File, f0: f32, amplitude: f32, length: u32)
    -> std::io::Result<()>
{
    let sample = |t: f32| (t * f0 * 2.0 * PI).sin();

    for t in (0 .. length).map(|x| x as f32 / 44100.0) {
        file.write_i16::<LittleEndian>(
            (sample(t) * amplitude) as i16
            )?;
    }
    Ok(())
}
