use std::fs::File;
use std::fs;
use std::io::{BufReader, BufRead};
use std::path::Path;

pub struct DesktopFile
{
    pub desktop_file_name: String,
    pub desktop_file_exec: String,
}

pub fn read_desktop_files(received_path: Vec<String>) -> Vec<DesktopFile>
{
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
        for name in files_with_game_category_name
        {
            let file = File::open( format!("{}/{}", path_to_read, name) ).unwrap();
            let file_content = BufReader::new(file);

            for line in file_content.lines() 
            {
                let line_content = line.unwrap();
                match line_content.find("Exec=")
                {
                    Some(..) => files.push( DesktopFile{desktop_file_name: name.to_string().replace(".desktop", ""), desktop_file_exec: line_content.replace("Exec=", "")} ),
                    None => {},
                }
            }
        }

    }

    files
}
