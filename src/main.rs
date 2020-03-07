use rusty_bar::static_plot::StaticPlot;
use rusty_bar::graph_builder;
use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor}
};
use std::io::{stdout, Write};

fn main() {
    /*
    let data: &[&[u32]] = &[&[32, 5, 17], &[20, 10, 15]];
    let labels: &[&str] = &[&"2018", &"some year"];
    let columns: &[&str] = &[&"Boys", &"Girls", &"Neuters"];
    let mut graph = graph_builder::static_graph_builder::StaticGraphBuilder::new();
    graph.set_colors(&["white", "red", "cyan"])
        .load_2d_array(data, labels, columns)
        .build()        
        .print_static();
    */
    /*
    execute!(
        stdout(),
        Print("0123456789\n"),
        Print("0123456789\n"),
        Print("0123456789\n"),
        cursor::MoveToPreviousLine(3),
        Print("X\n"),
        //cursor::RestorePosition,
        //Print("$$$")
    ).unwrap();
    */
}
