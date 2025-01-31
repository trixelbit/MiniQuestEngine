use std::rc::Rc;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use glium::{Display, Surface};
use glium::glutin::surface::WindowSurface;
use crate::Engine::Frame::GameFrame;
use crate::Engine::Frame::Input::Input;
use crate::Engine::GameAPI::GameAPI;
use crate::Entities::Entities;

pub struct CozyPlayer
{

}

impl CozyPlayer
{
    /// General Engine Update cycle.
    pub fn Update(
        display: &Display<WindowSurface>,
        api: &mut GameAPI,
        input: &mut Input,
        timeStart: DateTime<Local>,
        dateTimeLastFrame: &mut DateTime<Local>
    )
    {
        let now = SystemTime::now();
        let mut renderTime: u128 = 0;

        let mut target = display.draw();

        target.clear_color_and_depth((0.1, 0.0, 0.2, 1.0), 1.0);

        api.Audio.Update();

        let timeLastFrame = dateTimeLastFrame.clone();

        let viewMatrix = api.SceneManager.Entities.Camera.ViewMatrix();
        let perspective= api.SceneManager.Entities.Camera.PerspectiveMatrix();

        let frame =
            Rc::new(
                GameFrame::new(
                    input.GetStateCopy(),
                    Local::now() - timeStart,
                    Local::now() - timeLastFrame,
                    viewMatrix,
                    perspective
                )
            );

        Entities::Update(&frame, api, &mut target);
        //scene.Entities.PruneDeadEntities();


        input.ResetPressedAndReleased();
        input.SetMouseWheelPixelDelta((0.0, 0.0));
        input.SetMouseWheelLineOffset((0.0, 0.0));

        *dateTimeLastFrame = Local::now();

        let rnow = SystemTime::now();
        let _ = target.finish();
        display.finish();
        renderTime = renderTime + rnow.elapsed().unwrap().as_millis();

        match now.elapsed()
        {
            Ok(elapsed) =>
                {
                    println!("ms:{} fps:{}", elapsed.as_millis(), (1_000_000_000.0 / elapsed.as_nanos() as f32) as u32);
                    println!("render:{}", renderTime);

                },
            _ => {}
        };
    }

}