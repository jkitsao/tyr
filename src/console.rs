/*
    Format and write to stdout
*/

#![allow(unused)]
use std::io::Write;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
//Cyan: show information
pub fn show_info(message: String) {
    let mut bufwtr = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();
    buffer
        .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
        .unwrap();
    writeln!(&mut buffer, "{}", message).unwrap();
    bufwtr.print(&buffer).unwrap();
}
//Red: show warning
pub fn show_error(message: String) {
    let mut bufwtr = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();
    buffer
        .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
        .unwrap();
    writeln!(&mut buffer, "{}", message).unwrap();
    bufwtr.print(&buffer).unwrap();
}
//Green: show success
pub fn show_success(message: String) {
    let mut bufwtr = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();
    buffer
        .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
        .unwrap();
    writeln!(&mut buffer, "{}", message).unwrap();
    bufwtr.print(&buffer).unwrap();
}
