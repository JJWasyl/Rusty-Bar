use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use rusty_bar::graph_builder;
use std::io::{stdout, Write};
use std::{thread, time};

fn main() {
    let data: &[&[u32]] = &[&[100, 250, 170], &[200, 100, 150]];
    let labels: &[&str] = &[&"2018", &"some year"];
    let columns: &[&str] = &[&"Boys", &"Girls", &"Neuters"];

    let mut graph = graph_builder::static_graph_builder::StaticGraphBuilder::new()
        .set_colors(&["white", "red", "cyan"])
        .load_2d_array(data, labels, columns)
        .view_legend(true)
        .build();
    graph.print_static();

    let one_sec = time::Duration::from_millis(1000);
    let new_data: &[&[u32]] = &[&[15, 6, 13], &[11, 7, 25]];
    //thread::sleep(one_sec);
    //graph.refresh_data(new_data);

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
