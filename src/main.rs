use minifb::{Key, Scale, Window, WindowOptions};

use rand::prelude::*;
use std::usize;

const WINDOW_WIDTH: usize = 320;
const WINDOW_HEIGHT: usize = 200;

const FIRE_WIDTH: usize = 320;
const FIRE_HEIGHT: usize = 168;

const PALETTE: [u32; 37] = [
    0x07_07_07, 0x2F_0F_07, 0x2F_0F_07, 0x47_0F_07, 0x57_17_07, 0x67_1F_07, 0x77_1F_07, 0x8F_27_07,
    0x9F_2F_07, 0xAF_3F_07, 0xBF_47_07, 0xC7_47_07, 0xDF_4F_07, 0xDF_57_07, 0xDF_57_07, 0xD7_5F_07,
    0xD7_5F_07, 0xD7_67_0F, 0xCF_6F_0F, 0xCF_77_0F, 0xCF_7F_0F, 0xCF_87_17, 0xC7_87_17, 0xC7_8F_17,
    0xC7_97_1F, 0xBF_9F_1F, 0xBF_9F_1F, 0xBF_A7_27, 0xBF_A7_27, 0xBF_AF_2F, 0xB7_AF_2F, 0xB7_B7_2F,
    0xB7_B7_37, 0xCF_CF_6F, 0xDF_DF_9F, 0xEF_EF_C7, 0xFF_FF_FF,
];

fn init_fire(fire_pixels: &mut [usize]) {
    let mut color: usize = 0; // palette index 0
    for y in 0..(FIRE_HEIGHT - 1) {
        if y == (FIRE_HEIGHT - 2) {
            color = PALETTE.len() - 1;
        }
        for x in 0..(FIRE_WIDTH - 2) {
            fire_pixels[(FIRE_WIDTH * y) + x] = color
        }
    }
}

fn do_fire(fire_pixels: &mut [usize]) {
    for x in 0..(FIRE_WIDTH - 1) {
        for y in 2..(FIRE_HEIGHT - 1) {
            spread_fire(fire_pixels, y * FIRE_WIDTH + x);
        }
    }
}

fn spread_fire(fire_pixels: &mut [usize], from: usize) {
    let mut pixel = fire_pixels[from];
    if pixel == 0 {
        fire_pixels[from - FIRE_WIDTH] = 0;
    } else {
        let mut rng = rand::thread_rng();
        let rand = rng.gen_range(0..=7);
        let dst: usize = from.saturating_sub(rand) + 1;
        pixel = pixel.saturating_sub(rand & 1);
        // println!("dst: {}  pixel: {}", dst, pixel);
        fire_pixels[dst - FIRE_WIDTH] = pixel;
    }
}

fn main() {
    let mut window = Window::new(
        "Rust Fire - Press ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions {
            resize: true,
            scale: Scale::X1,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create window");

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // All elements initialized with 0
    // See https://doc.rust-lang.org/rust-by-example/primitives/array.html
    let mut fire_pixels: [usize; FIRE_WIDTH * FIRE_HEIGHT] = [0; FIRE_WIDTH * FIRE_HEIGHT];
    let mut buffer: Vec<u32> = Vec::with_capacity(WINDOW_WIDTH * WINDOW_HEIGHT);
    let mut size = (0, 0);

    init_fire(&mut fire_pixels);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let new_size = (window.get_size().0, window.get_size().1);
        if new_size != size {
            size = new_size;
            buffer.resize(size.0 * size.1, 0);
        }

        do_fire(&mut fire_pixels);

        let fire_offset = (WINDOW_HEIGHT - FIRE_HEIGHT) * WINDOW_WIDTH;
        for i in 0..fire_pixels.len() - 1 {
            buffer[fire_offset + i] = PALETTE[fire_pixels[i]];
        }

        window
            .update_with_buffer(&buffer, new_size.0, new_size.1)
            .unwrap();
    }
}
