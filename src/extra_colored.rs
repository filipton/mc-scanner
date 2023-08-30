use colored::{ColoredString, Colorize};

pub fn get_colored_bit(color_name: &str, bold: bool, text: &str) -> ColoredString {
    let mut colored_text = get_color_by_name(color_name, text);
    if bold {
        colored_text = colored_text.bold();
    }

    colored_text
}

fn get_color_by_name(color_name: &str, text: &str) -> ColoredString {
    let color = match color_name {
        "black" => (0, 0, 0),
        "dark_blue" => (0, 0, 170),
        "dark_green" => (0, 170, 0),
        "dark_aqua" => (0, 170, 170),
        "dark_red" => (170, 0, 0),
        "dark_purple" => (170, 0, 170),
        "gold" => (255, 170, 0),
        "gray" => (170, 170, 170),
        "dark_gray" => (85, 85, 85),
        "blue" => (85, 85, 255),
        "green" => (85, 255, 85),
        "aqua" => (85, 255, 255),
        "red" => (255, 85, 85),
        "light_purple" => (255, 85, 255),
        "yellow" => (255, 255, 85),
        "white" => (255, 255, 255),
        _ => (255, 255, 255),
    };

    text.truecolor(color.0, color.1, color.2)
}
