use std::io::{self, Read};

fn layer_fewest_0_result(picture: &[u8], width: usize, heigth: usize) -> usize {
    let layer_size = width * heigth;
    let layer = picture
        .chunks(layer_size)
        .min_by_key(|c| bytecount::count(c, b'0'))
        .unwrap();
    bytecount::count(layer, b'1') * bytecount::count(layer, b'2')
}

fn part2(picture: &[u8], width: usize, heigth: usize) -> i64 {
    0
}

fn main() {
    let mut picture = Vec::new();
    io::stdin().read_to_end(&mut picture).unwrap();

    println!("Part 1: {}", layer_fewest_0_result(&picture, 25, 6));
    println!("Part 2: {}", part2(&picture, 25, 6));
}
