use embedded_graphics::{
    prelude::{Point, DrawTarget},
    primitives::{Primitive, PrimitiveStyle},
    Drawable,
    pixelcolor::BinaryColor,
};

use crate::display_def::*;
use crate::uistate::{EventResult, UpdateRequest};
use crate::widget::view::{View, ViewScreen};
use crate::probe::{coordinates::Coordinates, graph::Graph};

use kampela_system::devices::power::VoltageValues;

pub struct Probe {
    graph: Graph,
    coordinates: Coordinates,
}

impl Probe {
    pub fn new(
    ) -> Self {
        Probe{
            graph: Graph::new(()),
            coordinates: Coordinates::new(()),
        }
    }
}

impl ViewScreen for Probe {
    type DrawInput<'a> = &'a VoltageValues;
    type DrawOutput = ();
    type TapInput<'a> = ();
    type TapOutput = ();
    
    fn draw_screen<'a, D>(&mut self, target: &mut D, voltages: Self::DrawInput<'a>) -> Result<(EventResult, Self::DrawOutput), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
        Self: 'a,
    {
        let request = Some(UpdateRequest::UltraFastSelective);
        let state = None;

        let filled = PrimitiveStyle::with_fill(BinaryColor::Off);

        SCREEN_AREA.into_styled(filled).draw(target)?;
        self.coordinates.draw(target, ())?;
        self.graph.draw(target, voltages)?;

        Ok((EventResult { request, state }, ()))
    }
    fn handle_tap_screen<'a>(&mut self, _: Point, _: ()) -> (EventResult, ())
    where
        Self: 'a
    {
        let event_result = EventResult{request: None, state: None};

        (event_result, ())
    }
}