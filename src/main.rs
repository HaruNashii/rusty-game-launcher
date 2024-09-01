use std::time::Duration;

use crate::
{
    read::read_desktop_files,
    config::read_config_file,
    window::{create_window, render_scene},
    ui::fonts,
    input_handler::only_check_quit,
};

mod config;
mod read;
mod window;
mod ui;
mod input_handler;

fn main() 
{
    let config_file_data = read_config_file();
    let files = read_desktop_files(config_file_data.path_to_scan);

    let (texture_creator, mut canvas) = create_window(config_file_data.window_size); 
    
    loop 
    {
        std::thread::sleep(Duration::from_millis(64));

        let fonts = fonts(&files, &texture_creator);
        render_scene(fonts, &mut canvas);
        only_check_quit();
    }
}
