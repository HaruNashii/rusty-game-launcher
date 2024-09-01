use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::process::exit;
use std::time::Duration;

use crate::window::EVENT_PUMP;

pub fn only_check_quit() 
{
    while unsafe{EVENT_PUMP.is_empty()} { std::thread::sleep(Duration::from_millis(500)); };
    let event_pump = unsafe{&mut EVENT_PUMP[0]};

        std::thread::sleep(Duration::from_millis(64));
        for event in event_pump.poll_iter() 
        {
            match event 
            {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => 
                {
                    print!("\x1B[2J\x1B[1;1H");
                    println!("bye bye :3");
                    exit(0);
                }

                _ => {}
            }
        }
}
