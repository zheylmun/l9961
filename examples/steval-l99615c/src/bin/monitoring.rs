#![no_main]
#![no_std]

use embassy_executor::Spawner;
use embassy_time::Delay;
use steval_l99615c::{self as functions, initialize_l9961};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let peripherals = embassy_stm32::init(Default::default());
    let mut l9961 = initialize_l9961(peripherals);
    let mut delay = Delay;
    l9961.wake_if_asleep(&mut delay).await;
    l9961.disable_measurements().await.unwrap();

    l9961.go_2_standby().await.unwrap();

    functions::exit()
}
