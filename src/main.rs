#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use presto_rp2350_rs::{self, peripherals};
use smart_leds::RGB8;
use {defmt_rtt as _, panic_probe as _};

use embassy_rp::block::ImageDef;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

use presto_rp2350_rs::audio::Notes;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Program start");
    let mut bsp = peripherals::init(Default::default()).await;
    bsp.BUZZER.play(Notes::C4).await;

    bsp.LEDS.set_light(1, RGB8::new(0, 255, 0));
    bsp.LEDS.update().await;

    let delay = Duration::from_secs(1);
    loop {
        info!("led on!");
        Timer::after(delay).await;

        info!("led off!");
        Timer::after(delay).await;
    }
}
