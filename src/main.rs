use rusty_bar::static_plot::print_static;

fn main() {
    let data: &[&[u32]] = &[&[32, 5, 17], &[20, 10, 15]];
    let labels: &[&str] = &[&"2017", &"some year"];
    let columns: &[&str] = &[&"Boys", &"Girls", &"Neuters"];
    print_static(data, labels, columns);
}

