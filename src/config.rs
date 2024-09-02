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
}

pub fn read_config_file() -> ConfigFileData
{
    //get user name because the username is also the name of the home directory
    //the env::home_dir() from the rust standart library is deprecated, thats why i'm using this
    //hacky method
    let command = "whoami";
    let mut parts = command.split_whitespace().collect::<Vec<&str>>();
    let stdout = Command::new(parts.remove(0)).args(parts).output().unwrap_or_else(|_| panic!("Failed to execute command '{}'", command)).stdout;
    let user_name = String::from_utf8(stdout).expect("Stdout was not valid UTF-8").replace("\n", "");


    let mut all_config_file_data = Vec::new();
    let mut converted_to_u32_config_file_data = Vec::new();
    let mut converted_to_bool_config_file_data = Vec::new();
    let options = vec!
    [
        "path_to_scan:",
        "window_size:",
        "use_gamemode:",
        "use_gamescope:",
        "gamescope_flags:",
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
    ];


    // verify if the config file exist and if it don't 
    // create the file with a holder config
    let config_path = format!("/home/{}/.config/rusty-game-launcher", user_name);
    let config_file_name = "config.i_will_kms";
    let full_path_of_config_file = format!("{}/{}", config_path, config_file_name);
    if !Path::new(&config_path).exists()
    {
        fs::create_dir_all(&config_path).unwrap();
    };
    if !Path::new(&full_path_of_config_file).exists()
    {
        let mut config_file = File::create(&full_path_of_config_file).unwrap();
        for values in default_values
        {
            write!(config_file, "\n {}", values).unwrap();
        };
    };



    // append the text from the file to one string
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
                    if holder.is_empty() 
                    {
                        holder.push(" ".to_string());
                    }
                    all_config_file_data.push(holder);
                },
                None => continue,
            }
        }
    }

    
    // convert strings to u32 to suit the struct field
    for string_to_parse in &all_config_file_data[1]
    {
        let converted_number: u32 = string_to_parse.parse().unwrap();
        converted_to_u32_config_file_data.push(converted_number);
    };


    // convert strings to bool to suit the struct field
    for index in 2..4
    {
        let string_to_convert = &all_config_file_data[index][0];
        let mut converted_bool = false;
        if string_to_convert == "false" {converted_bool = false};
        if string_to_convert == "true" {converted_bool = true};
        converted_to_bool_config_file_data.push(converted_bool);
    };


    // verify if the directory informed in the config file exists 
    // if don't stop the app and print one error message
    for path_to_scan_from_user in &all_config_file_data[0]
    {
        if !Path::new(&path_to_scan_from_user).exists()
        {
            panic!("\n THE PATH DESCRIBE 'path_to_scan' IN THE CONFIG FILE {} \n DOESN'T EXIST!! \n", &full_path_of_config_file);
        };
    };



    ConfigFileData
    {
        path_to_scan:  all_config_file_data[0].clone(),
        window_size: converted_to_u32_config_file_data,
        use_gamemode: converted_to_bool_config_file_data[0],
        use_gamescope: converted_to_bool_config_file_data[1],
        gamescope_flags:  all_config_file_data[4][0].clone(),
    }
}
