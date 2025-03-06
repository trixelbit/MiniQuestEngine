use glium::Texture2d;


pub struct TextureTicket
{
    _index: usize
}
impl TextureTicket
{
    pub fn Index(&self) -> usize
    {
        self._index
    }

    pub fn Create(index: usize) -> Self
    {
        Self
        {
            _index: index
        }
    }
}


/// The contains all textures that are usabled in game.
///
/// The goal is the prevent redundant multiple allocations of the same textures
/// and supply 1 central area where they can be accessed.
pub struct TextureRepo
{
    // using this instead of hashmap to possibly instead use a ticketing system
    _keys: Vec<String>,
    _textures: Vec<Texture2d>
}


impl TextureRepo
{
    pub fn RegisterTexture(&self, texturePath: &str) -> TextureTicket
    {
        for i in 0..self._keys.len()
        {
            if self._keys[i] == texturePath
            {
                return TextureTicket::Create(i);
            }
        }

        return TextureTicket::Create(self._keys.len());
    }

    pub unsafe fn Request(&self, ticket: TextureTicket) -> *const Texture2d
    {
         &self._textures[ticket.Index()]
    }
}






