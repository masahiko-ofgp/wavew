use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use std::io::Write;
use std::f64::{self, consts::PI};

const RIFF_CHUNK_ID: u32 = 0x52494646;     // "RIFF"
const FILE_FORMAT_TYPE: u32 = 0x57415645;  // "WAVE"
const FMT_CHUNK_ID: u32 = 0x666D7420;      // "fmt "
const DATA_CHUNK_ID: u32 = 0x64617461;     // "data"


pub struct MonoPcm {
    fs: u32,
    bits: u16,
    length: u32,
    s: Vec<f64>,
}
impl MonoPcm {
    pub fn new(fs: u32, bits: u16) -> Self {
        MonoPcm {
            fs: fs,
            bits: bits,
            length: fs * 1,
            s: vec![0.0_f64; (fs * 1) as usize],
        }
    }
    pub fn wave_write_8bit<F: Write>(&self, file: &mut F)
        -> std::io::Result<()>
    {
        file.write_u32::<BigEndian>(RIFF_CHUNK_ID)?;
        file.write_u32::<LittleEndian>(36 + self.length)?;
        file.write_u32::<BigEndian>(FILE_FORMAT_TYPE)?;
        file.write_u32::<BigEndian>(FMT_CHUNK_ID)?;
        file.write_u32::<LittleEndian>(16)?;
        file.write_u16::<LittleEndian>(1)?;
        file.write_u16::<LittleEndian>(1)?;
        file.write_u32::<LittleEndian>(self.fs)?;
        file.write_u32::<LittleEndian>(self.fs * (self.bits as u32) / 8)?;
        file.write_u16::<LittleEndian>(self.bits / 8)?;
        file.write_u16::<LittleEndian>(self.bits)?;
        file.write_u32::<BigEndian>(DATA_CHUNK_ID)?;
        file.write_u32::<LittleEndian>(self.length)?;

        for n in 0..(self.length as usize) {
            let mut tmp: f64 =
                (self.s[n] + 1.0) / 2.0 * 256.0;

            if tmp > 255.0 {
                tmp = 255.0;
            }
            else if tmp < 0.0 {
                tmp = 0.0;
            }
            // HACK: I have to think of a better casting method...
            let data: u8 = (tmp + 0.5).round() as u8;
            file.write_u8(data)?;
        }
        if self.length as usize % 2 == 1 {
            let data: u8 = 0;
            file.write_u8(data)?;
        }
        Ok(())
    }
    pub fn wave_write_16bit<F: Write>(&self, file: &mut F)
        -> std::io::Result<()>
    {
        file.write_u32::<BigEndian>(RIFF_CHUNK_ID)?;
        file.write_u32::<LittleEndian>(36 + self.length * 2)?;
        file.write_u32::<BigEndian>(FILE_FORMAT_TYPE)?;
        file.write_u32::<BigEndian>(FMT_CHUNK_ID)?;
        file.write_u32::<LittleEndian>(16)?;
        file.write_u16::<LittleEndian>(1)?;
        file.write_u16::<LittleEndian>(1)?;
        file.write_u32::<LittleEndian>(self.fs)?;
        file.write_u32::<LittleEndian>(self.fs * (self.bits as u32) / 8)?;
        file.write_u16::<LittleEndian>(self.bits / 8)?;
        file.write_u16::<LittleEndian>(self.bits)?;
        file.write_u32::<BigEndian>(DATA_CHUNK_ID)?;
        file.write_u32::<LittleEndian>(self.length * 2)?;

        for n in 0..(self.length as usize) {
            let mut tmp: f64 =
                (self.s[n] + 1.0) / 2.0 * 65536.0;

            if tmp > 65535.0 {
                tmp = 65535.0;
            }
            else if tmp < 0.0 {
                tmp = 0.0;
            }
            // HACK: I have to think of a better casting method...
            let data: i16 = ((tmp + 0.5).round() as i32  - 32768) as i16;
            file.write_i16::<LittleEndian>(data)?;
        }
        Ok(())
    }
}

pub fn sine_wave(pcm: &mut MonoPcm, a: f64, f0: f64) {
    for n in 0..(pcm.length as usize) {
        pcm.s[n] = a * (2.0 * PI * f0 * (n as f64) /
            (pcm.fs as f64)).sin();
    }
}
