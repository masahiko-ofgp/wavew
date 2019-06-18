use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use std::io::Write;

const CHUNK_ID: u32 = 0x52494646;  // "RIFF"
const FORMAT: u32 = 0x57415645;  // "WAVE"
const SUB_CHUNK_1_ID: u32 = 0x666D7420;  // "fmt "
const SUB_CHUNK_2_ID: u32 = 0x64617461;  // "data"


pub struct RiffChunk {
    riff_chunk_id: u32,
    riff_chunk_size: u32,
    file_format_type: u32,
}

impl RiffChunk {
    pub fn new(size: u32) -> Self {
        RiffChunk {
            riff_chunk_id: CHUNK_ID,
            riff_chunk_size: size,
            file_format_type: FORMAT,
        }
    }
    pub fn write<F: Write>(self, file: &mut F) -> std::io::Result<()> {
        file.write_u32::<BigEndian>(self.riff_chunk_id)?;
        file.write_u32::<LittleEndian>(self.riff_chunk_size)?;
        file.write_u32::<BigEndian>(self.file_format_type)?;
        Ok(())
    }
}

pub struct FmtChunk {
    fmt_chunk_id: u32,
    fmt_chunk_size: u32,
    wave_format_type: i16,
    channel: i16,
    samples_per_sec: u32,
    bytes_per_sec: u32,
    block_size: i16,
    bits_per_sample: i16
}

impl FmtChunk {
    pub fn new(size: u32, wftype: i16, ch: i16, spsec: u32,
           bpsec: u32, bsize: i16, bpsam: i16) -> Self
    {
        FmtChunk {
            fmt_chunk_id: SUB_CHUNK_1_ID,
            fmt_chunk_size: size,
            wave_format_type: wftype,
            channel: ch,
            samples_per_sec: spsec,
            bytes_per_sec: bpsec,
            block_size: bsize,
            bits_per_sample: bpsam,
        }
    }
    pub fn write<F: Write>(self, file: &mut F) -> std::io::Result<()> {
        file.write_u32::<BigEndian>(self.fmt_chunk_id)?;
        file.write_u32::<LittleEndian>(self.fmt_chunk_size)?;
        file.write_i16::<LittleEndian>(self.wave_format_type)?;
        file.write_i16::<LittleEndian>(self.channel)?;
        file.write_u32::<LittleEndian>(self.samples_per_sec)?;
        file.write_u32::<LittleEndian>(self.bytes_per_sec)?;
        file.write_i16::<LittleEndian>(self.block_size)?;
        file.write_i16::<LittleEndian>(self.bits_per_sample)?;
        Ok(())
    }
}

pub struct DataChunk {
    data_chunk_id: u32,
    data_chunk_size: u32,
    data: Vec<i16>,
}

impl DataChunk {
    pub fn new(size: u32) -> Self {
        DataChunk {
            data_chunk_id: SUB_CHUNK_2_ID,
            data_chunk_size: size,
            data: Vec::new(),
        }
    }
    pub fn add_data(&mut self, value: i16) {
        self.data.push(value);
    }
    pub fn write<F: Write>(self, file: &mut F) -> std::io::Result<()> {
        file.write_u32::<BigEndian>(self.data_chunk_id)?;
        file.write_u32::<LittleEndian>(self.data_chunk_size)?;
        for d in self.data.iter() {
            file.write_i16::<LittleEndian>(*d)?;
        }
        Ok(())
    }
}
