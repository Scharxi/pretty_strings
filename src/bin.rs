use pretty_strings::{Animation, Colored, Text, TextStyle};


fn main() {
    let text = Text::new("Hello World");
    println!("Underline bold: {}", text.underline().bold());
    println!("bold: {}", text.bold());
    println!("underline: {}", text.underline());
    println!("italic: {}", text.italic());
    println!("strikethrough: {}", text.strikethrough());
    println!("colored: {}", text.color(0xFF0000));
    text.step_by_step(0.5);
}