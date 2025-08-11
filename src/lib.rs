#![cfg_attr(not(test), no_std)]

pub(crate) mod fmt;
pub mod spec;
mod info;
use embedded_graphics::prelude::PixelColor;
use info::DispInfo;

use embedded_graphics::draw_target::DrawTarget;
use kolibri_embedded_gui::style::Style;
use rmk::channel::{CONTROLLER_CHANNEL, ControllerSub};
use rmk::controller::Controller;
use rmk::event::ControllerEvent;

pub trait DisplayUpdater {
    fn update(&mut self);
}

pub trait DisplayStyleProvider {
    type Color: PixelColor;
    fn style(&self) -> Style<Self::Color>;
}

pub trait DisplayDriver<Color: PixelColor>: DisplayUpdater + DisplayStyleProvider<Color = Color> + DrawTarget<Color = Color> {}

impl<T, Color> DisplayDriver<Color> for T where T: DisplayUpdater + DisplayStyleProvider<Color = Color> + DrawTarget<Color = Color>, Color: PixelColor {}

pub struct DisplayController<Color, DisplayImpl, const PERIPHERAL_COUNT: usize>
where
    Color: PixelColor,
    DisplayImpl: DisplayDriver<Color>,
{
    sub: ControllerSub,
    disp: DisplayImpl,
    info: DispInfo<PERIPHERAL_COUNT>,
    _phantom: core::marker::PhantomData<Color>,
}
impl<Color, DisplayImpl, const PERIPHERAL_COUNT: usize> DisplayController<Color, DisplayImpl, PERIPHERAL_COUNT> where
    Color: PixelColor,
    DisplayImpl: DisplayDriver<Color>,
{
    pub fn new(disp: DisplayImpl) -> Self {
        Self {
            sub: unwrap!(CONTROLLER_CHANNEL.subscriber()),
            disp: disp,
            info: DispInfo::default(),
            _phantom: core::marker::PhantomData
        }
    }
}

impl<Color, DisplayImpl, const PERIPHERAL_COUNT: usize> Controller for DisplayController<Color, DisplayImpl, PERIPHERAL_COUNT> where
    Color: PixelColor,
    DisplayImpl: DisplayDriver<Color>,
{
    type Event = ControllerEvent;
    async fn process_event(&mut self, event: Self::Event) {
        self.info.update_info(&event);
        self.redraw();
    }
    async fn next_message(&mut self) -> Self::Event {
        self.sub.next_message_pure().await
    }
}

impl<Color, DisplayImpl, const PERIPHERAL_COUNT: usize> DisplayController<Color, DisplayImpl, PERIPHERAL_COUNT> where
    Color: PixelColor,
    DisplayImpl: DisplayDriver<Color>,
{
    fn redraw(&mut self) {
        use kolibri_embedded_gui::ui::Ui;
        use kolibri_embedded_gui::label::Label;
        let style = self.disp.style();
        let mut ui = Ui::new_fullscreen(&mut self.disp, style);
        ui.clear_background().unwrap();

        ui.add(Label::new("Hello, RMK!"));

        let mut buffer = itoa::Buffer::new();
        let wpm_str = buffer.format(self.info.wpm);
        ui.add(Label::new("wpm:"));
        ui.add(Label::new(wpm_str));

        self.disp.update();
    }
}
