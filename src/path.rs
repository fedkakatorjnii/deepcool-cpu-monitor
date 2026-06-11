use anyhow::Context;
use std::env;
use users::os::unix::UserExt;

/// Расширяет путь, корректно обрабатывая `~` при запуске через sudo
pub fn expand_sudo_aware_path(path: &str) -> anyhow::Result<String> {
    // Проверяем, начинается ли путь с ~/ или равен ~
    if path.starts_with("~/") || path == "~" {
        // Проверяем, есть ли переменная SUDO_USER (признак запуска через sudo)
        if let Ok(sudo_user) = env::var("SUDO_USER") {
            // Пытаемся найти пользователя в системе по имени
            if let Some(user) = users::get_user_by_name(&sudo_user) {
                // Получаем его домашнюю директорию
                let tmp = user.home_dir();
                let home_dir = tmp.to_str().context("Bab config file path")?;

                return Ok(path.replacen("~", home_dir, 1));
            }
        }
    }

    // Если не под sudo, или пользователь не найден, используем стандартное поведение
    Ok(shellexpand::tilde(path).to_string())
}
