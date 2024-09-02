use std::fs::File;
use std::fs;
use std::io::{BufReader, BufRead, Write};
use std::path::Path;
use std::process::Command;



fn convert_bool(vector_of_strings: &[Vec<String>], bounds_take: usize, bounds_skip: usize) -> Vec<bool>
{

    let mut vector_to_return = Vec::new();
    for string_to_convert in vector_of_strings.iter().take(bounds_take).skip(bounds_skip)
    {
        let mut converted_bool = false;
        if string_to_convert[0] == "false"
        {
            converted_bool = false;
        } 
        else if string_to_convert[0] == "true"
        {
            converted_bool = true;
        };

        vector_to_return.push(converted_bool);
    };
    vector_to_return
}

fn convert_u8(vector_of_strings: &[Vec<String>], bounds_take: usize, bounds_skip: usize) -> Vec<u8>
{
    let mut vector_to_return = Vec::new();
    for string_to_parse in vector_of_strings.iter().take(bounds_take).skip(bounds_skip)
    {
        for string in string_to_parse
        {
            let converted_number: u8 = string.parse().unwrap();
            vector_to_return.push(converted_number);
        }
    };
    vector_to_return
}

fn convert_u32(vector_of_strings: &[Vec<String>]) -> Vec<u32>
{
    let mut vector_to_return = Vec::new();
    for string_to_parse in &vector_of_strings[1]
    {
        let converted_number: u32 = string_to_parse.parse().unwrap();
        vector_to_return.push(converted_number);
    };
    vector_to_return
}

fn convert_i32(vector_of_strings: &[Vec<String>], bounds_take: usize, bounds_skip: usize) -> Vec<i32>
{
    let mut vector_to_return = Vec::new();
    for string_to_parse in vector_of_strings.iter().take(bounds_take).skip(bounds_skip)
    {
        for string in string_to_parse
        {
            let converted_number: i32 = string.parse().unwrap();
            vector_to_return.push(converted_number);
        }
    };
    vector_to_return
}





fn get_user_name() -> String
{
    //get user name because the username is also the name of the home directory
    //the env::home_dir() from the rust standart library is deprecated, thats why i'm using this
    //hacky method
    let command = "whoami";
    let mut parts = command.split_whitespace().collect::<Vec<&str>>();
    let stdout = Command::new(parts.remove(0)).args(parts).output().unwrap_or_else(|_| panic!("Failed to execute command '{}'", command)).stdout;
    String::from_utf8(stdout).expect("Stdout was not valid UTF-8").replace("\n", "")
}





fn verify_if_config_file_exist(config_path: String, full_path_of_config_file: &String, default_values: &Vec<String>)
{
    if !Path::new(&config_path).exists()
    {
        fs::create_dir_all(&config_path).unwrap();
    };
    if !Path::new(&full_path_of_config_file).exists()
    {
        let mut config_file = File::create(full_path_of_config_file).unwrap();
        for values in default_values
        {
            write!(config_file, "\n {}", values).unwrap();
        };
    };
}






pub struct ConfigFileData
{
    pub path_to_scan: Vec<String>,
    pub window_size: Vec<u32>,
    pub use_gamemode: bool,
    pub use_gamescope: bool,
    pub gamescope_flags: String,
    pub object_per_line: i32,
    pub text_position: Vec<i32>,
    pub image_position: Vec<i32>,
    pub distance_between_texts: Vec<i32>,
    pub distance_between_images: Vec<i32>,
    pub background_color: Vec<u8>,
    pub foreground_color: Vec<u8>,
}

pub fn read_config_file() -> ConfigFileData
{
    let user_name = get_user_name();
    let mut all_config_file_data_as_string_vectors = Vec::new();
    let config_path = format!("/home/{}/.config/rusty-game-launcher", user_name);
    let config_file_name = "config.i_will_kms";
    let full_path_of_config_file = format!("{}/{}", config_path, config_file_name);


    let options = vec!
    [
        "path_to_scan:",
        "window_size:",
        "use_gamemode:",
        "use_gamescope:",
        "gamescope_flags:",
        "object_per_line:",
        "text_position:",
        "image_position:",
        "distance_between_texts:",
        "distance_between_images:",
        "background_color:",
        "foreground_color:",
    ];
    //define one default value in case the config file 
    // doesn't have one setted up
    let default_values = vec!
    [
        format!("path_to_scan:/usr/share/applications /var/lib/flatpak/exports/share/applications /home/{}/.local/share/flatpak/exports/share/applications /home/{}/.local/share/applications", user_name, user_name),
        "window_size:800 600".to_string(),
        "use_gamemode:true".to_string(),
        "use_gamescope:false".to_string(),
        "gamescope_flags:--fullscreen".to_string(),
        "object_per_line:3".to_string(),
        "text_position:78 185".to_string(),
        "image_position:75 70".to_string(),
        "distance_between_texts:250 250".to_string(),
        "distance_between_images:250 250".to_string(),
        "background_color:30 30 40".to_string(),
        "foreground_color:250 179 135".to_string(),
    ];


    // verify if the config file exist and if it don't 
    // create the file with a holder config
    verify_if_config_file_exist(config_path, &full_path_of_config_file, &default_values);


    // append all the texts from the config file to one vectors of strings
    let file = File::open(&full_path_of_config_file ).unwrap();
    let file_content = BufReader::new(file);
    let lines = file_content.lines();
    for line in lines
    {
        let line_content = line.unwrap();
        for option in &options
        {
            match line_content.find(option)
            {
                Some(..) => 
                {
                    let mut holder: Vec<String> = line_content.replace(option, "").split_whitespace().map(|v| v.to_string()).collect();
                    // prevent the app from crashing when one "gamescope_flag" is not setted
                    if holder.is_empty() 
                    {
                        holder.push(" ".to_string());
                    }
                    all_config_file_data_as_string_vectors.push(holder);
                },
                None => continue,
            }
        }
    }


    let i32_number_config_file_data: i32 = all_config_file_data_as_string_vectors[5][0].parse().unwrap();
    let u8_vector_config_file_data = convert_u8(&all_config_file_data_as_string_vectors, 12, 10);
    let u32_vector_config_file_data = convert_u32(&all_config_file_data_as_string_vectors);
    let i32_vector_config_file_data = convert_i32(&all_config_file_data_as_string_vectors, 10, 6);
    let bool_vector_config_file_data = convert_bool(&all_config_file_data_as_string_vectors, 4, 2);



    // verify if the directory informed in the config file exists 
    // if don't stop the app and print one error message
    let mut remove_list = Vec::new();
    for (index, path_to_scan_from_user) in all_config_file_data_as_string_vectors[0].iter().enumerate()
    {
        if !Path::new(&path_to_scan_from_user).exists()
        {
            println!("\n The Path '{}' Describe In 'path_to_scan' On The Config File '{}' \n Doesn't Exist! \n", path_to_scan_from_user, &full_path_of_config_file);
            remove_list.push(index);
        };
    };
    if !remove_list.is_empty() 
    {
        for index in remove_list
        {
            all_config_file_data_as_string_vectors.remove(index);
        }
    }

    ConfigFileData
    {
        path_to_scan:  all_config_file_data_as_string_vectors[0].clone(),
        window_size: u32_vector_config_file_data,
        use_gamemode: bool_vector_config_file_data[0],
        use_gamescope: bool_vector_config_file_data[1],
        gamescope_flags:  all_config_file_data_as_string_vectors[4][0].clone(),
        object_per_line: i32_number_config_file_data,
        text_position: vec![i32_vector_config_file_data[0], i32_vector_config_file_data[1]],
        image_position: vec![i32_vector_config_file_data[2], i32_vector_config_file_data[3]],
        distance_between_texts: vec![i32_vector_config_file_data[4], i32_vector_config_file_data[5]],
        distance_between_images: vec![i32_vector_config_file_data[6], i32_vector_config_file_data[7]],
        background_color: vec![u8_vector_config_file_data[0], u8_vector_config_file_data[1], u8_vector_config_file_data[2]],
        foreground_color: vec![u8_vector_config_file_data[3], u8_vector_config_file_data[4], u8_vector_config_file_data[5]],
    }
}
