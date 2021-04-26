use std::fs::File;
use wavew::wave::{MonoPcm, sine_wave};


fn main() -> std::io::Result<()> {
    let mut f1 = File::create("wavefiles/sine8.wav")?;
    let mut pcm1 = MonoPcm::new(44100, 8);

    let mut f2 = File::create("wavefiles/sine16.wav")?;
    let mut pcm2 = MonoPcm::new(44100, 16);

    let f0: f64 = 500.0;
    let amplitude: f64 = 0.1;

    sine_wave(&mut pcm1, amplitude, f0);
    sine_wave(&mut pcm2, amplitude, f0);
    
    pcm1.wave_write_8bit(&mut f1)?;
    pcm2.wave_write_16bit(&mut f2)?;

    Ok(())
}
