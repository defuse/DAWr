use hound;
use std::i32;
use std::i16;

pub fn load_wav_to_stereo(filename: &str) -> (Vec<f32>, Vec<f32>) {
    let mut reader = hound::WavReader::open(filename).unwrap();
    let spec = reader.spec();
    if spec.channels != 2 || spec.sample_rate != 44100 {
        panic!("Sorry, this only supports 2-channel 44.1kHz WAV.");
    }

    let samples = reader.samples::<i32>();

    let mut left = Vec::<f32>::new();
    let mut right = Vec::<f32>::new();

    let mut channel = 0;

    for sample in samples {
        if channel % 2 == 0 {
            left.push((sample.unwrap() as f32) /  2.0_f32.powf(spec.bits_per_sample as f32));
        } else {
            right.push((sample.unwrap() as f32) / 2.0_f32.powf(spec.bits_per_sample as f32));
        }
        channel = (channel + 1) % 2;
    }

    assert!(left.len() == right.len());

    (left, right)
}

pub fn save_stereo_to_wav(left: &Vec<f32>, right: &Vec<f32>, filename: &str) {
    assert!(left.len() == right.len());

    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(filename, spec).unwrap();

    for i in 0..left.len() {
        let amplitude = i16::MAX as f32;
        writer.write_sample((left[i] * amplitude) as i16).unwrap();
        writer.write_sample((right[i] * amplitude) as i16).unwrap();
    }
}
