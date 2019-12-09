use rayon::prelude::*;
use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
struct Image {
    layers: HashMap<usize, HashMap<usize, Vec<usize>>>,
    height: usize,
    width: usize,
    number_of_layers: usize,
}

impl Image {
    pub fn from_string(input: &str, height: usize, width: usize) -> Image {
        let number_of_layers = input.len() / (height * width);
        let mut layers = HashMap::new();

        (0..number_of_layers).for_each(|x| {
            let mut layer = HashMap::new();

            (0..height).for_each(|y| {
                let mut row: Vec<usize> = Vec::with_capacity(width);
                let correct_layer = x * height * width;
                let correct_row = correct_layer + (y * width);
                input[correct_row..correct_row + width]
                    .chars()
                    .for_each(|x| row.push(x.to_digit(10).unwrap() as usize));

                layer.insert(y, row);
            });

            layers.insert(x, layer);
        });

        Image {
            layers,
            height,
            width,
            number_of_layers,
        }
    }

    fn find_fewest_digits_layer(&self, input: usize) -> usize {
        let mut input_count = std::usize::MAX;
        let mut layer_number = None;
        (0..self.layers.len()).for_each(|x| {
            let count = self.count_digit(x, 0);

            if count < input_count {
                input_count = count;
                layer_number = Some(x);
            };
        });
        layer_number.unwrap()
    }

    fn count_digit(&self, layer: usize, input: usize) -> usize {
        let layer = self.layers.get(&layer).unwrap();

        let mut count = 0;

        layer.iter().for_each(|rows| {
            let row = rows.1;

            count += row.iter().filter(|val| **val == input).count();
        });

        count
    }

    fn into_renderable_image(&self) -> RenderableImage {
        let mut image = HashMap::new();

        (0..self.height).for_each(|h| {
            let mut row = Vec::with_capacity(self.width);
            (0..self.width).for_each(|w| {
                row.push(self.get_pixel(0, h, w));
            });

            image.insert(h, row);
        });

        RenderableImage {
            image,
            height: self.height,
            width: self.width,
        }
    }

    fn get_pixel(&self, layer: usize, row: usize, column: usize) -> Pixel {
        match self
            .layers
            .get(&layer)
            .unwrap()
            .get(&row)
            .expect("get_pixel")[column]
        {
            0 => Pixel::Black,
            1 => Pixel::White,
            2 => {
                if layer + 1 >= self.number_of_layers {
                    Pixel::Transparent
                } else {
                    self.get_pixel(layer + 1, row, column)
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct RenderableImage {
    image: HashMap<usize, Vec<Pixel>>,
    height: usize,
    width: usize,
}

impl fmt::Display for RenderableImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (0..self.height).for_each(|row_num| {
            self.image
                .get(&row_num)
                .unwrap()
                .iter()
                .for_each(|pixel| match pixel {
                    Pixel::Black => write!(f, "■").expect("Pixel::Black"),
                    Pixel::White => write!(f, "□").expect("Pixel::White"),
                    Pixel::Transparent => write!(f, " ").expect("Pixel::Transparent"),
                });
            write!(f, "\n").expect("here");
        });
        write!(f, "")
    }
}

#[derive(Debug, Clone, Copy)]
enum Pixel {
    Transparent,
    White,
    Black,
}

// #[aoc_generator(day8)]
// fn process_input(input: &str) -> Image {
//     input.trim().lines().map(|x| Orbit::new(x)).collect()
// }

#[aoc(day8, part1)]
fn d8p1(input: &str) -> usize {
    let image = Image::from_string(input, 6, 25);

    let layer = image.find_fewest_digits_layer(0);
    debug_print!("Layer is : {:?}", layer);
    debug_print!("Layer : {:?}", image.layers.get(&layer).unwrap());
    let ones = image.count_digit(layer, 1);
    debug_print!("Ones : {:?}", ones);
    let twos = image.count_digit(layer, 2);
    debug_print!("Twos : {:?}", twos);

    ones * twos
}

#[aoc(day8, part2)]
fn d8p2(input: &str) -> usize {
    let image = Image::from_string(input, 6, 25);
    let render = image.into_renderable_image();
    println!("{}", render);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let input = "123456222012";
        let image = Image::from_string(input, 2, 3);

        let layer = image.find_fewest_digits_layer(0);
        let ones = image.count_digit(layer, 1);
        let twos = image.count_digit(layer, 2);

        assert_eq!(ones * twos, 1);
    }
}
