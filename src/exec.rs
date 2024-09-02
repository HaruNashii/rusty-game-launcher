use std::process::Command;

pub fn exec_app(app_arg: u8, gamescope_arg: &String, app_path: &String)
{
    let command_to_exec = match app_arg
    {
        1 => format!("gamemoderun {}", app_path),
        2 => format!("gamescope {} {}", gamescope_arg, app_path),
        3 => format!("gamemoderun gamescope {} {}", gamescope_arg, app_path),
        _=> app_path.to_string(),
    };

    let mut parts = command_to_exec.split_whitespace().collect::<Vec<&str>>();
    Command::new(parts.remove(0)).args(parts).spawn().unwrap();
}
