use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::process::exit;

use crate::window::{CAMERA_Y_POSITION, CAMERA_X_POSITION};

pub fn handle_input(start_position_x: i32, start_position_y: i32, window_width: u32, window_height: u32, gride_type: usize, mut selected_option: usize, all_objects: &[Vec<i32>], object_per_line: i32, amount_limit: usize, event_pump: &mut sdl2::EventPump) -> (bool, usize)
{
    let first_object_position_x = unsafe{CAMERA_X_POSITION + all_objects[0][0]};
    let last_object_position_x = unsafe{CAMERA_X_POSITION + all_objects.last().unwrap()[0]};

    let first_object_position_y = unsafe{CAMERA_Y_POSITION + all_objects[0][1]};
    let last_object_position_y = unsafe{CAMERA_Y_POSITION +all_objects.last().unwrap()[1]};
    unsafe 
    {
        if first_object_position_x > (start_position_x + 25) { CAMERA_X_POSITION -= 20; };
        if last_object_position_x < (window_width - 250) as i32 { CAMERA_X_POSITION += 20; };

        if first_object_position_y > (start_position_y + 100) { CAMERA_Y_POSITION -= 15; };
        if last_object_position_y < (window_height - 100) as i32 { CAMERA_Y_POSITION += 15; };
    }

    for event in event_pump.poll_iter() 
    {
        match event 
        {                
            //===============================================================================================================//
            //----------------------------------------------------KEYBOARD---------------------------------------------------//
            //===============================================================================================================//
            Event::KeyDown { keycode: Some(Keycode::Up), .. } =>
            {
                match gride_type
                {
                    1 => 
                    {
                        if (selected_option as i32) >= object_per_line
                        {
                            unsafe{CAMERA_Y_POSITION += 175};
                            selected_option -= object_per_line as usize;
                        };
                    }
                    2 =>
                    {
                        if (selected_option as i32) - 1 != -1 as i32
                        {
                            if selected_option as i32 % object_per_line == 0
                            {
                                unsafe{CAMERA_X_POSITION += 175};   
                            };
                            selected_option -= 1;
                        };
                    }
                    _=>{}
                }
            }


            Event::KeyDown { keycode: Some(Keycode::Down), .. } =>
            {
                match gride_type
                {
                    1 => 
                    {
                        if (selected_option + object_per_line as usize) < amount_limit
                        {
                            unsafe{CAMERA_Y_POSITION -= 175};
                            selected_option += object_per_line as usize;
                        };
                    }
                    2 =>
                    {
                        if selected_option + 1 < amount_limit  
                        {                            
                            if (selected_option + 1) as i32 % object_per_line == 0
                            {
                                unsafe{CAMERA_X_POSITION -= 175};
                            };
                            selected_option += 1;
                        };
                    }
                    _=>{}
                }
            }


            Event::KeyDown { keycode: Some(Keycode::Right), .. } => 
            {
                match gride_type
                {
                    1 => 
                    {
                        if selected_option + 1 < amount_limit  
                        {                            
                            if (selected_option + 1) as i32 % object_per_line == 0
                            {
                                unsafe{CAMERA_Y_POSITION -= 175};
                            };
                            selected_option += 1;
                        };
                    }
                    2 => 
                    {
                        if (selected_option + object_per_line as usize) < amount_limit
                        {
                            unsafe{CAMERA_X_POSITION -= 175};
                            selected_option += object_per_line as usize;
                        };

                    }
                    _=>{}
                }
            }

            Event::KeyDown { keycode: Some(Keycode::Left), .. } => 
            {
                match gride_type
                {
                    1 => 
                    {
                        if selected_option >= 1
                        {
                            if selected_option as i32 % object_per_line == 0
                            {
                                unsafe{CAMERA_Y_POSITION += 175};
                            }
                            selected_option -= 1;
                        };
                    }
                    2 =>
                    {
                        if selected_option as usize >= object_per_line as usize
                        {
                            unsafe{CAMERA_X_POSITION += 175};   
                            selected_option -= object_per_line as usize;
                        };
                    }
                    _=>{}
                }
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
                    unsafe
                    {
                        if x >= CAMERA_X_POSITION + object[0] - 50  && x <= CAMERA_X_POSITION + object[0] + 100
                        && y >= CAMERA_Y_POSITION + object[1] - 100 && y <= CAMERA_Y_POSITION + object[1] + 200
                        {
                            selected_option = index;
                        };
                    };
                };
            }

            Event::MouseButtonDown {mouse_btn: MouseButton::Left, ..} =>
            {
                return (true, selected_option);
            }
           
            
            Event::MouseWheel {y, ..} =>
            {
                match gride_type
                {
                    1 =>
                    {
                        unsafe 
                        {
                            if y == 1
                            {
                                CAMERA_Y_POSITION -= 20;
                            };
                            if y == -1
                            {
                                CAMERA_Y_POSITION += 20;
                            };
                        };
                    }
                    2 =>
                    {
                        unsafe 
                        {
                            if y == 1
                            {
                                CAMERA_X_POSITION -= 20;
                            };
                            if y == -1
                            {
                                CAMERA_X_POSITION += 20;
                            };
                        };
                    }
                    _=>{}
                }
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
