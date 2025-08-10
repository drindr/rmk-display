use memory_lcd_spi::{MemoryLCD, framebuffer::{Sharp, FramebufferBW}, DisplaySpec};
use embedded_hal::{spi::SpiBus, digital::OutputPin};
use crate::DisplayController;
use crate::DisplayUpdater;

use embedded_graphics::{pixelcolor::BinaryColor};
use embedded_graphics::{draw_target::DrawTarget, geometry::{OriginDimensions, Size}};
use embedded_graphics::Pixel;

pub struct NiceView;
impl DisplaySpec for NiceView {
    const WIDTH: u16 = 160;
    const HEIGHT: u16 = 68;
    const SIZE: usize = Self::WIDTH as usize * Self::HEIGHT as usize / 2;

    type Framebuffer = FramebufferBW<{ Self::WIDTH }, { Self::HEIGHT }, {Self::SIZE}, Sharp>;
}

pub fn create_controller<SPI: SpiBus<u8>, CS: OutputPin, const PERIPHERAL_COUNT: usize>(spi_bus: SPI, cs: CS) -> DisplayController<Wrapper<SPI, CS>, PERIPHERAL_COUNT>{
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
