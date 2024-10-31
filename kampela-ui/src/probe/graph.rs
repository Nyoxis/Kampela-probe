use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget, Point, Size},
    primitives::{
        Line, Primitive, PrimitiveStyle, Rectangle
    },
    Drawable
};
use kampela_system::devices::power::VoltageValues;

use crate::{display_def::*, widget::view::{DrawView, View}};
use crate::widget::view::Widget;

const MAX_VALUE: i32 = 15000;
const STEP_SCALE: i32 = 1;

pub const GRAPH_MAX_HEIGHT: i32 = 170;

pub const GRAPH_AREA: Rectangle = Rectangle{
    top_left: Point {
        x: 0,
        y: 0,
    },
    size: Size{
        width: SCREEN_SIZE_X,
        height: SCREEN_SIZE_Y,
    },
};

const GRAPH_WIDGET: Widget = Widget::new(GRAPH_AREA, SCREEN_ZERO);
pub struct Graph {
}

impl Graph {
    pub fn new(_: ()) -> Self {
        Graph {
        }
    }
}

impl View for Graph {
    type DrawInput<'a> = &'a VoltageValues;
    type DrawOutput = ();
    type TapInput<'a> = ();
    type TapOutput = ();

    fn bounding_box(&self) -> Rectangle {
        GRAPH_WIDGET.bounding_box()
    }

    fn bounding_box_absolut(&self) -> Rectangle {
        GRAPH_WIDGET.bounding_box_absolute()
    }

    fn draw_view<'a, D>(&mut self, target: &mut DrawView<D>, voltages: Self::DrawInput<'a>) -> Result<Self::DrawOutput,D::Error>
        where 
            D: DrawTarget<Color = BinaryColor>,
            Self: 'a,
        {
        
        let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

        let mut voltages_iter = voltages.iter().rev();
        if let Some(v) = voltages_iter.next() {
            let mut last_height = GRAPH_AREA.size.height as i32 - (v * GRAPH_MAX_HEIGHT) / MAX_VALUE;
            let mut offset = GRAPH_AREA.size.width as i32;

            for value in voltages_iter {
                let height = GRAPH_AREA.size.height as i32 - (*value * GRAPH_MAX_HEIGHT) / MAX_VALUE;
                let start = offset;
                let end = start - STEP_SCALE;
                Line::new(
                    Point { x: start, y: last_height },
                    Point { x: end, y: height }
                )
                    .into_styled(line_style)
                    .draw(target)?;
                offset = end;
                last_height = height;
            }
        }


        Ok(())
    }

    fn handle_tap_view<'a>(&mut self, _: Point, _: ()) -> Self::TapOutput
    where Self: 'a {
    }
}