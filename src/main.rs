use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::sys::*;
use rand::Rng;
use smart_leds::{SmartLedsWrite, RGB8};
use std::thread::sleep;
use std::time::Duration;
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

mod snake;
use crate::snake::SnakeGame;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");
    log::info!("Starting shenanigans...");

    let num_pixels = 64;
    let peripherals = Peripherals::take().unwrap();
    let led_pin = peripherals.pins.gpio5;
    let channel = peripherals.rmt.channel0;
    let mut ws2812 = Ws2812Esp32Rmt::new(channel, led_pin).unwrap();

    let mut last_dir = 0;
    let mut dir = 2;
    let mut new_dir = 0;

    let board_size = 8;
    let snake_size = 4;
    let mut new_game = SnakeGame::init(board_size, snake_size);

    let mut rng = rand::thread_rng();

    let mut r_c: u8 = 0;
    let mut g_c: u8 = 0;
    let mut b_c: u8 = 0;

    loop {

        new_dir = new_game.get_dir();
        
        log::info!("Direction: {}", new_dir);

        new_game.update(new_dir);

        let mut pixels: Vec<RGB8> = vec![RGB8 {r: 0, g: 0, b: 0}; (board_size * board_size) as usize];

        if (new_game.get_cnt() % 5 == 0) {
            r_c = rng.gen_range(0..50);
            g_c = rng.gen_range(0..255);
            b_c = rng.gen_range(0..255);
        }

        for (val, pixel) in new_game.get_board().iter_mut().zip(pixels.iter_mut()) {
            if *val == 2 {
                
                let snake_color = RGB8 {r: r_c, g: g_c, b: b_c};

                *pixel = snake_color;
            } else if *val == 1 {
                *pixel = RGB8 {r: 255, g: 0, b: 0};
            }
        }

        ws2812.write(pixels).unwrap();

        sleep(Duration::from_millis(100));

    }
}
