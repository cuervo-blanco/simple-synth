use std::{
    sync::mpsc::{Sender, Receiver},
    f32::consts::{E, PI}
    };
use cpal::SampleFormat;
use cpal::traits::{DeviceTrait, StreamTrait};
use ringbuf::{
    traits::{Consumer, Producer, Split, Observer}, 
    HeapRb,
};

#[derive(Debug, Copy, Clone)]
pub enum WaveType {
    Sine, 
    Square,
    Sawtooth,
    Triangle,
}

pub struct Wave {
    frequency: f32,
    amplitude: f32,
    sample_rate: u32,
    channels: usize,
    value: Option<f32>,
    duration: Option<f32>,
}
impl Wave {

    pub fn new (
        frequency: f32, 
        amplitude: f32, 
        sample_rate: u32,
        channels: usize,
        output: Sender<Vec<f32>>,
        buffer_size: usize,
        wave_type: WaveType,
        duration: Option<f32>,
        attack_duration: f32,
        ) -> Self {

        let mut wave = Self {
            frequency,
            amplitude,
            sample_rate,
            channels,
            value: None,
            duration
        };

        std::thread::spawn( move || {
            let mut phase = 0.0 as f32;
            let phase_increment = 2.0 * PI * wave.frequency / wave.sample_rate as f32;
            let mut time = 0.0 as f32;
            let total_samples = wave.sample_rate as f32 * duration.unwrap_or(0.0);
            let attack_samples = (sample_rate as  f32 * (attack_duration / 1000.0)) as f32;
            loop {
                let block: Vec<f32> = (0..buffer_size)
                    .flat_map(|_| {
                        let sample_amplitude = if time * wave.sample_rate as f32 <= attack_samples {
                           wave.amplitude * (time * wave.sample_rate as f32 / attack_samples).min(1.0) 
                        } else if let Some(_duration) = wave.duration {
                            let remaining_samples = total_samples - (time * wave.sample_rate as f32);
                            if remaining_samples <= total_samples {
                                // Exponential Decay
                                let decay_factor = (remaining_samples / total_samples).max(0.0);
                                wave.amplitude * E.powf(-5.0 * (1.0 - decay_factor))
                            } else {
                                wave.amplitude
                            }
                        } else {
                            wave.amplitude
                        };

                        let sample = match wave_type {
                            WaveType::Sine => (phase).sin() * sample_amplitude,
                            WaveType::Square => {
                                if (phase).sin() >= 0.0 {
                                    sample_amplitude
                                } else {
                                    -sample_amplitude
                                }
                            }
                            WaveType::Sawtooth => 
                                (2.0 * sample_amplitude / PI) * (phase - PI / 2.0),
                            WaveType::Triangle => {
                                2.0 * sample_amplitude * (2.0 * (phase / (2.0 * PI) - (phase / (2.0 * PI)).floor()) - 1.0).abs() - sample_amplitude
                            },
                        };


                        phase += phase_increment;
                        if phase > 2.0 * PI {
                            phase -= 2.0 * PI;
                        }
                        time += 1.0 / wave.sample_rate as f32;

                        wave.value = Some(sample);
                        std::iter::repeat(sample).take(channels as usize)
                    })
                .collect();

                if output.send(block).is_err(){
                    println!("Wave::new - Failed to send block, terminating wave
                    wave generator thread");
                    break;
                }

                if let Some(duration) = wave.duration {
                    if time >= duration {
                        break;
                    }
                }
            }
        });

        wave
    }

    pub fn play(
        self, 
        receiver: Receiver<Vec<f32>>,
        buffer_size: usize,
        device: &cpal::Device,
        config: &cpal::SupportedStreamConfig,
        duration: f32,
        ) { 

        let ring = HeapRb::<f32>::new(buffer_size * self.channels);
        let (mut producer, mut consumer) = ring.split();

        std::thread::spawn(move || {
            while let Ok(block) = receiver.recv() {
                for sample in block {
                    while producer.is_full() {
                        std::thread::sleep(std::time::Duration::from_millis(1));
                    }
                    producer.try_push(sample).expect("Wave::play - Failed to push into producer");
                }
            }
        });

        let sample_format = config.sample_format();
        let device = device.clone();
        let config: cpal::StreamConfig = config.clone().into();

        std::thread::spawn( move || {
            let stream = match sample_format {
                SampleFormat::F32 => {
                    device.build_output_stream(
                        &config,
                        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                            for sample in data {
                                *sample = consumer.try_pop().unwrap_or(0.0);
                            }
                        },
                        move |err| {
                            // react to errors here.
                            eprintln!("Failed to output samples into stream: {}", err);
                        },
                        None //None=blocking, Some(Duration)=timeout
                    )
                },
                SampleFormat::I16 => {
                    println!("Not yet implemented(I16)");
                    todo!();
                },
                SampleFormat::U16 => {
                    println!("Not yet implemented (U16)");
                    todo!();
                }
                sample_format => panic!("Unsupported sample format '{sample_format}'")
            }.unwrap();
            stream.play().expect("Wave::play - Failed to play stream");
            let duration: u64 = ((duration + 1.0) * 1000.0) as u64;
            std::thread::sleep(std::time::Duration::from_millis(duration));
        });

    }
}

