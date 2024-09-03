use std::time::Duration;

use crate::
{
    read::read_desktop_files,
    config::read_config_file,
    window::{create_window, render_scene},
    ui::{create_layout, fonts, images},
    input_handler::handle_input,
    exec::exec_app,
};

mod config;
mod read;
mod window;
mod ui;
mod input_handler;
mod exec;

fn main() 
{
    let config_file_data = read_config_file();
    let gride_type = 2;

    let files = read_desktop_files(config_file_data.path_to_scan);
    let fonts_objects = create_layout(vec![config_file_data.text_position[0], config_file_data.text_position[1], files.len() as i32, config_file_data.object_per_line, config_file_data.distance_between_texts[0], config_file_data.distance_between_texts[1]], gride_type);
    let images_objects = create_layout(vec![config_file_data.image_position[0], config_file_data.image_position[1], files.len() as i32, config_file_data.object_per_line, config_file_data.distance_between_images[0], config_file_data.distance_between_images[1]], gride_type);
    let (texture_creator, mut canvas, mut event_pump) = create_window(&config_file_data.window_size); 

    let mut selected_option = 0;
    
    loop 
    {
        let images = images(&images_objects, &files, &texture_creator);
        let fonts = fonts(&fonts_objects, &files, &texture_creator);
        let (clicked, option_returned) = handle_input(config_file_data.image_position[0], config_file_data.image_position[1], config_file_data.window_size[0], config_file_data.window_size[1], gride_type, selected_option, &fonts_objects, config_file_data.object_per_line, fonts.ui_rect_vector.len(), &mut event_pump);

        selected_option = option_returned;
        if clicked
        {
            let mut app_flag: u8 = 0;
            if config_file_data.use_gamemode {app_flag = 1}
            if config_file_data.use_gamescope {app_flag = 2}
            if config_file_data.use_gamemode && config_file_data.use_gamescope {app_flag = 3}

            exec_app(app_flag, &config_file_data.gamescope_flags, &files[selected_option].desktop_file_exec);
        };

        render_scene(selected_option, &fonts, &images, &config_file_data.foreground_color, &config_file_data.background_color, &mut canvas);
    }
}
