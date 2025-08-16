use crate::AnimationWidget;
use crate::info::InfoProvider;
use crate::{DisplayController, DisplayDriver};
use embedded_hal::{digital::OutputPin, spi::SpiBus};
use memory_lcd_spi::{
    DisplaySpec, MemoryLCD,
    framebuffer::{FramebufferBW, Rotation, Sharp},
};

use embedded_graphics::geometry::Size;
use embedded_graphics::mono_font;
use embedded_graphics::pixelcolor::BinaryColor;
use kolibri_embedded_gui::style::Spacing;
use kolibri_embedded_gui::style::Style;

pub(crate) const DISPLAY_STYLE: Style<BinaryColor> = Style {
    background_color: BinaryColor::Off,
    text_color: BinaryColor::On,
    primary_color: BinaryColor::On,
    spacing: Spacing {
        item_spacing: Size::new(10, 5),
        button_padding: Size::new(4, 4),
        default_padding: Size::new(2, 2),
        window_border_padding: Size::new(3, 3),
    },
    default_font: mono_font::ascii::FONT_6X13,
    border_color: BinaryColor::On,
    border_width: 1,
    default_widget_height: 16,
    icon_color: BinaryColor::On,
    secondary_color: BinaryColor::Off,
    highlight_border_color: BinaryColor::On,
    highlight_border_width: 2,
    highlight_item_background_color: BinaryColor::Off,
    item_background_color: BinaryColor::Off,
};

pub type BongoCatAnimation<'a> = bongo_cat_impl::BongoCat<'a>;

#[bongo_cat::bongo_cat(binary, width = 60, height = 60, both)]
mod bongo_cat_impl {
    use crate::AnimationWidget;
    use embedded_graphics::draw_target::DrawTarget;
    use embedded_graphics::geometry::{Point, Size};
    use embedded_graphics::image::{Image, ImageRaw};
    use embedded_graphics::pixelcolor::BinaryColor;
    use embedded_graphics::transform::Transform;
    use kolibri_embedded_gui::smartstate::{Container, Smartstate};
    use kolibri_embedded_gui::ui::{GuiResult, Response, Ui, Widget};
    pub struct BongoCat<'a> {
        up: u8,
        smartstate: Container<'a, Smartstate>,
    }
    impl<'a> BongoCat<'a> {
        pub fn smartstate(mut self, smartstate: &'a mut Smartstate) -> Self {
            self.smartstate.set(smartstate);
            self
        }
    }
    impl Widget<BinaryColor> for BongoCat<'_> {
        fn draw<DRAW: DrawTarget<Color = BinaryColor>>(
            &mut self,
            ui: &mut Ui<DRAW, BinaryColor>,
        ) -> GuiResult<Response> {
            let iresponse = ui.allocate_space(Size::new(WIDTH, HEIGHT))?;
            let redraw = !self
                .smartstate
                .eq_option(&Some(Smartstate::state(self.up.into())));
            self.smartstate
                .modify(|st| *st = Smartstate::state(self.up.into()));
            if redraw {
                ui.start_drawing(&iresponse.area);
                let raw_image = if self.up % 2 != 0 {
                    ImageRaw::<BinaryColor>::new(BOTH_UP, WIDTH)
                } else {
                    ImageRaw::<BinaryColor>::new(DEFAULT, WIDTH)
                };
                let mut image = Image::new(&raw_image, Point::zero());
                image.translate_mut(iresponse.area.top_left);
                ui.draw(&image)?;
                ui.finalize()?;
            }
            Ok(Response::new(iresponse))
        }
    }
    impl AnimationWidget<BinaryColor> for BongoCat<'_> {
        fn new() -> Self {
            Self {
                smartstate: Container::empty(),
                up: 0,
            }
        }
        fn set(mut self, state: u8) -> Self {
            self.up = state % 2;
            self
        }
    }
}

pub struct NiceViewDisplaySpec;
impl DisplaySpec for NiceViewDisplaySpec {
    const WIDTH: u16 = 160;
    const HEIGHT: u16 = 68;
    const SIZE: usize = Self::WIDTH as usize * Self::HEIGHT as usize / 2;

    type Framebuffer = FramebufferBW<{ Self::WIDTH }, { Self::HEIGHT }, { Self::SIZE }, Sharp>;
}

pub struct NiceView<SPI, CS, Animation>
where
    SPI: SpiBus<u8>,
    CS: OutputPin,
    Animation: AnimationWidget<BinaryColor>,
{
    lcd: MemoryLCD<NiceViewDisplaySpec, SPI, CS>,
    increment: u8,
    _phantom: core::marker::PhantomData<Animation>,
}

impl<SPI, CS, Animation> NiceView<SPI, CS, Animation>
where
    SPI: SpiBus<u8>,
    CS: OutputPin,
    Animation: AnimationWidget<BinaryColor>,
{
    pub fn new(lcd: MemoryLCD<NiceViewDisplaySpec, SPI, CS>) -> Self {
        Self {
            lcd,
            increment: 0,
            _phantom: core::marker::PhantomData,
        }
    }

    pub fn new_controller<const PERIPHERAL_COUNT: usize>(
        spi_bus: SPI,
        cs: CS,
    ) -> DisplayController<Self, PERIPHERAL_COUNT> {
        let mut lcd = MemoryLCD::<NiceViewDisplaySpec, SPI, CS>::new(spi_bus, cs);
        lcd.set_rotation(Rotation::Deg270);
        let niceview = NiceView::new(lcd);
        let controller = DisplayController::new(niceview);
        controller
    }

    fn style() -> Style<BinaryColor> {
        DISPLAY_STYLE
    }
}

impl<SPI, CS, Animation> DisplayDriver for NiceView<SPI, CS, Animation>
where
    SPI: SpiBus<u8>,
    CS: OutputPin,
    Animation: AnimationWidget<BinaryColor>,
{
    fn draw<Info: InfoProvider>(&mut self, info: &Info) {
        use kolibri_embedded_gui::label::Label;
        use kolibri_embedded_gui::ui::Ui;
        let style = Self::style();
        let mut ui = Ui::new_fullscreen(&mut *self.lcd, style);
        ui.clear_background().unwrap();

        ui.add(Label::new("RMK!"));

        let mut buffer = itoa::Buffer::new();
        let wpm_str = buffer.format(info.wpm());
        ui.add_horizontal(Label::new("wpm:"));
        ui.add_horizontal(Label::new(wpm_str));
        if info.wpm() != 0 {
            ui.add_horizontal(Animation::new().set(self.increment));
            self.increment = self.increment.wrapping_add(1);
        } else {
            ui.add_horizontal(Animation::new().set(0));
            self.increment = 0;
        }
        self.lcd.update().unwrap();
    }
}
