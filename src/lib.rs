#![cfg_attr(not(test), no_std)]

pub(crate) mod fmt;
mod style;
pub mod spec;
mod info;
use info::DispInfo;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::BinaryColor;
use rmk::channel::{CONTROLLER_CHANNEL, ControllerSub};
use rmk::controller::Controller;
use rmk::event::ControllerEvent;

pub trait DisplayUpdater {
    fn update(&mut self);
}

pub struct DisplayController<DISPLAY: DrawTarget<Color = BinaryColor> + DisplayUpdater, const PERIPHERAL_COUNT: usize> {
    sub: ControllerSub,
    disp: DISPLAY,
    info: DispInfo<PERIPHERAL_COUNT>
}

impl<DISPLAY: DrawTarget<Color = BinaryColor> + DisplayUpdater, const PERIPHERAL_COUNT: usize> DisplayController<DISPLAY, PERIPHERAL_COUNT> {
    pub fn new(disp: DISPLAY) -> Self {
        Self {
            sub: unwrap!(CONTROLLER_CHANNEL.subscriber()),
            disp: disp,
            info: DispInfo::default()
        }
    }
}

impl<DISPLAY: DrawTarget<Color = BinaryColor> + DisplayUpdater, const PERIPHERAL_COUNT: usize> Controller for DisplayController<DISPLAY, PERIPHERAL_COUNT> {
    type Event = ControllerEvent;
    async fn process_event(&mut self, event: Self::Event) {
        self.info.update_info(&event);
        self.redraw();
    }
    async fn next_message(&mut self) -> Self::Event {
        self.sub.next_message_pure().await
    }
}

impl<DISPLAY: DrawTarget<Color = BinaryColor> + DisplayUpdater, const PERIPHERAL_COUNT: usize> DisplayController<DISPLAY, PERIPHERAL_COUNT> {

    fn redraw(&mut self) {
        use kolibri_embedded_gui::ui::Ui;
        use kolibri_embedded_gui::label::Label;
        let mut ui = Ui::new_fullscreen(&mut self.disp, style::DISPLAY_STYLE);
        ui.clear_background().unwrap();

        ui.add(Label::new("Hello, RMK!"));

        let mut buffer = itoa::Buffer::new();
        let wpm_str = buffer.format(self.info.wpm);
        ui.add(Label::new("wpm:"));
        ui.add(Label::new(wpm_str));

        self.disp.update();
    }
}
