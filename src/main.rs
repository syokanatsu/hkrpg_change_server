use crate::starrail::{
    game_config::{
        get_game_channel, get_launcher_channel, install_bilibili_sdk, remove_bilibili_sdk,
        set_game_channel, set_launcher_channel,
    },
    game_path::{get_starrail_game_path, get_starrail_install_path},
};
use anyhow::Result;
use std::{
    process::{self, exit},
    ptr::null_mut,
};
use widestring::WideCString;

use winapi::um::{shellapi::ShellExecuteW, winuser::SW_SHOWNORMAL};
mod starrail;

fn main() {
    loop {
        match print_cur_info() {
            Ok((install_path, game_path)) => {
                println!("输入1、2切换官服(1)和B服(2), s启动崩铁, q退出");
                let mut line = String::new();
                let _ = std::io::stdin().read_line(&mut line);

                line = line.trim().to_owned();

                if line.starts_with('1') {
                    if let Err(e) = (|| {
                        set_launcher_channel(
                            install_path.clone() + "\\config.ini",
                            starrail::game_config::Channel::MIHOYO,
                        )?;

                        set_game_channel(
                            game_path.clone() + "\\config.ini",
                            starrail::game_config::Channel::MIHOYO,
                        )?;

                        remove_bilibili_sdk(game_path)
                    })() {
                        println!("Error: {e:?}");
                        break;
                    }
                } else if line.starts_with('2') {
                    if let Err(e) = (|| {
                        set_launcher_channel(
                            install_path.clone() + "\\config.ini",
                            starrail::game_config::Channel::BILIBILI,
                        )?;

                        set_game_channel(
                            game_path.clone() + "\\config.ini",
                            starrail::game_config::Channel::BILIBILI,
                        )?;

                        install_bilibili_sdk(game_path)
                    })() {
                        println!("Error: {e:?}");
                        break;
                    }
                } else if line.starts_with('q') || line.starts_with('Q') {
                    exit(0);
                } else if line.starts_with('s') || line.starts_with('S') {
                    match execute_hkrpg(install_path.clone()) {
                        Ok(result) => {
                            if result {
                                exit(0);
                            } else {
                                println!("启动失败");
                                break;
                            }
                        }
                        Err(e) => {
                            println!("Error: {e:?}");
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {e:?}");
                break;
            }
        }
    }

    let _ = process::Command::new("cmd").arg("/c").arg("pause").status();
}

fn print_cur_info() -> Result<(String, String)> {
    let install_path = get_starrail_install_path()?;
    let game_path = get_starrail_game_path(install_path.clone() + "\\config.ini")?;
    println!("-------------------------------------------------------------");
    println!("当前启动器安装目录: {}", &install_path);
    println!("当前游戏安装目录: {}", &game_path);
    println!(
        "当前Launcher Channel: {:?}",
        get_launcher_channel(install_path.clone() + "\\config.ini")?
    );
    println!(
        "当前Game Channel: {:?}",
        get_game_channel(game_path.clone() + "\\config.ini")?
    );
    println!("-------------------------------------------------------------");
    Ok((install_path, game_path))
}

fn execute_hkrpg(install_path: String) -> Result<bool> {
    let wide_hkrpg_path = WideCString::from_str(install_path + "\\launcher.exe")?;
    let verb = WideCString::from_str("runas")?;
    unsafe {
        let result = ShellExecuteW(
            null_mut(),
            verb.as_ptr(),
            wide_hkrpg_path.as_ptr(),
            null_mut(),
            null_mut(),
            SW_SHOWNORMAL,
        );
        if result as i32 <= 32 {
            return Ok(false);
        }
    }
    Ok(true)
}
