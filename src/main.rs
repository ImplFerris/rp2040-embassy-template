#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;

{% if defmt -%}
// defmt Logging
use defmt_rtt as _;
use defmt::info;

use panic_probe as _;
{% else -%}
use panic_halt as _;
{% endif %}
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    {% if defmt %}
    info!("Initializing the program");
    {% endif %}
    loop {
        Timer::after_secs(1).await;
    }
}
