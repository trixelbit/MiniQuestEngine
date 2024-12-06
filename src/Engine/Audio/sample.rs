

/// An audio sample that can be played by by an audio track.
#[derive(Clone)]
pub struct AudioSample
{
    _path: String,
    _volume: f32,
    _loops: bool,
    _audioSpace: EAudioSpace,
    _audioTrack: ETargetTrack,
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
            _audioTrack: soundType
        }
    }

    // Get Properties

    /// Path to audio file. 
    pub fn Path(&self) -> String
    {
        self._path.clone()
    }

    /// Volume of sample.
    pub fn Volume(&self) -> f32
    {
        self._volume
    }

    /// Does this track loop.
    pub fn Loops(&self) -> bool
    {
        self._loops
    }

    /// Track this sample should be played on.
    pub fn Track(&self) -> ETargetTrack
    {
        self._audioTrack.clone()
    }


    pub fn SetVolume(&mut self, volume: f32)
    {
        self._volume = volume;
    }
}

#[derive(Copy, Clone)]
pub enum EAudioSpace
{
    Is3D,
    Is2D
}

#[derive(Copy, Clone)]
pub enum ETargetTrack
{
    Effect,
    Music,
}

