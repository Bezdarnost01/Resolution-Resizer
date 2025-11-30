use windows::Win32::Graphics::Gdi::{
    EnumDisplaySettingsW, ChangeDisplaySettingsW,
    DEVMODEW, ENUM_CURRENT_SETTINGS,
    DM_PELSWIDTH, DM_PELSHEIGHT,
    CDS_UPDATEREGISTRY, DISP_CHANGE_SUCCESSFUL,
};

fn get_current_resolution() -> Result<(u32, u32), String> {
    unsafe {
        let mut devmode = DEVMODEW::default();
        devmode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;

        if !EnumDisplaySettingsW(None, ENUM_CURRENT_SETTINGS, &mut devmode).as_bool() {
            return Err("EnumDisplaySettingsW failed".into());
        }

        Ok((devmode.dmPelsWidth, devmode.dmPelsHeight))
    }
}

fn set_resolution(width: u32, height: u32) -> Result<(), String> {
    unsafe {
        let mut devmode = DEVMODEW::default();
        devmode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;

        if !EnumDisplaySettingsW(None, ENUM_CURRENT_SETTINGS, &mut devmode).as_bool() {
            return Err("EnumDisplaySettingsW failed".into());
        }

        devmode.dmPelsWidth = width;
        devmode.dmPelsHeight = height;

        devmode.dmFields = DM_PELSWIDTH | DM_PELSHEIGHT;

        let result = ChangeDisplaySettingsW(Some(&devmode), CDS_UPDATEREGISTRY);

        if result != DISP_CHANGE_SUCCESSFUL {
            return Err(format!("ChangeDisplaySettingsW failed: {:?}", result));
        }

        Ok(())
    }
}

fn main() {
    let (w, h) = match get_current_resolution() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Не удалось получить текущее разрешение: {e}");
            return;
        }
    };

    println!("Текущее разрешение: {}x{}", w, h);

    let (new_w, new_h) = if (w, h) == (1920, 1080) {
        (1440, 1080)
    } else {
        (1920, 1080)
    };

    if let Err(e) = set_resolution(new_w, new_h) {
        eprintln!("Ошибка при смене разрешения: {e}");
    } else {
        println!("Новое разрешение: {}x{}", new_w, new_h);
    }
}

