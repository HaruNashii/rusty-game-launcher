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
            match name.to_string().find("steam.desktop")
            {
                Some(..) => continue,
                None => {},
            };

            let file = File::open( format!("{}/{}", path_to_read, name) ).unwrap();
            let file_content = BufReader::new(&file);

            for line in file_content.lines() 
            {
                let line_content = line.unwrap();
                match line_content.find("Game;")
                {
                    Some(..) => files_with_game_category_name.push(name),
                    None => {},
                }
            }
        }

        // check if the file has the exec argument
        // if YES return the exec argument line and the file name
        let mut file_exec = String::new();
        let mut file_name = String::new();
        let mut file_image = String::new();
        for name in files_with_game_category_name
        {
            let file = File::open( format!("{}/{}", path_to_read, name) ).unwrap();
            let file_content = BufReader::new(file);

            for line in file_content.lines() 
            {
                file_name = name.to_string().replace(".desktop", "");
                let line_content = line.unwrap();
                match line_content.find("Exec=")
                {
                    Some(..) => 
                    {
                        file_exec = line_content.replace("Exec=", "");
                    }
                    None => {},
                }

                match line_content.find("Icon=")
                {
                    Some(..) => 
                    {
                        file_image = line_content.replace("Icon=", "");

                        let desktop_file_image_check = vec!
                        [
                            // HARD CODED BECAUSE I'M LAZY, I WILL CHANGE IT LATER
                            format!("/var/lib/flatpak/exports/share/icons/hicolor/16x16/apps/{}.png", file_image),
                            format!("/var/lib/flatpak/exports/share/icons/hicolor/24x24/apps/{}.png", file_image),
                            format!("/var/lib/flatpak/exports/share/icons/hicolor/32x32/apps/{}.png", file_image),
                            format!("/var/lib/flatpak/exports/share/icons/hicolor/48x48/apps/{}.png", file_image),
                            format!("/var/lib/flatpak/exports/share/icons/hicolor/64x64/apps/{}.png", file_image),
                            format!("/var/lib/flatpak/exports/share/icons/hicolor/128x128/apps/{}.png", file_image),
                            format!("/var/lib/flatpak/exports/share/icons/hicolor/256x256/apps/{}.png", file_image),
                            format!("/var/lib/flatpak/exports/share/icons/hicolor/512x512/apps/{}.png", file_image),

                            format!("/home/{}/.local/share/flatpak/exports/share/icons/16x16/apps/{}.png", user_name, file_image),
                            format!("/home/{}/.local/share/flatpak/exports/share/icons/24x24/apps/{}.png", user_name, file_image),
                            format!("/home/{}/.local/share/flatpak/exports/share/icons/32x32/apps/{}.png", user_name, file_image),
                            format!("/home/{}/.local/share/flatpak/exports/share/icons/48x48/apps/{}.png", user_name, file_image),
                            format!("/home/{}/.local/share/flatpak/exports/share/icons/64x64/apps/{}.png", user_name, file_image),
                            format!("/home/{}/.local/share/flatpak/exports/share/icons/128x128/apps/{}.png", user_name, file_image),
                            format!("/home/{}/.local/share/flatpak/exports/share/icons/256x256/apps/{}.png", user_name, file_image),
                            format!("/home/{}/.local/share/flatpak/exports/share/icons/512x512/apps/{}.png", user_name, file_image),

                            format!("/home/{}/.local/share/icons/hicolor/16x16/apps/{}.png", user_name, file_image),
                            format!("/home/{}/.local/share/icons/hicolor/24x24/apps/{}.png", user_name, file_image),
                            format!("/home/{}/.local/share/icons/hicolor/32x32/apps/{}.png", user_name, file_image),
                            format!("/home/{}/.local/share/icons/hicolor/48x48/apps/{}.png", user_name, file_image),
                            format!("/home/{}/.local/share/icons/hicolor/64x64/apps/{}.png", user_name, file_image),
                            format!("/home/{}/.local/share/icons/hicolor/128x128/apps/{}.png", user_name, file_image),
                            format!("/home/{}/.local/share/icons/hicolor/256x256/apps/{}.png", user_name, file_image),
                        ];
                        for path in &desktop_file_image_check
                        {
                            if Path::new(&path).exists()
                            {
                                file_image = path.to_string();
                            };
                        }
                    }
                    None => {},
                }
            }
            files.push( DesktopFile{desktop_file_name: file_name.replace('"', ""), desktop_file_exec: file_exec.replace('"', ""), desktop_file_image: file_image.replace('"', "")} );
        }
    }

    files
}
