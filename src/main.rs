use crate::read::read_desktop_files;

mod read;

fn main() 
{
    let files = read_desktop_files(vec!["/home/haru/.local/share/applications", "/usr/share/applications"]);
    for file in files 
    {
        println!("\nfile name    = {}", file.desktop_file_name);
        println!("file content = {}\n", file.desktop_file_exec);
    }
}
