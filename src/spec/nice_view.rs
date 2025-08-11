use memory_lcd_spi::{MemoryLCD, framebuffer::{Sharp, FramebufferBW}, DisplaySpec};
use embedded_hal::{spi::SpiBus, digital::OutputPin};
use crate::{DisplayController, DisplayUpdater, DisplayStyleProvider};

use embedded_graphics::{pixelcolor::BinaryColor};
use embedded_graphics::{draw_target::DrawTarget, geometry::{OriginDimensions, Size}};
use embedded_graphics::Pixel;
use embedded_graphics::mono_font;
use kolibri_embedded_gui::style::Style;
use kolibri_embedded_gui::style::Spacing;

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

pub struct NiceView;
impl DisplaySpec for NiceView {
    const WIDTH: u16 = 160;
    const HEIGHT: u16 = 68;
    const SIZE: usize = Self::WIDTH as usize * Self::HEIGHT as usize / 2;

    type Framebuffer = FramebufferBW<{ Self::WIDTH }, { Self::HEIGHT }, {Self::SIZE}, Sharp>;
}

pub fn create_controller<SPI: SpiBus<u8>, CS: OutputPin, const PERIPHERAL_COUNT: usize>(spi_bus: SPI, cs: CS) -> DisplayController<BinaryColor, Wrapper<SPI, CS>, PERIPHERAL_COUNT>{
    let lcd = MemoryLCD::<NiceView, SPI, CS>::new(spi_bus, cs);
    let wrapper = Wrapper{lcd};
    let controller = DisplayController::new(wrapper);
    controller
}

pub struct Wrapper <SPI: SpiBus<u8>, CS: OutputPin>{
    lcd:  MemoryLCD<NiceView, SPI, CS>,
}

impl <SPI: SpiBus<u8>, CS: OutputPin> DisplayUpdater for Wrapper<SPI, CS> {
    fn update(&mut self) {
        self.lcd.update().unwrap();
    }
}

impl <SPI: SpiBus<u8>, CS: OutputPin> OriginDimensions for Wrapper<SPI, CS> {
    fn size(&self) -> Size {
        self.lcd.size()
    }
}

impl <SPI: SpiBus<u8>, CS: OutputPin> DrawTarget for Wrapper<SPI, CS> {
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

impl <SPI: SpiBus<u8>, CS: OutputPin> DisplayStyleProvider for Wrapper<SPI, CS> {
    type Color = BinaryColor;

    fn style(&self) -> Style<Self::Color> {
        DISPLAY_STYLE
    }
}
