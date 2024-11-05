pub mod channel;
pub mod sample;

use rodio::*;
use std::fs::File;
use std::io::BufReader;

use crate::Math::Float3;
use crate::Audio::sample::*;
use crate::Audio::channel::Channel;

/// General Audio Module used for the playing and listening of audio.
///
/// The Audio Module contains AudioTracks which AudioSamples can be pushed to.
pub struct AudioModule
{
    _listenerPosition: Option<Float3>,

    _stream: OutputStream,
    _handle: OutputStreamHandle,

    _effectChannel: Channel,
    _musicChannel: Channel,
}

impl AudioModule
{
    pub fn Create() -> Self
    {
        let streamOption = OutputStream::try_default();

        if streamOption.is_err()
        {
            panic!("Failed to create stream: {}", streamOption.err().unwrap());
        }

        let (stream, stream_handle) = 
            OutputStream::try_default().unwrap();

        let musicSink = Sink::try_new(&stream_handle).unwrap();
        let effectSink = Sink::try_new(&stream_handle).unwrap();

        Self
        {
            _stream: stream,
            _handle: stream_handle,
            _musicChannel: Channel::Create("Music", musicSink),
            _effectChannel: Channel::Create("SFX", effectSink),
            _listenerPosition: None
        }
    }

    pub fn PlayAudio(&mut self, sample: &AudioSample)
    {
        match &sample.Track()
        {
            ETargetTrack::Music => self._musicChannel.PlayTrack(sample),
            ETargetTrack::Effect => self._effectChannel.PlayTrack(sample),
        } 
    }

    pub fn Update(&mut self)
    {
        self._effectChannel.Update();
        self._musicChannel.Update();
    }
}





