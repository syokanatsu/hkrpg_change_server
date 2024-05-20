use std::fs;

use anyhow::Result;
use ini::Ini;

const SDK_PKG_VERSION: &str = include_str!("../../assets/Game/sdk_pkg_version");
const FAILEDLOG_DB: &[u8] = include_bytes!("../../assets/Game/StarRail_Data/Plugins/failedlog.db");
const LICENSE_TXT: &str = include_str!("../../assets/Game/StarRail_Data/Plugins/license.txt");
const PCGAMESDK_DLL: &[u8] =
    include_bytes!("../../assets/Game/StarRail_Data/Plugins/PCGameSDK.dll");

#[derive(Debug, PartialEq)]
pub enum Channel {
    MIHOYO,
    BILIBILI,
}

fn get_channel<S>(config_path: String, section: Option<S>) -> Result<Option<Channel>>
where
    S: Into<String>,
{
    let conf = Ini::load_from_file(config_path)?;

    let mut channel = None;

    if let Some(section) = conf.section(section) {
        let cps = section.get("cps").unwrap_or("");
        let c = section.get("channel").unwrap_or("");
        let sc = section.get("sub_channel").unwrap_or_default();

        if cps == "bilibili_PC" && c == "14" && sc == "0" {
            channel = Some(Channel::BILIBILI);
        } else if cps == "gw_PC" && c == "1" && sc == "1" {
            channel = Some(Channel::MIHOYO);
        }
    }

    Ok(channel)
}

pub fn get_launcher_channel(config_path: String) -> Result<Option<Channel>> {
    get_channel(config_path, Some("launcher"))
}

pub fn get_game_channel(config_path: String) -> Result<Option<Channel>> {
    get_channel(config_path, Some("General"))
}

fn set_channel<S>(config_path: String, section: Option<S>, channel: Channel) -> Result<()>
where
    S: Into<String>,
{
    let cur_channel = get_launcher_channel(config_path.clone())?;
    if cur_channel.is_some() && cur_channel.unwrap() == channel {
        return Ok(());
    }

    let mut conf = Ini::load_from_file(&config_path)?;

    let cps;
    let c;
    let sc;

    match channel {
        Channel::MIHOYO => {
            cps = "gw_PC";
            c = "1";
            sc = "1"
        }
        Channel::BILIBILI => {
            cps = "bilibili_PC";
            c = "14";
            sc = "0"
        }
    }

    conf.with_section(section)
        .set("cps", cps)
        .set("channel", c)
        .set("sub_channel", sc);
    conf.write_to_file(config_path)?;
    Ok(())
}

pub fn set_launcher_channel(config_path: String, channel: Channel) -> Result<()> {
    set_channel(config_path, Some("launcher"), channel)
}

pub fn set_game_channel(config_path: String, channel: Channel) -> Result<()> {
    set_channel(config_path, Some("General"), channel)
}

pub fn install_bilibili_sdk(install_path: String) -> Result<()> {
    fs::write(install_path.clone() + "\\sdk_pkg_version", SDK_PKG_VERSION)?;
    fs::write(
        install_path.clone() + "\\StarRail_Data\\Plugins\\failedlog.db",
        FAILEDLOG_DB,
    )?;
    fs::write(
        install_path.clone() + "\\StarRail_Data\\Plugins\\license.txt",
        LICENSE_TXT,
    )?;
    fs::write(
        install_path.clone() + "\\StarRail_Data\\Plugins\\PCGameSDK.dll",
        PCGAMESDK_DLL,
    )?;

    Ok(())
}

pub fn remove_bilibili_sdk(install_path: String) -> Result<()> {
    fs::remove_file(install_path.clone() + "\\sdk_pkg_version")?;
    fs::remove_file(install_path.clone() + "\\StarRail_Data\\Plugins\\failedlog.db")?;
    fs::remove_file(install_path.clone() + "\\StarRail_Data\\Plugins\\license.txt")?;
    fs::remove_file(install_path.clone() + "\\StarRail_Data\\Plugins\\PCGameSDK.dll")?;
    Ok(())
}

#[test]
fn test_channel() {
    use super::game_path::get_starrail_install_path;
    let channel =
        get_launcher_channel(get_starrail_install_path().unwrap() + "\\config.ini").unwrap();
    println!("channel: {:?}", channel);
}

#[test]
fn test_set_channel() {
    use super::game_path::get_starrail_install_path;
    set_launcher_channel(
        get_starrail_install_path().unwrap() + "\\config.ini",
        Channel::MIHOYO,
    )
    .unwrap();

    let channel =
        get_launcher_channel(get_starrail_install_path().unwrap() + "\\config.ini").unwrap();

    assert_eq!(channel, Some(Channel::MIHOYO));

    set_launcher_channel(
        get_starrail_install_path().unwrap() + "\\config.ini",
        Channel::BILIBILI,
    )
    .unwrap();

    let channel =
        get_launcher_channel(get_starrail_install_path().unwrap() + "\\config.ini").unwrap();

    assert_eq!(channel, Some(Channel::BILIBILI));
}

#[test]
fn test_resource() {
    println!("{}", SDK_PKG_VERSION);
}
