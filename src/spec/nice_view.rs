use crate::{DisplayController, DisplayProvider};
use crate::AnimationWidget;
use embedded_hal::{digital::OutputPin, spi::SpiBus};
use memory_lcd_spi::{
    DisplaySpec, MemoryLCD,
    framebuffer::{FramebufferBW, Rotation, Sharp},
};

use embedded_graphics::Pixel;
use embedded_graphics::mono_font;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
};
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

pub struct NiceView;
impl DisplaySpec for NiceView {
    const WIDTH: u16 = 160;
    const HEIGHT: u16 = 68;
    const SIZE: usize = Self::WIDTH as usize * Self::HEIGHT as usize / 2;

    type Framebuffer = FramebufferBW<{ Self::WIDTH }, { Self::HEIGHT }, { Self::SIZE }, Sharp>;
}

pub fn new_controller<SPI, CS, Animation, const PERIPHERAL_COUNT: usize>(
    spi_bus: SPI,
    cs: CS,
) -> DisplayController<BinaryColor, Wrapper<SPI, CS>, Animation, PERIPHERAL_COUNT>
where
    SPI: SpiBus<u8>,
    CS: OutputPin,
    Animation: AnimationWidget<BinaryColor>,
{
    let mut lcd = MemoryLCD::<NiceView, SPI, CS>::new(spi_bus, cs);
    lcd.set_rotation(Rotation::Deg270);
    let wrapper = Wrapper { lcd };
    let controller = DisplayController::new(wrapper);
    controller
}

pub struct Wrapper<SPI: SpiBus<u8>, CS: OutputPin> {
    lcd: MemoryLCD<NiceView, SPI, CS>,
}

impl<SPI: SpiBus<u8>, CS: OutputPin> OriginDimensions for Wrapper<SPI, CS> {
    fn size(&self) -> Size {
        self.lcd.size()
    }
}

impl<SPI: SpiBus<u8>, CS: OutputPin> DrawTarget for Wrapper<SPI, CS> {
    type Color = BinaryColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.lcd.draw_iter(pixels)
    }
    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.lcd.clear(color)
    }
}

impl<SPI: SpiBus<u8>, CS: OutputPin> DisplayProvider for Wrapper<SPI, CS> {
    type Color = BinaryColor;

    fn style(&self) -> Style<Self::Color> {
        DISPLAY_STYLE
    }
    fn update(&mut self) {
        self.lcd.update().unwrap();
    }
}
