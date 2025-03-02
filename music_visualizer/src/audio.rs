use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;

pub struct SongStatus {
    pub is_paused: bool,
}

pub struct Audio {
    _stream: OutputStream,
    pub sink: Sink,
}

impl Audio {
    pub fn new(audio_file_path: &str) -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();

        let sink = Sink::try_new(&stream_handle).unwrap();

        let file = File::open(audio_file_path).expect("Failed to open file");
        let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio");

        sink.append(source);

        Self {
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

    pub fn status(&self) -> SongStatus {
        SongStatus {
            is_paused: self.sink.is_paused(),
        }
    }
}
