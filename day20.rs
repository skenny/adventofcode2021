use crate::aoc;

pub fn run() {
    println!("AOC 2021 - Day 20");

    let sample_input = aoc::read_input("input/day20-sample.txt");
    println!("part 1 = {}", enhance(&sample_input, 2));
    println!("part 2 = {}", enhance(&sample_input, 50));

    let real_input = aoc::read_input("input/day20.txt");
    println!("part 1 = {}", enhance(&real_input, 2));
    println!("part 2 = {}", enhance(&real_input, 50));
}

fn enhance(input: &[String], iterations: i8) -> i64 {
    let (enhancement_algo, mut image) = parse_input(input);

    let mut infinite_value = false;
    for _iteration in 0..iterations {
        let mut new_image = vec![vec![false; image.len() + 2]; image.len() + 2];
        let mut new_infinite_value = false;
        for y in 0..new_image.len() {
            for x in 0..new_image.len() {
                let orig_x = x as isize - 1;
                let orig_y = y as isize - 1;
                let pixels = find_pixels(orig_x, orig_y, &image, infinite_value);
                let binary = pixels.iter().map(|pixel| if *pixel { "1" } else { "0" }).collect::<Vec<&str>>().join("");
                let decimal = usize::from_str_radix(&binary, 2).unwrap();
                let enhancement_pixel = enhancement_algo[decimal];
                if orig_x < 0 && orig_y < 0 {
                    new_infinite_value = enhancement_pixel;
                }
                new_image[y][x] = enhancement_pixel;
            }
        }
        infinite_value = new_infinite_value;
        image = new_image;
    }

    //print_image(&image);

    let mut lit_pixels = 0;
    for y in 0..image.len() {
        for x in 0..image.len() {
            if image[y][x] { lit_pixels += 1; }
        }
    }
    lit_pixels
}

fn find_pixels(x: isize, y: isize, image: &Vec<Vec<bool>>, infinite_value: bool) -> Vec<bool> {
    let mut pixels: Vec<bool> = Vec::new();
    let image_size = image.len() as isize;
    for vy in -1..=1 {
        for vx in -1..=1 {
            let nx = x + vx;
            let ny = y + vy;
            if nx >= 0 && nx < image_size && ny >= 0 && ny < image_size {
                pixels.push(image[ny as usize][nx as usize]);
            } else {
                pixels.push(infinite_value);
            }
        }
    }
    pixels
}

fn parse_input(input: &[String]) -> (Vec<bool>, Vec<Vec<bool>>) {
    let enhancement_algo = input[0].chars().map(|c| if c == '.' { false } else { true }).collect();
    let mut image: Vec<Vec<bool>> = Vec::new();
    for line in 2..input.len() {
        image.push(input[line].chars().map(|c| if c == '.' { false } else { true }).collect());
    }
    (enhancement_algo, image)
}

fn print_image(image: &Vec<Vec<bool>>) {
    image.iter().for_each(|row| println!("{}", row.iter().map(|pixel| if *pixel { "#" } else { "." }).collect::<Vec<&str>>().join("")));
}