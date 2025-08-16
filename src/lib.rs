#![cfg_attr(not(test), no_std)]

pub(crate) mod fmt;
mod info;
pub mod spec;
use embassy_time::Instant;
use embedded_graphics::prelude::PixelColor;
use info::Infomation;

use kolibri_embedded_gui::ui::Widget;
use rmk::channel::{CONTROLLER_CHANNEL, ControllerSub};
use rmk::controller::Controller;
use rmk::event::ControllerEvent;

use crate::info::InfoProvider;

pub trait AnimationWidget<Color: PixelColor>: Widget<Color> {
    fn new() -> Self;
    fn set(self, value: u8) -> Self;
}

pub trait DisplayDriver {
    fn draw<Info: InfoProvider>(&mut self, info: &Info);
}

pub struct DisplayController<DisplayImpl, const PERIPHERAL_COUNT: usize>
where
    DisplayImpl: DisplayDriver,
{
    sub: ControllerSub,
    disp: DisplayImpl,
    info: Infomation<PERIPHERAL_COUNT>,
    last_update: Instant,
}
impl<DisplayImpl, const PERIPHERAL_COUNT: usize> DisplayController<DisplayImpl, PERIPHERAL_COUNT>
where
    DisplayImpl: DisplayDriver,
{
    pub fn new(disp: DisplayImpl) -> Self {
        Self {
            sub: unwrap!(CONTROLLER_CHANNEL.subscriber()),
            disp: disp,
            info: Infomation::default(),
            last_update: Instant::MIN,
        }
    }
}

impl<DisplayImpl, const PERIPHERAL_COUNT: usize> Controller
    for DisplayController<DisplayImpl, PERIPHERAL_COUNT>
where
    DisplayImpl: DisplayDriver,
{
    type Event = ControllerEvent;
    async fn process_event(&mut self, event: Self::Event) {
        self.info.update(&event);
        if self.last_update.elapsed().as_millis() > 10 {
            self.disp.draw(&self.info);
            self.last_update = Instant::now();
        }
    }
    async fn next_message(&mut self) -> Self::Event {
        self.sub.next_message_pure().await
    }
}
