use rusty_bar::static_plot::StaticPlot;
use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor}
};

fn main() {
    let data: &[&[u32]] = &[&[32, 5, 17], &[20, 10, 15]];
    //let labels: &[&str] = &[&"2017", &"some year"];
    let labels = &[];
    let columns: &[&str] = &[&"Boys", &"Girls", &"Neuters"];
    let cp: Vec<Color> = vec![Color::Red, Color::Cyan, Color::White, Color::Green];
    let plotter = StaticPlot::new(data, labels, columns, false,  true, cp);
    plotter.print_static();
}
