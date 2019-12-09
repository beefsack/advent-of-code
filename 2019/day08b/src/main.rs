use anyhow::{anyhow, Result};
use std::io::{stdin, BufRead};

#[derive(Debug)]
struct SpaceImage<'a> {
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

    fn render(&self) -> String {
        let layers = self.layers();
        if layers.is_empty() {
            return "".to_string();
        }
        let mut output = layers[0].data.as_bytes().to_owned();
        for l in layers.into_iter().skip(1) {
            for (index, c) in l.data.as_bytes().iter().enumerate() {
                if output[index] == b'2' {
                    output[index] = *c;
                }
            }
        }
        (0..output.len())
            .step_by(self.width)
            .map(|start| String::from_utf8(output[start..start + self.width].to_owned()).unwrap())
            .collect::<Vec<String>>()
            .join("\n")
            .replace("0", " ")
            .replace("1", "#")
    }
}

#[derive(Debug)]
struct SpaceImageLayer<'a> {
    data: &'a str,
    width: usize,
    height: usize,
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
    println!("{}", image.render());
    Ok(())
}
