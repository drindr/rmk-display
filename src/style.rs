use kolibri_embedded_gui::style::{Style, Spacing};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::mono_font;
use embedded_graphics::geometry::Size;

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
