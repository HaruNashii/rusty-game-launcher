use std::fs::File;
use std::fs;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub struct DesktopFile
{
    pub desktop_file_name: String,
    pub desktop_file_exec: String,
    pub desktop_file_image: String,
}

pub fn read_desktop_files(received_path: Vec<String>) -> Vec<DesktopFile>
{
    //get user name because the username is also the name of the home directory
    //the env::home_dir() from the rust standart library is deprecated, thats why i'm using this
    //hacky method
    let command = "whoami";
    let mut parts = command.split_whitespace().collect::<Vec<&str>>();
    let stdout = Command::new(parts.remove(0)).args(parts).output().unwrap_or_else(|_| panic!("Failed to execute command '{}'", command)).stdout;
    let user_name = String::from_utf8(stdout).expect("Stdout was not valid UTF-8").replace("\n", "");


    let mut files = Vec::new();
    for path_to_read in &received_path
    {
        if !Path::new(path_to_read).exists()
        {
            continue;
        };


        let path_entries = fs::read_dir(path_to_read).unwrap();
        let file_names: Vec<String> = path_entries.filter_map(|entry| { let path = entry.ok().unwrap().path(); if path.is_file() { path.file_name().unwrap().to_str().map(|s| s.to_owned()) } else { None } }) .collect();
        let mut files_with_game_category_name = Vec::new();


        // check if the file has the Game category
        for name in &file_names
        {
            if name.to_string().contains("steam.desktop") { continue };
            
            let file = File::open( format!("{}/{}", path_to_read, name) ).unwrap();
            let file_content = BufReader::new(&file);

            for line in file_content.lines() 
            {
                let line_content = line.unwrap();
                if line_content.contains("Game") && line_content.contains("Categories=") && !line_content.contains("Discord") { files_with_game_category_name.push(name) }
            }
        }

        // check if the file has the exec argument
        // if YES return the exec argument line and the file name
        let mut file_exec = String::new();
        let mut file_image = String::new();
        for name in files_with_game_category_name
        {
            let file = File::open( format!("{}/{}", path_to_read, name) ).unwrap();
            let file_content = BufReader::new(file);

            for line in file_content.lines() 
            {
                let line_content = line.unwrap();
                if line_content.contains("Exec=") { file_exec = line_content.replace("Exec=", ""); }
                if line_content.contains("Icon=")
                {
                    file_image = line_content.replace("Icon=", "");

                    let size_of_image = vec!["512x512", "256x256", "128x128", "64x64", "48x48", "32x32", "24x24", "16x16"];
                    'size_for_loop: for size in size_of_image
                    {
                        let desktop_file_image_check = vec!
                        [
                            format!("/usr/share/pixmaps/{}.png", file_image),
                            format!("/var/lib/flatpak/exports/share/icons/hicolor/{}/apps/{}.png", size, file_image),
                            format!("/home/{}/.local/share/flatpak/exports/share/icons/{}/apps/{}.png", user_name, size, file_image),
                            format!("/home/{}/.local/share/icons/hicolor/{}/apps/{}.png", user_name, size, file_image),
                        ];

                        for path in &desktop_file_image_check
                        {
                            if Path::new(path).exists()
                            {
                                file_image = path.to_string();
                                break 'size_for_loop;
                            };
                        }
                    };
                }
            }
            files.push( DesktopFile{desktop_file_name: name.to_string().replace(".desktop", "").replace('"', ""), desktop_file_exec: file_exec.replace('"', ""), desktop_file_image: file_image.replace('"', "")} );
        }
    }

    files
}
