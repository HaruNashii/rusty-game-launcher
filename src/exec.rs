use std::process::Command;

pub fn exec_app(app_arg: u8, gamescope_arg: &String, app_path: &String)
{
    let mut command_to_exec = String::new();
    match app_arg
    {
        1 => command_to_exec = format!("gamemoderun {}", app_path),
        2 => command_to_exec = format!("gamescope {} {}", gamescope_arg, app_path),
        3 => command_to_exec = format!("gamemoderun gamescope {} {}", gamescope_arg, app_path),
        _=> command_to_exec = app_path.to_string(),
    }

    let mut parts = command_to_exec.split_whitespace().collect::<Vec<&str>>();
    Command::new(parts.remove(0)).args(parts).spawn().unwrap();
}
