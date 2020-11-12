use druid::{Color, Env, Key, Point};

pub const BUTTON_ICON_PADDING: Key<f64> = Key::new("ink.scribl.widget.button-icon-padding");
pub const BUTTON_ICON_DISABLED_COLOR: Key<Color> =
    Key::new("ink.scribl.widget.button-icon-disabled-color");
pub const BUTTON_ICON_SELECTED_COLOR: Key<Color> =
    Key::new("ink.scribl.widget.button-icon-selected-color");
pub const BUTTON_ICON_COLOR: Key<Color> = Key::new("ink.scribl.widget.button-icon-color");
pub const BUTTON_ICON_BUTTON_COLOR: Key<Color> =
    Key::new("ink.scribl.widget.button-icon-button-color");
pub const BUTTON_ICON_HOT_STROKE_THICKNESS: Key<f64> =
    Key::new("ink.scribl.widget.button-icon-hot-stroke-thickness");

pub const DROP_SHADOW_RADIUS: Key<f64> = Key::new("ink.scribl.widget.drop-shadow-radius");
pub const DROP_SHADOW_OFFSET: Key<Point> = Key::new("ink.scribl.widget.drop-shadow-offset");
pub const DROP_SHADOW_COLOR: Key<Color> = Key::new("ink.scribl.widget.drop-shadow-color");

// These colors are lightened versions of the utexas secondary color palette. We use them
// for coloring the UI elements.
pub const UI_LIGHT_YELLOW: Color = Color::rgb8(255, 239, 153);
pub const UI_LIGHT_GREEN: Color = Color::rgb8(211, 230, 172);
pub const UI_DARK_GREEN: Color = Color::rgb8(87, 157, 66);
pub const UI_LIGHT_BLUE: Color = Color::rgb8(87, 242, 255);
pub const UI_DARK_BLUE: Color = Color::rgb8(0, 95, 134);
pub const UI_BEIGE: Color = Color::rgb8(214, 210, 196);
pub const UI_LIGHT_STEEL_BLUE: Color = Color::rgb8(156, 173, 183);

pub fn configure_env(e: &mut Env) {
    e.set(BUTTON_ICON_PADDING, 4.0);
    e.set(BUTTON_ICON_DISABLED_COLOR, Color::rgb8(0x70, 0x70, 0x70));
    e.set(BUTTON_ICON_SELECTED_COLOR, UI_DARK_GREEN);
    e.set(BUTTON_ICON_COLOR, Color::rgb8(0x70, 0x70, 0x70));
    e.set(BUTTON_ICON_BUTTON_COLOR, Color::rgb8(0xA0, 0xA0, 0xA0));
    e.set(BUTTON_ICON_HOT_STROKE_THICKNESS, 4.0);

    e.set(DROP_SHADOW_RADIUS, 6.0);
    e.set(DROP_SHADOW_OFFSET, Point::new(2.0, 2.0));
    e.set(DROP_SHADOW_COLOR, Color::rgb8(0x10, 0x10, 0x10));
}

/// An icon made up of a single path (which should be filled with whatever color we want).
pub struct Icon {
    /// The width of the icon.
    pub width: u32,
    /// The height of the icon.
    pub height: u32,
    /// The icon's path, in SVG format.
    pub path: &'static str,
}

mod modal;
mod radio;
mod separator;
mod sunken_container;
pub(crate) mod toggle_button;

pub use modal::{ModalHost, TooltipExt, TooltipHost};
pub use radio::RadioGroup;
pub use separator::Separator;
pub use sunken_container::SunkenContainer;
pub use toggle_button::{ToggleButton, ToggleButtonState};
