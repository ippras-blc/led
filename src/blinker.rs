use crate::{Led, BLACK, RGB8};
use async_channel::Receiver;
use esp_idf_svc::timer::EspAsyncTimer;
use std::time::Duration;

const ON: Duration = Duration::from_millis(100);
const OFF: Duration = Duration::from_millis(10);

impl Led<'_> {
    pub async fn blinker(&mut self, mut timer: EspAsyncTimer, receiver: Receiver<RGB8>) {
        while let Ok(color) = receiver.recv().await {
            self.set_color(color).ok();
            timer.after(ON).await.ok();
            self.set_color(BLACK).ok();
            timer.after(OFF).await.ok();
        }
    }
}
