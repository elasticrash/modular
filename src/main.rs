use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32::consts::PI;
use std::time::Instant;

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device available");
    let config = device.default_output_config().unwrap();
    
    let sample_rate = config.sample_rate().0 as f32;
    let mut phase: f32 = 0.0;
    let freq = 440.0;
    let step = 2.0 * PI * freq / sample_rate;

     let gate_interval = 100; // 100ms gate
    let mut last_toggle = Instant::now();
    let mut gate_open = true;

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            
            if last_toggle.elapsed().as_millis() >= gate_interval {
                gate_open = !gate_open; // Toggle gate
                last_toggle = Instant::now();
            }

            for sample in data.iter_mut() {
                let square_wave = if phase.sin() >= 0.0 { 0.3 } else { -0.3 };
                //*sample = (phase.sin() * 0.3) as f32;
                //*sample = if phase.sin() >= 0.0 { 0.3 } else { -0.3 };
                *sample = if gate_open { square_wave } else { 0.0 };
                phase = (phase + step) % (2.0 * PI);
            }
        },
        |err| eprintln!("Stream error: {}", err),
        None,
    ).unwrap();

    stream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(3)); // Play for 3 seconds
}
