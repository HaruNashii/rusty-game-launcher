use sdl2::pixels::Color;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

use crate::ui::Fonts;

pub static mut CAMERA_Y_POSITION: i32 = 0;
pub static mut EVENT_PUMP: Vec<sdl2::EventPump> = Vec::new();


pub fn create_window(window_size: Vec<u32>) -> (TextureCreator<WindowContext>, Canvas<Window>)
{
    let sdl_started = sdl2::init().unwrap();
    let video_system = sdl_started.video().unwrap();
    let window = video_system.window("rusty-game-launcher", window_size[0], window_size[1]).position_centered().build().map_err(|e| e.to_string()).unwrap();

    let canvas = window.into_canvas().accelerated().build().map_err(|e| e.to_string()).unwrap();
    let texture_creator = canvas.texture_creator();
    let event_pump = sdl_started.event_pump().unwrap();

    unsafe{EVENT_PUMP.push(event_pump)};

    (texture_creator, canvas)
}



pub fn render_scene(fonts: Fonts, canvas: &mut Canvas<Window>)
{
    canvas.set_draw_color(Color::RGB(30, 30, 46));
    canvas.clear();

    if !fonts.ui_vector.is_empty()
    {
        for (index, rect) in fonts.ui_rect_vector.iter().enumerate()
        {
            canvas.copy(&fonts.ui_vector[index], None, *rect).unwrap();
        };
    }

    canvas.present();
}
