use console::style;

use std::fmt::Debug;

pub fn print_results<T: Debug>(day: usize, part_1: T, part_2: T) {
    let style_part = |num: usize, part: T| {
        format!(
            "{}: {:?}",
            style(format!("[Part {num}]")).magenta().bold(),
            style(part).bold()
        )
    };
    let day = style(format!("[Day {day}]")).yellow().bold();
    let p1 = style_part(1, part_1);
    let p2 = style_part(2, part_2);
    print!("\n{day}:\n\t{p1}\n\t{p2}\n")
}
