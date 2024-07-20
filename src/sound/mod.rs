mod track;

use std::time::Duration;

use anyhow::{Context, Result};
use rodio::{OutputStream, OutputStreamHandle, Source};
use track::{InMemoryTrackDecoder, Track};

pub struct Sound {
    source: InMemoryTrackDecoder,
    stream_handle: OutputStreamHandle,
    duration: Duration,
    /// If this is dropped playback will end & attached
    /// `OutputStreamHandle`s will no longer work.
    #[allow(dead_code)]
    output_stream: OutputStream,
}

impl Sound {
    pub fn new() -> Result<Self> {
        let (output_stream, stream_handle) =
            OutputStream::try_default().context("Failed to retrieve output stream")?;
        let track = Track::Calm.load().context("Failed to decode music file")?;

        Ok(Sound {
            source: track.decoder,
            duration: track.duration,
            output_stream,
            stream_handle,
        })
    }

    pub fn play(self) -> Result<()> {
        self.stream_handle
            .play_raw(self.source.convert_samples())
            .context("Failed to play raw audio")?;
        std::thread::sleep(self.duration);

        Ok(())
    }
}
