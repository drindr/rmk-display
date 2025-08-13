#![cfg_attr(not(test), no_std)]

pub(crate) mod fmt;
mod info;
pub mod spec;
use embedded_graphics::prelude::PixelColor;
use info::DispInfo;

use embedded_graphics::draw_target::DrawTarget;
use kolibri_embedded_gui::style::Style;
use kolibri_embedded_gui::ui::Widget;
use rmk::channel::{CONTROLLER_CHANNEL, ControllerSub};
use rmk::controller::Controller;
use rmk::event::ControllerEvent;

pub trait AnimationWidget<Color: PixelColor>: Widget<Color> {
    fn new() -> Self;
    fn set(self, value: u8) -> Self;
}

pub trait DisplayProvider<'a> {
    type Color: PixelColor;
    type Animation: AnimationWidget<Self::Color> + 'a;
    fn style(&self) -> Style<Self::Color>;
    fn update(&mut self);
}

pub trait DisplayDriver<'a, Color: PixelColor>:
    DisplayProvider<'a, Color = Color> + DrawTarget<Color = Color>
{
}

impl<'a, T, Color> DisplayDriver<'a, Color> for T
where
    T: DisplayProvider<'a, Color = Color> + DrawTarget<Color = Color>,
    Color: PixelColor,
{
}

pub struct DisplayController<'a, Color, DisplayImpl, const PERIPHERAL_COUNT: usize>
where
    Color: PixelColor,
    DisplayImpl: DisplayDriver<'a, Color>,
{
    sub: ControllerSub,
    disp: DisplayImpl,
    info: DispInfo<PERIPHERAL_COUNT>,
    _phantom: core::marker::PhantomData<&'a Color>,
}
impl<'a, Color, DisplayImpl, const PERIPHERAL_COUNT: usize>
    DisplayController<'a, Color, DisplayImpl, PERIPHERAL_COUNT>
where
    Color: PixelColor,
    DisplayImpl: DisplayDriver<'a, Color>,
{
    pub fn new(disp: DisplayImpl) -> Self {
        Self {
            sub: unwrap!(CONTROLLER_CHANNEL.subscriber()),
            disp: disp,
            info: DispInfo::default(),
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<'a, Color, DisplayImpl, const PERIPHERAL_COUNT: usize> Controller
    for DisplayController<'a, Color, DisplayImpl, PERIPHERAL_COUNT>
where
    Color: PixelColor,
    DisplayImpl: DisplayDriver<'a, Color>,
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

impl<'a, Color, DisplayImpl, const PERIPHERAL_COUNT: usize>
    DisplayController<'a, Color, DisplayImpl, PERIPHERAL_COUNT>
where
    Color: PixelColor,
    DisplayImpl: DisplayDriver<'a, Color>,
{
    fn redraw(&mut self) {
        use kolibri_embedded_gui::label::Label;
        use kolibri_embedded_gui::ui::Ui;
        let style = self.disp.style();
        let mut ui = Ui::new_fullscreen(&mut self.disp, style);
        ui.clear_background().unwrap();

        ui.add(Label::new("Hello, "));
        ui.add(Label::new("RMK!"));

        let mut buffer = itoa::Buffer::new();
        let wpm_str = buffer.format(self.info.wpm);
        ui.add_horizontal(Label::new("wpm:"));
        ui.add_horizontal(Label::new(wpm_str));
        if self.info.wpm != 0 {
            ui.add_horizontal(DisplayImpl::Animation::new().set(self.info.up as u8));
            self.info.up = !self.info.up;
        } else {
            ui.add_horizontal(DisplayImpl::Animation::new().set(0));
        }

        self.disp.update();
    }
}
