//! UI state unit; almost all inerfacing should be done through this "object"

#[cfg(not(feature="std"))]
mod stdwrap {
    pub use alloc::string::String;
    pub use alloc::borrow::ToOwned;
    pub use alloc::boxed::Box;
}
#[cfg(feature="std")]
mod stdwrap {
    pub use std::string::String;
    pub use std::borrow::ToOwned;
    pub use std::boxed::Box;
}

use alloc::string::ToString;
use stdwrap::*;

use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    pixelcolor::BinaryColor,
    prelude::Primitive,
    primitives::{
        Line,
        PrimitiveStyle,
        Rectangle,
    },
    Drawable,
};

use crate::{dialog::Dialog, display_def::*, probe::probe::Probe, widget::view::ViewScreen};

use crate::message;

use kampela_system::devices::power::VoltageValues;

pub struct EventResult{
    pub request: Option<UpdateRequest>,
    pub state: Option<UnitScreen>,
}

#[derive(Clone)]
pub enum UpdateRequest {
    Hidden,
    Slow,
    Fast,
    UltraFast,
    PartBlack(Rectangle),
    PartWhite(Rectangle),
}

pub trait UpdateRequestMutate {
    fn propagate(&mut self, new_request: Self);
}

impl UpdateRequestMutate for Option<UpdateRequest> {
    fn propagate(&mut self, new_request: Self) {
        if let Some(r) = new_request {
            self.replace(r);
        }
    }
}
/// State of UI
pub struct UIState<D> where
    D: DrawTarget<Color = BinaryColor>,
{
    screen: Screen,
    pub display: D,
    unlocked: bool,
}

pub enum UnitScreen {
    Probe,
    ShowMessage(String),
    ShowDialog(
        &'static str,
        (&'static str, &'static str),
        (Box<dyn FnOnce() -> EventResult>, Box<dyn FnOnce() -> EventResult>),
        bool
    ),
    Locked,
}

impl Default for UnitScreen {
    fn default() -> Self {UnitScreen::Locked}
}

/// keeps states of screens, initialization can take a lot of memory
pub enum Screen {
    Probe(Probe),
    ShowMessage(String, Option<UnitScreen>),
    ShowDialog(Dialog),
    Locked,
}

impl Screen {
    pub fn get_unit(&self) -> Option<UnitScreen> {
        match self {
            Screen::ShowMessage(s, _) => Some(UnitScreen::ShowMessage(s.to_owned())),
            Screen::Locked => Some(UnitScreen::Locked),
            _ => None,
        }
    }
}
impl Default for Screen {
    fn default() -> Self {Screen::Locked}
}

impl <D: DrawTarget<Color = BinaryColor>> UIState<D> {
    pub fn new(display: D) -> Self {
        let state = UIState {
            screen: Screen::ShowMessage("Kampela-probe".to_string(), Some(UnitScreen::Probe)),
            display,
            unlocked: true,
        };
        state
    }

    fn switch_screen(&mut self, s: Option<UnitScreen> ) {
        if let Some(s) = s {
            match s {
                UnitScreen::Probe => {
                    self.screen = Screen::Probe(Probe::new())
                },
                UnitScreen::Locked => {
                    self.screen = Screen::Locked;
                },
                UnitScreen::ShowMessage(m) => {
                    self.screen = Screen::ShowMessage(m, None);
                },
                UnitScreen::ShowDialog(message, options, routes, negative) => {
                    self.screen = Screen::ShowDialog(Dialog::new(message, options, routes, negative));
                },
            }
        }
    }

    /// Read user touch event
    pub fn handle_tap(
        &mut self,
        point: Point,
    ) -> Option<UpdateRequest> {
        let mut out = None;
        let mut new_screen = None;
        match self.screen {
            Screen::Probe(ref mut a) => {
                let (res, _) = a.handle_tap_screen(point, ());
                out = res.request;
                new_screen = res.state;
            }
            Screen::ShowDialog(ref mut a) => {
                let (res, _) = a.handle_tap_screen(point, ());
                out = res.request;
                new_screen = res.state;
            },
            _ => (),
        }
        self.switch_screen(new_screen);
        out
    }

    /// Display new screen state; should be called only when needed, is slow
    pub fn render(
        &mut self,
        is_clear_update: bool,
        voltages: &VoltageValues
    ) -> Result<Option<UpdateRequest>, <D as DrawTarget>::Error> {
        let display = &mut self.display;
        if is_clear_update {
            let clear = PrimitiveStyle::with_fill(BinaryColor::Off);
            display.bounding_box().into_styled(clear).draw(display)?;
        }
        let mut out = None;
        let mut new_screen = None;

        match self.screen {
            Screen::Probe(ref mut a) => {
                let (res, _) = a.draw_screen(display, voltages)?;
                out = res.request;
                new_screen = res.state;
            },
            Screen::Locked => {
                let linestyle = PrimitiveStyle::with_stroke(BinaryColor::On, 5);
                Line::new(
                    Point::new(0, 0),
                    Point::new(SCREEN_SIZE_X as i32, SCREEN_SIZE_Y as i32),
                )
                .into_styled(linestyle)
                .draw(display)?;
                Line::new(
                    Point::new(SCREEN_SIZE_X as i32, 0),
                    Point::new(0, SCREEN_SIZE_Y as i32),
                )
                .into_styled(linestyle)
                .draw(display)?;
            },
            Screen::ShowMessage(ref m, ref next) => {
                message::draw(display, m, true)?;
                if next.is_some() {
                    out = Some(UpdateRequest::UltraFast)
                }
                new_screen = match core::mem::replace(&mut self.screen, Screen::ShowMessage("".to_owned(), None)) {
                    Screen::ShowMessage(_, n) => n,
                    _ => None
                };
            },
            Screen::ShowDialog(ref mut a) => {
                let (res, _) = a.draw_screen(display, ())?;
                out = res.request;
                new_screen = res.state;
            }
        }
        self.switch_screen(new_screen);
        Ok(out)
    }
}


