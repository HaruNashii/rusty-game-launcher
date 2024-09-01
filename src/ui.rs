//use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use crate::
{
    window::CAMERA_Y_POSITION,
    read::DesktopFile,
};

pub const OBJECT_PER_LINE: i32 = 3;

fn create_layout(user_grid_data: Vec<i32>) -> Vec<Vec<i32>>
{
        let mut all_objects = Vec::new();
        let mut object_x = user_grid_data[0];
        let mut object_y = user_grid_data[1];

        for current_object in 1..user_grid_data[2] + 1 
        {
            let object: Vec<i32> = vec![object_x, object_y];
            if current_object % user_grid_data[3] == 0 
            {
                object_y += user_grid_data[4];
                object_x = user_grid_data[0]
            }
            else 
            {
                object_x += user_grid_data[5];
            }
            all_objects.push(object);
        }

        all_objects
}


fn font_generator<'a>(texture_creator: &'a TextureCreator<WindowContext>, additional_text: Option<&str>, mut text: String, size: u16, x: i32, y: i32, ) -> (Texture<'a>, Rect) {
    let ttf_context = sdl2::ttf::init().unwrap();
    while text.len() > 20 { text.pop(); };
    if text.len() == 20 { text.push_str("...") };
    
    let font = ttf_context.load_font("/usr/share/fonts/TTF/JetBrainsMono-Bold.ttf", size).unwrap();
    match additional_text 
    {
        Some(some_text) => 
        {
            let surface = font.render(&format!("{}{}", some_text, text)).blended(Color::RGB(255, 255, 255)).unwrap();
            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
            let font_rect = Rect::new(x, y, surface.width(), surface.height());

            (texture, font_rect)
        },

        None => 
        {
            let surface = font.render(&text).blended(Color::RGB(255, 255, 255)).unwrap();
            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
            let font_rect = Rect::new(x, y, surface.width(), surface.height());

            (texture, font_rect)
        },
    }
}




//pub struct Images<'a>
//{
//    pub image_vector: Vec<Texture<'a>>,
//    pub rect_image_vector: Vec<Rect>,
//}
//
//pub fn images(texture_creator: &TextureCreator<WindowContext>) -> Posters
//{
//    let mut image_vector = Vec::new();
//    let mut rect_image_vector = Vec::new();
//    
//    if !path_of_images.is_empty()
//    {
//        let all_objects = create_layout(vec![75, 75, path_of_images.len() as i32, OBJECT_PER_LINE, 175, 250]);
//        for (index, path) in path_of_images.iter().enumerate()
//        {
//            let rect = Rect::new(all_objects[index][0], unsafe{CAMERA_Y_POSITION + all_objects[index][1]}, 140, 100);
//            rect_image_vector.push(rect);
//
//            let texture = texture_creator.load_texture(path).unwrap();
//            image_vector.push(texture);
//        };
//    };
//
//    Images
//    {
//        image_vector,
//        rect_image_vector,
//    }
//}   



pub struct Fonts<'a>
{
    pub ui_vector: Vec<Texture<'a>>,
    pub ui_rect_vector: Vec<Rect>,
}

pub fn fonts<'a>(files: &'a Vec<DesktopFile>, texture_creator: &'a TextureCreator<WindowContext>) -> Fonts<'a>
{
    let mut ui_vector = Vec::new();
    let mut ui_rect_vector = Vec::new();
           
    for (index, file) in files.iter().enumerate()
    {
        let all_objects = create_layout(vec![75, 200, files.len() as i32, OBJECT_PER_LINE, 175, 250]);
        let (ui_fonts_texture, ui_fonts_rect) = font_generator(texture_creator, None, file.desktop_file_name.clone(), 10, all_objects[index][0], unsafe{CAMERA_Y_POSITION} + all_objects[index][1]);
        ui_vector.push(ui_fonts_texture);
        ui_rect_vector.push(ui_fonts_rect);
    }


    Fonts 
    {
        ui_vector,
        ui_rect_vector,
    }
}
