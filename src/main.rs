use std::time::Duration;

use crate::
{
    read::read_desktop_files,
    config::read_config_file,
    window::{create_window, render_scene},
    ui::{create_layout, fonts, images, OBJECT_PER_LINE},
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
    let files = read_desktop_files(config_file_data.path_to_scan);

    let fonts_objects = create_layout(vec![config_file_data.text_position[0], config_file_data.text_position[1], files.len() as i32, OBJECT_PER_LINE, config_file_data.distance_between_texts[0], config_file_data.distance_between_texts[1]]);
    let images_objects = create_layout(vec![config_file_data.image_position[0], config_file_data.image_position[1], files.len() as i32, OBJECT_PER_LINE, config_file_data.distance_between_images[0], config_file_data.distance_between_images[1]]);

    let mut selected_option = 0;

    let (texture_creator, mut canvas, mut event_pump) = create_window(config_file_data.window_size); 
    
    loop 
    {
        std::thread::sleep(Duration::from_millis(64));

        let images = images(&images_objects, &files, &texture_creator);
        let fonts = fonts(&fonts_objects, &files, &texture_creator);
        let (clicked, option_returned) = handle_input(selected_option, &fonts_objects, fonts.ui_rect_vector.len(), &mut event_pump);
        selected_option = option_returned;
        if clicked
        {
            let mut app_flag: u8 = 0;
            if config_file_data.use_gamemode {app_flag = 1}
            if config_file_data.use_gamescope {app_flag = 2}
            if config_file_data.use_gamemode && config_file_data.use_gamescope {app_flag = 3}

            exec_app(app_flag, &config_file_data.gamescope_flags, &files[selected_option].desktop_file_exec);
        };
        render_scene(selected_option, &fonts_objects, &fonts, &images, &mut canvas);
    }
}
