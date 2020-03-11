use core::pin::Pin;
use futures::stream::Stream;
use futures::task::{Context, Poll};
use rand;
use rand::Rng;
use rusty_bar::graph_builder;
use std::io::stdout;
use std::{thread, time};

fn main() {
    let data = vec![vec![100, 250, 170], vec![200, 100, 150]];
    let labels: &[&str] = &[&"Station1", &"Station2"];
    let columns: &[&str] = &[&"CPU", &"GPU", &"MEMORY"];
    let rng = rand::thread_rng();

    let mut graph = graph_builder::GraphBuilder::new()
        .set_colors(&["red", "green", "cyan"])
        .load_2d_vec(data)
        .load_labels(labels)
        .load_col_labels(columns)
        .view_legend(true)
        .build();
    graph.print_static(stdout());

    let wait = time::Duration::from_millis(500);
    loop {
        thread::sleep(wait);
        let new_data = generate_new_data(3, 2, 250, rng);
        graph.refresh_2d_data(new_data, stdout());
    }
}

fn generate_new_data(
    x: usize,
    y: usize,
    max: u32,
    mut rng: rand::rngs::ThreadRng,
) -> Vec<Vec<u32>> {
    let mut new_data: Vec<Vec<u32>> = vec![vec![0; x]; y];
    let max_val = max;
    for row in 0..new_data.len() {
        for col in 0..new_data[0].len() {
            new_data[row][col] = rng.gen_range(0u32, max_val);
        }
    }
    new_data
}

struct DataGenerator {
    rng: rand::rngs::ThreadRng,
    labels: usize,
    columns: usize,
    max: u32,
}

impl DataGenerator {
    pub fn new(y: usize, x: usize, max_val: u32) -> Self {
        DataGenerator {
            rng: rand::thread_rng(),
            labels: y,
            columns: x,
            max: max_val,
        }
    }
}

impl Stream for DataGenerator {
    type Item = Vec<Vec<u32>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut new_data: Vec<Vec<u32>> = vec![vec![0; self.columns]; self.labels];
        let max_val = self.max;
        for row in 0..new_data.len() {
            for col in 0..new_data[0].len() {
                new_data[row][col] = self.rng.gen_range(0u32, max_val);
            }
        }
        Poll::Ready(Some(new_data))
    }
}
