use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::process::exit;

use crate::window::CAMERA_Y_POSITION;

pub fn handle_input(screen_height: i32, mut selected_option: usize, all_objects: &[Vec<i32>], amount_limit: usize, event_pump: &mut sdl2::EventPump) -> (bool, usize)
{
    for event in event_pump.poll_iter() 
    {
        match event 
        {
            //===============================================================================================================//
            //----------------------------------------------------KEYBOARD---------------------------------------------------//
            //===============================================================================================================//
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => 
            {
                    if selected_option + 1 < amount_limit  
                    {
                        selected_option += 1;
                    };
            }

            Event::KeyDown { keycode: Some(Keycode::Left), .. } => 
            {
                    if selected_option >= 1
                    {
                        selected_option -= 1;
                    };
            }


            Event::KeyDown { keycode: Some(Keycode::Return), .. } => 
            {
                return (true, selected_option);
            }



            //===============================================================================================================//
            //------------------------------------------------------MOUSE---------------------------------------------------//
            //===============================================================================================================//
            Event::MouseMotion { x, y, .. } => 
            {
                for (index, object) in all_objects.iter().enumerate()
                {
                    if x >= object[0] - 50 && x <= object[0] + 100 
                    && y >= unsafe{CAMERA_Y_POSITION + object[1] - 100} && y <= unsafe{CAMERA_Y_POSITION + object[1] + 200}
                    {
                        selected_option = index;
                    };
                };
            }

            Event::MouseButtonDown {mouse_btn: MouseButton::Left, ..} =>
            {
                return (true, selected_option);
            }
           
            
            Event::MouseWheel {y, ..} =>
            {
                unsafe 
                {
                    if y == -1 && CAMERA_Y_POSITION <= 0
                    {
                        CAMERA_Y_POSITION += 20;
                    };

                    if y == 1 && CAMERA_Y_POSITION + all_objects.last().unwrap()[1] > (screen_height - 100)
                    {
                        CAMERA_Y_POSITION -= 20;
                    };
                };
            }
            

            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => 
            {
                print!("\x1B[2J\x1B[1;1H");
                println!("bye bye :3");
                exit(0);
            }


            _ => {}
        }
    };
    (false, selected_option)
}   
