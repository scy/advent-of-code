use std::convert::TryInto;
use std::io::{self, BufRead, BufReader};
use itertools::Itertools;

struct Row {
    pixels: Vec<u8>,
}

struct Layer {
    rows: Vec<Row>,
}

impl Layer {
    fn count_digits(&self, digit: u8) -> usize {
        self.rows.iter().map(|row| row.pixels.iter().filter(|&&p| p == digit).count()).sum()
    }
}

struct Image {
    layers: Vec<Layer>,
}

impl Image {
    fn from_string(input: &str, width: usize, height: usize) -> Self {
        let len = input.len();
        let num_layers = len / (width * height);
        let mut chars = input.chars();
        let mut image = Image { layers: vec![] };
        for _ in 0..num_layers {
            let mut layer = Layer { rows: vec![] };
            for _ in 0..height {
                let mut row = Row { pixels: vec![] };
                for _ in 0..width {
                    row.pixels.push(chars.next().unwrap().to_digit(10).unwrap().try_into().unwrap());
                }
                layer.rows.push(row);
            }
            image.layers.push(layer);
        }
        image
    }

    fn from_stdin(width: usize, height: usize) -> Self {
        let buffered = BufReader::new(io::stdin());
        Self::from_string(&buffered.lines().map(|line| line.unwrap()).join(","), width, height)
    }

    fn get_layer_with_fewest(&self, digit: u8) -> &Layer {
        let mut min_count = None;
        let mut found_layer = None;
        for layer in self.layers.iter() {
            let count = layer.count_digits(digit);
            match min_count {
                None => {
                    min_count = Some(count);
                    found_layer = Some(layer);
                }
                Some(min_count_val) => {
                    if count < min_count_val {
                        min_count = Some(count);
                        found_layer = Some(layer);
                    }
                }
            }
        }
        found_layer.unwrap()
    }
}

fn main() {
    let image = Image::from_stdin(25, 6);
    let layer = image.get_layer_with_fewest(0);

    println!("Checksum (1 digits multiplied by 2 digits): {}", layer.count_digits(1) * layer.count_digits(2));
}

#[test]
fn example() {
    let image = Image::from_string("123456789012", 3, 2);
    let layer = image.get_layer_with_fewest(0);
    assert_eq!(layer.count_digits(1) * layer.count_digits(2), 1);
}
