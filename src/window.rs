use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

use crate::ui::{Fonts, Images};

pub static mut CAMERA_Y_POSITION: i32 = 0;

pub fn create_window(window_size: Vec<u32>) -> (TextureCreator<WindowContext>, Canvas<Window>, sdl2::EventPump)
{
    let sdl_started = sdl2::init().unwrap();
    let video_system = sdl_started.video().unwrap();
    let window = video_system.window("rusty-game-launcher", window_size[0], window_size[1]).position_centered().build().map_err(|e| e.to_string()).unwrap();

    let canvas = window.into_canvas().accelerated().build().map_err(|e| e.to_string()).unwrap();
    let texture_creator = canvas.texture_creator();
    let event_pump = sdl_started.event_pump().unwrap();

    (texture_creator, canvas, event_pump)
}



pub fn render_scene(current_selected: usize, font_grid_position: &[Vec<i32>], fonts: &Fonts, images: &Images, canvas: &mut Canvas<Window>)
{
    canvas.set_draw_color(Color::RGB(30, 30, 46));
    canvas.clear();

    if !font_grid_position.is_empty()
    {
        canvas.set_draw_color(Color::RGB(250, 179, 135));
        canvas.fill_rect(Rect::new(font_grid_position[current_selected][0] - 30, font_grid_position[current_selected][1] - 125, 200, 200)).unwrap();
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
