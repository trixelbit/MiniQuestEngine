use std::collections::HashMap;
use std::sync::Mutex;
use winit::keyboard::{KeyCode, PhysicalKey};


pub struct Input
{
    _inputState : InputState
}
impl Input
{
    pub fn New() -> Self
    {
        Self
        {
            _inputState: InputState::New()
        }
    }

    pub fn GetStateCopy(&self) -> InputState
    {
        self._inputState.clone()
    }

    /// # Description:
    /// Sets all maps to false.
    pub fn reset_maps(&mut self)
    {
        Input::reset_map_state(&mut self._inputState._released);
        Input::reset_map_state(&mut self._inputState._pressed);
        Input::reset_map_state(&mut self._inputState._is_down);
    }

    fn reset_map_state(key_map: &mut HashMap<PhysicalKey, bool>)
    {
        let mut map = key_map;

        for(_, value) in map.iter_mut()
        {
            *value = false;
        }
    }

    /// # Description:
    /// The state for a given key press.
    pub fn set_key_state(&mut self, keyCode: PhysicalKey, button_state: bool)
    {
        let mut map = &mut self._inputState._is_down;
        if !map.contains_key(&keyCode)
        {
            map.insert(keyCode, button_state);
            return;
        }

        let isKeyDown = self._inputState._is_down[&keyCode];

        // transitioning from un-pressed to pressed
        if !isKeyDown && button_state
        {
            self._inputState._pressed.insert(keyCode, true);
        }

        // transitioning from pressed to un-pressed
        if isKeyDown && !button_state
        {
            self._inputState._released.insert(keyCode, true);
        }

        self._inputState._is_down.insert(keyCode, true);
    }
}

#[derive(Clone)]
pub struct InputState
{
    _is_down: HashMap<PhysicalKey, bool>,
    _pressed: HashMap<PhysicalKey, bool>,
    _released: HashMap<PhysicalKey, bool>
}

impl InputState
{
    pub fn New() -> Self
    {
        Self
        {
            _is_down: HashMap::new(),
            _pressed: HashMap::new(),
            _released: HashMap::new()
        }
    }

    /// # Description:
    /// Returns if the provided key is held down.
    pub fn IsKeyDown(&self, key: KeyCode) -> bool
    {
        InputState::CheckMap(&self._is_down, key)
    }


    /// # Description:
    /// Returns if the provided key was pressed this frame.
    pub fn IsKeyPressed(&self, key: KeyCode) -> bool
    {
        InputState::CheckMap(&self._pressed, key)
    }

    /// # Description:
    /// Return if the provided key was released this frame.
    pub fn IsKeyReleased(&self, key: KeyCode) -> bool
    {
        InputState::CheckMap(&self._released, key)
    }

    fn CheckMap(map : &HashMap<PhysicalKey, bool>, key: KeyCode) -> bool
    {
        let key_code = PhysicalKey::Code(key);

        if !map.contains_key(&key_code)
        {
            return false;
        }

        return map[&key_code]
    }
}

