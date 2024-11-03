use rodio::*;
use std::fs::File;
use std::io::BufReader;

use crate::Math::Float3;


pub struct Audio
{
    _listenerPosition: Option<Float3>,

    _stream: OutputStream,
    _handle: OutputStreamHandle,
    _effectSink: Sink,
    _musicSink: Sink,
}

/// General Audio Module used for the playing and listening of audio.
///
/// The Audio Module contains AudioTracks which ca 
impl Audio
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
            _musicSink: musicSink,
            _effectSink: effectSink,
            _listenerPosition: None
        }
    }

    pub fn PlayAudio(&mut self, sample: &AudioSample)
    {
        let fileOption = File::open(&sample._path);

        if fileOption.is_err()
        {
            panic!("Failed to find file at path {}",&sample._path);
        }

        // todo:  maybe possible better to cache either file load and/or decoded file in audio sample.
        let file = BufReader::new(fileOption.unwrap());
        let source: Decoder<BufReader<File>>  = Decoder::new(file).unwrap();

        match &sample._audioType
        {
            ETargetTrack::Music => Audio::PutAudioInSink(source, &mut self._musicSink),
            ETargetTrack::Effect => Audio::PutAudioInSink(source, &mut self._effectSink),
        } 
    }

    fn PutAudioInSink(
        decoder: Decoder<BufReader<File>>, 
        sink: &mut Sink)
    {
        sink.append(decoder);
        //sink.sleep_until_end();
    }
}

pub struct AudioSample
{
    _path: String,
    _volume: f32,
    _loops: bool,
    _audioSpace: EAudioSpace,
    _audioType: ETargetTrack,
}


impl AudioSample
{
    pub fn Create(
        path: String, 
        volume: f32, 
        loops: bool, 
        space: EAudioSpace, 
        soundType: ETargetTrack) -> Self
    {
        Self
        {
            _path: path,
            _volume: volume,
            _loops: loops,
            _audioSpace: space,
            _audioType: soundType
        }
    }
}

pub enum EAudioSpace
{
    Is3D,
    Is2D
}

// bettername?
pub enum ETargetTrack
{
    Effect,
    Music,
}



