use rodio::*;
use std::fs::File;
use std::io::BufReader;
use crate::Engine::Audio::sample::*;


/// A track that contains a single stream of audio.
pub struct Channel
{
    pub Name: String,
    _volume: f32,
    _sink: Sink,
    _currentSample: Option<AudioSample>
}

impl Channel
{
    pub fn Create(name: &str, sink: Sink) -> Self
    {
        Self
        {
            Name: String::from(name),
            _volume: 1.0,
            _sink: sink,
            _currentSample: None
        }
    }

    pub fn PlayTrack(&mut self, sample: &AudioSample)
    {
        let fileOption = File::open(&sample.Path());

        if fileOption.is_err()
        {
            panic!("Failed to find file at path {}", &sample.Path());
        }

        // todo:  maybe possible better to cache either file load and/or decoded file in audio sample.
        let file = BufReader::new(fileOption.unwrap());
        let source: Decoder<BufReader<File>>  = Decoder::new(file).unwrap();

        self._sink.stop();
        self._sink.set_volume(sample.Volume() * self._volume);
        self._sink.append(source);
   
        // find way to remove extra copies
        self._currentSample = Some(sample.clone());
    }

    pub fn Update(&mut self)
    {
        // No audio to player in track
        if self._currentSample.is_none()
        {
            return;
        }

        let sample = self._currentSample.clone().unwrap();
        
        // If single play sample finished 
        if !sample.Loops()
        {
            if self._sink.empty()
            {
                self._currentSample = None;
            }

            return;
        }

        // Play track again if stopped
        if self._sink.empty() && sample.Loops()
        {
            self.PlayTrack(&sample);
        }
    }
}




