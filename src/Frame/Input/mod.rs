use std::collections::HashMap;
use winit::event::MouseButton;
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

    pub fn ResetPressedAndReleased(&mut self)
    {
        Input::ResetMap(&mut self._inputState._is_key_pressed);
        Input::ResetMap(&mut self._inputState._is_key_released);
        Input::ResetMap(&mut self._inputState._is_mouse_button_pressed);
        Input::ResetMap(&mut self._inputState._is_mouse_button_released);
    }

    fn ResetMap<TKey>(map: &mut HashMap<TKey, bool>)
    {
        map.iter_mut().for_each(|x|
            {
                *x.1 = false;
            }
        )
    }

    pub fn SetMouseWheelPixelDelta(&mut self, value: (f64, f64))
    {
        self._inputState._mouseWheelPixelDelta = value;
    }

    pub fn SetMouseWheelLineOffset(&mut self, value: (f32, f32))
    {
        self._inputState._mouseWheelLineDelta = value;
    }

    pub fn GetStateCopy(&self) -> InputState
    {
        self._inputState.clone()
    }

    pub fn SetMousePosition(&mut self, newPosition: (f64, f64))
    {
        self._inputState._mousePosition = newPosition;
    }

    pub fn Mouse_Pressed(&mut self, button: MouseButton)
    {
        self._inputState._is_mouse_button_pressed.insert(button, true);
        self._inputState._is_mouse_button_down.insert(button, true);
        self._inputState._is_mouse_button_released.insert(button, false);
    }

    pub fn Mouse_Release(&mut self, button: MouseButton)
    {
        self._inputState._is_mouse_button_pressed.insert(button, false);
        self._inputState._is_mouse_button_down.insert(button, false);
        self._inputState._is_mouse_button_released.insert(button, true);
    }

    pub fn Key_Pressed(&mut self, keyCode: PhysicalKey)
    {

        self._inputState._is_key_pressed.insert(keyCode, true);
        self._inputState._is_key_down.insert(keyCode, true);
        self._inputState._is_key_released.insert(keyCode, false);
    }

    pub fn Key_Released(&mut self, keyCode: PhysicalKey)
    {
        self._inputState._is_key_pressed.insert(keyCode, false);
        self._inputState._is_key_down.insert(keyCode, false);
        self._inputState._is_key_released.insert(keyCode, true);
    }

    /// # Description:
    /// Sets all maps to false.
    pub fn reset_maps(&mut self)
    {
        Input::reset_map_state(&mut self._inputState._is_key_released);
        Input::reset_map_state(&mut self._inputState._is_key_pressed);
        Input::reset_map_state(&mut self._inputState._is_key_down);
    }

    fn reset_map_state(key_map: &mut HashMap<PhysicalKey, bool>)
    {
        let mut map = key_map;

        for(_, value) in map.iter_mut()
        {
            *value = false;
        }
    }

    fn reset_mouse_state(button_map: &mut HashMap<MouseButton, bool>)
    {
        let mut map = button_map;

        for(_, value) in map.iter_mut()
        {
            *value = false;
        }
    }
}

#[derive(Clone)]
pub struct InputState
{
    _mousePosition: (f64, f64),
    _mouseWheelPixelDelta: (f64, f64),
    _mouseWheelLineDelta: (f32, f32),


    _is_mouse_button_down: HashMap<MouseButton, bool>,
    _is_mouse_button_pressed: HashMap<MouseButton, bool>,
    _is_mouse_button_released: HashMap<MouseButton, bool>,

    _is_key_down: HashMap<PhysicalKey, bool>,
    _is_key_pressed: HashMap<PhysicalKey, bool>,
    _is_key_released: HashMap<PhysicalKey, bool>,
}

impl InputState
{
    pub fn New() -> Self
    {
        Self
        {
            _mousePosition: (0.0, 0.0),
            _mouseWheelPixelDelta: (0.0, 0.0),
            _mouseWheelLineDelta: (0.0, 0.0),
            _is_mouse_button_down: HashMap::new(),
            _is_mouse_button_pressed: HashMap::new(),
            _is_mouse_button_released: HashMap::new(),

            _is_key_down: HashMap::new(),
            _is_key_pressed: HashMap::new(),
            _is_key_released: HashMap::new()
        }
    }


    /// # Description:
    /// Returns if the provided key is held down.
    pub fn IsKeyDown(&self, key: KeyCode) -> bool
    {
        InputState::CheckKeyMap(&self._is_key_down, key)
    }


    /// ## Description:
    /// Returns if the provided key that was pressed this frame.
    pub fn IsKeyPressed(&self, key: KeyCode) -> bool
    {
        InputState::CheckKeyMap(&self._is_key_pressed, key)
    }

    /// ## Description:
    /// Return if the provided key that was released this frame.
    pub fn IsKeyReleased(&self, key: KeyCode) -> bool
    {
        InputState::CheckKeyMap(&self._is_key_released, key)
    }

    /// Returns the current mouse position in the window.
    /// The values are in pixel units where 0,0 is the top-left of the window.
    pub fn MousePosition(&self) -> (f64, f64)
    {
        self._mousePosition
    }

    pub fn MouseWheelPixelDelta(&self) -> (f64, f64)
    {
        self._mouseWheelPixelDelta
    }

    pub fn MouseWheelLineDelta(&self) -> (f32, f32)
    {
        self._mouseWheelLineDelta
    }

    /// # Description:
    /// Returns if the provided mouse button is held down.
    pub fn IsMouseButtonDown(&self, button: MouseButton) -> bool
    {
        InputState::CheckMouseMap(&self._is_mouse_button_down, button)
    }

    /// ## Description:
    /// Returns if the provided mouse button was pressed this frame.
    pub fn IsMousePressed(&self, button: MouseButton) -> bool
    {
        InputState::CheckMouseMap(&self._is_mouse_button_pressed, button)
    }

    /// ## Description:
    /// Return if the provided mouse button was released this frame.
    pub fn IsMouseReleased(&self, button: MouseButton) -> bool
    {
        InputState::CheckMouseMap(&self._is_mouse_button_released, button)
    }

    fn CheckKeyMap(map : &HashMap<PhysicalKey, bool>, key: KeyCode) -> bool
    {
        let key_code = PhysicalKey::Code(key);

        if !map.contains_key(&key_code)
        {
            return false;
        }

        return map[&key_code]
    }

    fn CheckMouseMap(map : &HashMap<MouseButton, bool>, button: MouseButton) -> bool
    {
        if !map.contains_key(&button)
        {
            return false;
        }

        return map[&button]
    }
}

