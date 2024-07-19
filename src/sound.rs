use std::fs::File;
use std::io::BufReader;

use anyhow::{Context, Result};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};

pub struct Sound {
    source: Decoder<BufReader<File>>,
    stream_handle: OutputStreamHandle,
    /// If this is dropped playback will end & attached
    /// `OutputStreamHandle`s will no longer work.
    #[allow(dead_code)]
    output_stream: OutputStream,
}

impl Sound {
    pub fn new() -> Result<Self> {
        let (output_stream, stream_handle) =
            OutputStream::try_default().context("Failed to retrieve output stream")?;
        let file = File::open("assets/music.mp3").context("Failed to open music file")?;
        let source = Decoder::new(BufReader::new(file)).context("Failed to decode music file")?;

        Ok(Sound {
            source,
            output_stream,
            stream_handle,
        })
    }

    pub fn play(self) -> Result<()> {
        self.stream_handle
            .play_raw(self.source.convert_samples())
            .context("Failed to play raw audio")?;
        std::thread::sleep(std::time::Duration::from_secs(5));

        Ok(())
    }
}
