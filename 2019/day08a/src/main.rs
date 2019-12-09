use anyhow::{anyhow, Result};
use std::io::{stdin, BufRead};

#[derive(Debug)]
struct SpaceImage<'a> {
    data: &'a str,
    width: usize,
    height: usize,
}

#[derive(Debug)]
struct SpaceImageLayer<'a> {
    data: &'a str,
    width: usize,
    height: usize,
}

impl<'a> SpaceImage<'a> {
    fn layer_size(&self) -> usize {
        self.width * self.height
    }

    fn layers(&self) -> Vec<SpaceImageLayer<'a>> {
        let layer_size = self.layer_size();
        (0..self.data.len())
            .step_by(self.width * self.height)
            .map(|start| SpaceImageLayer {
                data: &self.data[start..start + layer_size],
                width: self.width,
                height: self.height,
            })
            .collect()
    }
}

fn num_zeroes(s: &str) -> usize {
    s.chars()
        .map(|c| match c {
            '0' => 1,
            _ => 0,
        })
        .sum()
}

fn main() -> Result<()> {
    let raw: String = stdin()
        .lock()
        .lines()
        .next()
        .ok_or_else(|| anyhow!("expected a line"))??;
    let image = SpaceImage {
        data: &raw,
        width: 25,
        height: 6,
    };
    let mut sorted_layers = image.layers();
    sorted_layers.sort_by(|a, b| num_zeroes(a.data).cmp(&num_zeroes(b.data)));
    let (ones, twos) = sorted_layers[0]
        .data
        .chars()
        .fold((0, 0), |(ones, twos), c| {
            (ones + (c == '1') as usize, twos + (c == '2') as usize)
        });
    println!("{}", ones * twos);
    Ok(())
}
