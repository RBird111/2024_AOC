use console::style;

use std::fmt::Debug;

pub fn print_results<T: Debug>(day: usize, parts: &[T]) {
    let style_part = |(num, part): (usize, &T)| {
        format!(
            "{}: {:?}",
            style(format!("\n\t[Part {}]", num + 1)).magenta().bold(),
            style(part).bold()
        )
    };
    let day = style(format!("[Day {day}]")).yellow().bold();
    let parts: String = parts.iter().enumerate().map(style_part).collect();
    print!("\n{day}{parts}\n")
}
