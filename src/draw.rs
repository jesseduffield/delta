use std::io::Write;

use box_drawing;
use console::strip_ansi_codes;
use syntect::highlighting::Color;
use unicode_segmentation::UnicodeSegmentation;

use crate::paint;

/// Write text to stream, surrounded by a box, leaving the cursor just
/// beyond the bottom right corner.
pub fn write_boxed(
    writer: &mut dyn Write,
    text: &str,
    _line_width: usize, // ignored
    color: Color,
    heavy: bool,
) -> std::io::Result<()> {
    let up_left = if heavy {
        box_drawing::heavy::UP_LEFT
    } else {
        box_drawing::light::UP_LEFT
    };
    let box_width = strip_ansi_codes(text).graphemes(true).count() + 1;
    write_boxed_partial(writer, text, box_width, color, heavy)?;
    write!(writer, "{}", paint::paint_text_foreground(up_left, color))?;
    Ok(())
}

/// Write text to stream, surrounded by a box, and extend a line from
/// the bottom right corner.
pub fn write_boxed_with_line(
    writer: &mut dyn Write,
    text: &str,
    line_width: usize,
    color: Color,
    heavy: bool,
) -> std::io::Result<()> {
    let box_width = strip_ansi_codes(text).graphemes(true).count() + 1;
    write_boxed_with_horizontal_whisker(writer, text, box_width, color, heavy)?;
    write_horizontal_line(
        writer,
        if line_width > box_width {
            line_width - box_width - 1
        } else {
            0
        },
        color,
        heavy,
    )?;
    write!(writer, "\n")?;
    Ok(())
}

pub fn write_underlined(
    writer: &mut dyn Write,
    text: &str,
    line_width: usize,
    color: Color,
    heavy: bool,
) -> std::io::Result<()> {
    writeln!(writer, "{}", paint::paint_text_foreground(text, color))?;
    write_horizontal_line(writer, line_width - 1, color, heavy)?;
    write!(writer, "\n")?;
    Ok(())
}

fn write_horizontal_line(
    writer: &mut dyn Write,
    line_width: usize,
    color: Color,
    heavy: bool,
) -> std::io::Result<()> {
    let horizontal = if heavy {
        box_drawing::heavy::HORIZONTAL
    } else {
        box_drawing::light::HORIZONTAL
    };
    write!(
        writer,
        "{}",
        paint::paint_text_foreground(&horizontal.repeat(line_width), color)
    )
}

pub fn write_boxed_with_horizontal_whisker(
    writer: &mut dyn Write,
    text: &str,
    box_width: usize,
    color: Color,
    heavy: bool,
) -> std::io::Result<()> {
    let up_horizontal = if heavy {
        box_drawing::heavy::UP_HORIZONTAL
    } else {
        box_drawing::light::UP_HORIZONTAL
    };
    write_boxed_partial(writer, text, box_width, color, heavy)?;
    write!(
        writer,
        "{}",
        paint::paint_text_foreground(up_horizontal, color)
    )?;
    Ok(())
}

fn write_boxed_partial(
    writer: &mut dyn Write,
    text: &str,
    box_width: usize,
    color: Color,
    heavy: bool,
) -> std::io::Result<()> {
    let horizontal = if heavy {
        box_drawing::heavy::HORIZONTAL
    } else {
        box_drawing::light::HORIZONTAL
    };
    let down_left = if heavy {
        box_drawing::heavy::DOWN_LEFT
    } else {
        box_drawing::light::DOWN_LEFT
    };
    let vertical = if heavy {
        box_drawing::heavy::VERTICAL
    } else {
        box_drawing::light::VERTICAL
    };

    let horizontal_edge = horizontal.repeat(box_width);
    write!(
        writer,
        "{}{}\n{} {}\n{}",
        paint::paint_text_foreground(&horizontal_edge, color),
        paint::paint_text_foreground(down_left, color),
        paint::paint_text_foreground(text, color),
        paint::paint_text_foreground(vertical, color),
        paint::paint_text_foreground(&horizontal_edge, color),
    )
}
