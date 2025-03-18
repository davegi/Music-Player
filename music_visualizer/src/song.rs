use hound;
use rodio::{OutputStream, Sink, Source, buffer::SamplesBuffer};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

pub struct Song {
    pub name: String,
    _stream: OutputStream, // Keeps the audio stream alive
    pub sink: Sink,        // Controls playback operations
}

impl Song {
    pub fn new(song_file_path: &str) -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let file_path = format!("music_library/{}.wav", song_file_path);
        let reader = hound::WavReader::open(&file_path)
            .expect(&format!("Failed to open WAV file: {}", file_path));

        let spec = reader.spec();
        if spec.sample_format != hound::SampleFormat::Int {
            panic!("Unsupported WAV format: only PCM integer WAV files are supported.");
        }

        let samples: Vec<f32> = reader
            .into_samples::<i16>()
            .map(|s| s.unwrap() as f32 / i16::MAX as f32)
            .collect();

        let source = SamplesBuffer::new(spec.channels as u16, spec.sample_rate, samples).buffered(); // Buffered source for continuous playback

        sink.append(source);

        Self {
            name: song_file_path.to_string(),
            _stream: stream,
            sink,
        }
    }

    pub fn play(&mut self) {
        self.sink.play();
    }

    pub fn pause(&mut self) {
        self.sink.pause();
    }

    pub fn status(&self) -> bool {
        self.sink.is_paused()
    }
}
