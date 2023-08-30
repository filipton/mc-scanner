use colored::{ColoredString, Colorize};

use crate::info_getter::Extra;

pub fn parse_extra(extra: &Vec<Extra>) -> String {
    let mut tmp = String::new();
    for extra in extra {
        let mut colored_text = get_colored_bit(
            &extra.clone().color.unwrap_or(String::from("white")),
            extra.bold.unwrap_or(false),
            extra.italic.unwrap_or(false),
            extra.strikethrough.unwrap_or(false),
            extra.underline.unwrap_or(false),
            &extra.text,
        );

        if extra.bold.unwrap_or(false) {
            colored_text = colored_text.bold();
        }

        tmp += format!("{}", colored_text).as_str();
    }
    return tmp;
}

fn get_colored_bit(
    color_name: &str,
    bold: bool,
    italic: bool,
    strike_through: bool,
    underline: bool,
    text: &str,
) -> ColoredString {
    let color = get_color_by_name(color_name);
    let mut colored_text = text.truecolor(color.0, color.1, color.2);

    if bold {
        colored_text = colored_text.bold();
    }
    if italic {
        colored_text = colored_text.italic();
    }
    if strike_through {
        colored_text = colored_text.strikethrough();
    }
    if underline {
        colored_text = colored_text.underline();
    }

    colored_text
}

fn get_color_by_name(color_name: &str) -> (u8, u8, u8) {
    match color_name {
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
        _ => (170, 170, 170),
    }
}

pub fn parse_old_desc_to_extra(text: &str) -> Vec<Extra> {
    let mut extra = Vec::new();

    let mut tmp = String::new();
    let mut color = String::from("gray");
    let mut obfuscated = false;
    let mut bold = false;
    let mut italic = false;
    let mut underline = false;
    let mut strikethrough = false;

    let chars = text.chars().collect::<Vec<char>>();

    let mut i = 0;
    loop {
        let c = if let Some(c) = chars.get(i) {
            *c
        } else {
            break;
        };

        if c == '§' {
            i += 1;
            let c = chars.get(i).unwrap_or(&'\0');
            let paragraph = parse_paragraph(*c);

            extra.push(Extra {
                color: Some(color.clone()),
                obfuscated: Some(obfuscated),
                bold: Some(bold),
                italic: Some(italic),
                underline: Some(underline),
                strikethrough: Some(strikethrough),
                text: tmp.clone(),
            });
            tmp.clear();

            match paragraph {
                Paragraph::Color(color_name) => {
                    color = color_name;
                    obfuscated = false;
                    bold = false;
                    italic = false;
                    underline = false;
                    strikethrough = false;
                },
                Paragraph::Obfuscated(val) => obfuscated = val,
                Paragraph::Bold(val) => bold = val,
                Paragraph::Italic(val) => italic = val,
                Paragraph::Underline(val) => underline = val,
                Paragraph::Strikethrough(val) => strikethrough = val,
                Paragraph::Reset => {
                    color = String::from("gray");
                    obfuscated = false;
                    bold = false;
                    italic = false;
                    underline = false;
                    strikethrough = false;
                }
                Paragraph::None => {}
            }
        } else {
            tmp.push(c);
        }

        i += 1;
    }

    if !tmp.is_empty() {
        extra.push(Extra {
            color: Some(color),
            obfuscated: Some(obfuscated),
            bold: Some(bold),
            italic: Some(italic),
            underline: Some(underline),
            strikethrough: Some(strikethrough),
            text: tmp,
        });
    }

    extra
}

fn parse_paragraph(paragraph: char) -> Paragraph {
    match paragraph {
        '0' => Paragraph::Color(String::from("black")),
        '1' => Paragraph::Color(String::from("dark_blue")),
        '2' => Paragraph::Color(String::from("dark_green")),
        '3' => Paragraph::Color(String::from("dark_aqua")),
        '4' => Paragraph::Color(String::from("dark_red")),
        '5' => Paragraph::Color(String::from("dark_purple")),
        '6' => Paragraph::Color(String::from("gold")),
        '7' => Paragraph::Color(String::from("gray")),
        '8' => Paragraph::Color(String::from("dark_gray")),
        '9' => Paragraph::Color(String::from("blue")),
        'a' => Paragraph::Color(String::from("green")),
        'b' => Paragraph::Color(String::from("aqua")),
        'c' => Paragraph::Color(String::from("red")),
        'd' => Paragraph::Color(String::from("light_purple")),
        'e' => Paragraph::Color(String::from("yellow")),
        'f' => Paragraph::Color(String::from("white")),
        'k' => Paragraph::Obfuscated(true),
        'l' => Paragraph::Bold(true),
        'm' => Paragraph::Strikethrough(true),
        'n' => Paragraph::Underline(true),
        'o' => Paragraph::Italic(true),
        'r' => Paragraph::Reset,
        _ => Paragraph::None,
    }
}

enum Paragraph {
    Color(String),
    Obfuscated(bool),
    Bold(bool),
    Italic(bool),
    Underline(bool),
    Strikethrough(bool),
    Reset,
    None,
}
