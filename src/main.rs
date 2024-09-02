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
    let all_objects = create_layout(vec![125, 200, files.len() as i32, OBJECT_PER_LINE, 250, 250]);

    let mut selected_option = 0;
    let mut clicked = false;

    let (texture_creator, mut canvas, mut event_pump) = create_window(config_file_data.window_size); 
    
    loop 
    {
        std::thread::sleep(Duration::from_millis(64));

        let images = images(&all_objects, &files, &texture_creator);
        let fonts = fonts(&all_objects, &files, &texture_creator);
        (clicked, selected_option) = handle_input(selected_option, &all_objects, fonts.ui_rect_vector.len(), &mut event_pump);
        if clicked
        {
            let mut app_flag: u8 = 0;
            if config_file_data.use_gamemode {app_flag = 1}
            if config_file_data.use_gamescope {app_flag = 2}
            if config_file_data.use_gamemode && config_file_data.use_gamescope {app_flag = 3}

            exec_app(app_flag, &config_file_data.gamescope_flags, &files[selected_option].desktop_file_exec);
        };
        render_scene(selected_option, &all_objects, &fonts, &images, &mut canvas);
    }
}
