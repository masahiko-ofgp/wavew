use std::fs::File;
use wavew::wave::{MonoPcm, StereoPcm, ExampleWave};


fn main() -> std::io::Result<()>{
    let mut f1 = File::create("wavefiles/triangle8.wav")?;
    let mut pcm1 = MonoPcm::new(44100, 8);
    let mut f2 = File::create("wavefiles/triangle16.wav")?;
    let mut pcm2 = MonoPcm::new(44100, 16);

    let f0: f64 = 500.0;
    
    pcm1.triangle_wave(f0);
    pcm1.wave_write_8bit(&mut f1)?;

    pcm2.triangle_wave(f0);
    pcm2.wave_write_16bit(&mut f2)?;


    let mut f3 = File::create("wavefiles/triangle8_stereo.wav")?;
    let mut pcm3 = StereoPcm::new(44100, 8);
    let mut f4 = File::create("wavefiles/triangle16_stereo.wav")?;
    let mut pcm4 = StereoPcm::new(44100, 16);

    pcm3.triangle_wave(f0);
    pcm3.wave_write_8bit(&mut f3)?;

    pcm4.triangle_wave(f0);
    pcm4.wave_write_16bit(&mut f4)?;

    Ok(())
}
