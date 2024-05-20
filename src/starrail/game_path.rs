use anyhow::Result;
use ini::Ini;
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

pub fn get_starrail_install_path() -> Result<String> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let hkpath_reg = hklm.open_subkey("Software\\Classes\\hkrpg-cn\\shell\\open\\command")?;

    let mut hkpath: String = hkpath_reg.get_value("")?;

    let mut hkpath_vec: Vec<&str> = hkpath.split('\"').collect();

    hkpath_vec = hkpath_vec
        .iter()
        .map(|s| s.to_owned())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();

    if hkpath_vec.len() > 0 {
        hkpath = hkpath_vec[0].to_owned();
    }

    // 去除路径前后的引号
    hkpath = hkpath.trim_matches(|c| c == '\"' || c == '\'').to_string();

    hkpath = {
        let mut a: Vec<&str> = hkpath.split('\\').collect();
        a.pop();
        a.join("\\")
    };

    Ok(hkpath)
}

pub fn get_starrail_game_path(config_path: String) -> Result<String> {
    let conf = Ini::load_from_file(config_path)?;

    if let Some(section) = conf.section(Some("launcher")) {
        let game_install_path = section.get("game_install_path").unwrap_or_default();
        return Ok(game_install_path.to_owned());
    }

    Ok("".to_owned())
}

#[test]
fn test_get_path() {
    match get_starrail_install_path() {
        Ok(path) => println!("path: {}", &path),
        Err(e) => println!("error: {e:?}"),
    }
}
