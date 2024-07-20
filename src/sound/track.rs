use std::{io::Cursor, time::Duration};

use anyhow::{Context, Result};
use rodio::{Decoder, Source};

const CALM: &[u8; 401280] = include_bytes!("../../assets/sound/Calm.mp3");

pub type InMemoryTrackDecoder = Decoder<Cursor<Vec<u8>>>;

pub struct InMemoryTrack {
    pub duration: Duration,
    pub decoder: InMemoryTrackDecoder,
}

pub enum Track {
    Calm,
}

impl Track {
    pub fn bytes(&self) -> &'static [u8] {
        match self {
            Track::Calm => CALM,
        }
    }

    pub fn load(&self) -> Result<InMemoryTrack> {
        let cursor = Cursor::new(self.bytes().to_vec());
        let decoder = Decoder::new(cursor).context("Failed to decode music file")?;
        let duration = decoder.total_duration().unwrap_or(Duration::from_secs(10));

        Ok(InMemoryTrack { decoder, duration })
    }
}
