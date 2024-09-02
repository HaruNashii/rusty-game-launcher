use std::fs::File;
use std::fs;
use std::io::{BufReader, BufRead, Write};
use std::path::Path;
use std::process::Command;

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

pub fn read_config_file() -> ConfigFileData
{
    let user_name = get_user_name();

    let mut all_config_file_data_as_string_vectors = Vec::new();
    let mut converted_to_i32_config_file_data = Vec::new();
    let mut converted_to_u32_config_file_data = Vec::new();
    let mut converted_to_bool_config_file_data = Vec::new();
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
    ];
    //define one default value in case the config file 
    // doesn't have one setted up
    let default_values = vec!
    [
        format!("path_to_scan:/usr/share/applications /home/{}/.local/share/applications", user_name),
        "window_size:800 600".to_string(),
        "use_gamemode:false".to_string(),
        "use_gamescope:true".to_string(),
        "gamescope_flags:--fullscreen".to_string(),
        "object_per_line:3".to_string(),
        "text_position:28 215".to_string(),
        "image_position:25 115".to_string(),
        "distance_between_texts:250 250".to_string(),
        "distance_between_images:250 250".to_string(),
    ];


    // verify if the config file exist and if it don't 
    // create the file with a holder config
    let config_path = format!("/home/{}/.config/rusty-game-launcher", user_name);
    let config_file_name = "config.i_will_kms";
    let full_path_of_config_file = format!("{}/{}", config_path, config_file_name);
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

    // convert strings to i32 to suit the struct field
    let converted_to_i32_number_config_file_data: i32 = all_config_file_data_as_string_vectors[5][0].parse().unwrap();

    // convert strings to a vector of i32 to suit the struct field
    for string_to_parse in all_config_file_data_as_string_vectors.iter().skip(5)
    {
        for string in string_to_parse
        {
            let converted_number: i32 = string.parse().unwrap();
            converted_to_i32_config_file_data.push(converted_number);
        }
    };
    

    // convert strings to u32 to suit the struct field
    for string_to_parse in &all_config_file_data_as_string_vectors[1]
    {
        let converted_number: u32 = string_to_parse.parse().unwrap();
        converted_to_u32_config_file_data.push(converted_number);
    };


    // convert strings to bool to suit the struct field
    for string_to_convert in all_config_file_data_as_string_vectors.iter().take(4).skip(2)
    {
        let mut converted_bool = false;
        if string_to_convert[0] == "false" {converted_bool = false};
        if string_to_convert[0] == "true" {converted_bool = true};
        converted_to_bool_config_file_data.push(converted_bool);
    };


    // verify if the directory informed in the config file exists 
    // if don't stop the app and print one error message
    for path_to_scan_from_user in &all_config_file_data_as_string_vectors[0]
    {
        if !Path::new(&path_to_scan_from_user).exists()
        {
            panic!("\n THE PATH DESCRIBE 'path_to_scan' IN THE CONFIG FILE {} \n DOESN'T EXIST!! \n", &full_path_of_config_file);
        };
    };


    ConfigFileData
    {
        path_to_scan:  all_config_file_data_as_string_vectors[0].clone(),
        window_size: converted_to_u32_config_file_data,
        use_gamemode: converted_to_bool_config_file_data[0],
        use_gamescope: converted_to_bool_config_file_data[1],
        gamescope_flags:  all_config_file_data_as_string_vectors[4][0].clone(),
        object_per_line: converted_to_i32_number_config_file_data,
        text_position: vec![converted_to_i32_config_file_data[0], converted_to_i32_config_file_data[1]],
        image_position: vec![converted_to_i32_config_file_data[2], converted_to_i32_config_file_data[3]],
        distance_between_texts: vec![converted_to_i32_config_file_data[4], converted_to_i32_config_file_data[5]],
        distance_between_images: vec![converted_to_i32_config_file_data[6], converted_to_i32_config_file_data[7]],
    }
}
