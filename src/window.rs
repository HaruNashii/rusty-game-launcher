use sdl2::pixels::Color;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

use crate::ui::{Fonts, Images};

pub static mut CAMERA_Y_POSITION: i32 = 0;
pub static mut CAMERA_X_POSITION: i32 = 0;

pub fn create_window(window_size: &[u32]) -> (TextureCreator<WindowContext>, Canvas<Window>, sdl2::EventPump)
{
    let sdl_started = sdl2::init().unwrap();
    let video_system = sdl_started.video().unwrap();
    let window = video_system.window("rusty-game-launcher", window_size[0], window_size[1]).position_centered().build().map_err(|e| e.to_string()).unwrap();

    let canvas = window.into_canvas().accelerated().build().map_err(|e| e.to_string()).unwrap();
    let texture_creator = canvas.texture_creator();
    let event_pump = sdl_started.event_pump().unwrap();

    (texture_creator, canvas, event_pump)
}



pub fn render_scene(current_selected: usize, fonts: &Fonts, images: &Images, foreground_color: &[u8], background_color: &[u8], canvas: &mut Canvas<Window>)
{
    canvas.set_draw_color(Color::RGB(background_color[0], background_color[1], background_color[2]));
    canvas.clear();

    if !images.rect_image_selection_vector.is_empty()
    {
        canvas.set_draw_color(Color::RGB(foreground_color[0], foreground_color[1], foreground_color[2]));
        canvas.fill_rect(images.rect_image_selection_vector[current_selected]).unwrap();
    }

    if !fonts.ui_vector.is_empty()
    {
        for (index, rect) in fonts.ui_rect_vector.iter().enumerate()
        {
            canvas.copy(&fonts.ui_vector[index], None, *rect).unwrap();
        };
    }

    if !images.rect_image_vector.is_empty()
    {
        for (index, rect) in images.rect_image_vector.iter().enumerate()
        {
            canvas.copy(&images.image_vector[index], None, *rect).unwrap();
        };
    }

    canvas.present();
}
