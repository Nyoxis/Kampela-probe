use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget, Point, Size},
    primitives::Rectangle,
    Drawable, Pixel
};

use crate::{display_def::*, widget::view::{DrawView, View}};
use crate::widget::view::Widget;

use crate::probe::graph::{GRAPH_AREA, GRAPH_MAX_HEIGHT};
use crate::probe::labels::{Label, LABEL_FONT};

pub const LABEL_SIZE: Size = Size{
    width: LABEL_FONT.character_size.width * 3,
    height: LABEL_FONT.character_size.height,
};

pub const V5_AREA: Rectangle = Rectangle{
    top_left: Point {
        x: 0,
        y: GRAPH_AREA.size.height as i32 - (GRAPH_MAX_HEIGHT / 3) - LABEL_SIZE.height as i32 / 2,
    },
    size: LABEL_SIZE
};
const V5_WIDGET: Widget = Widget::new(V5_AREA, GRAPH_AREA.top_left);

pub const V10_AREA: Rectangle = Rectangle{
    top_left: Point {
        x: 0,
        y: GRAPH_AREA.size.height as i32 - (GRAPH_MAX_HEIGHT / 3 * 2) - LABEL_SIZE.height as i32 / 2,
    },
    size: LABEL_SIZE
};
const V10_WIDGET: Widget = Widget::new(V10_AREA, GRAPH_AREA.top_left);

pub const V15_AREA: Rectangle = Rectangle{
    top_left: Point {
        x: 0,
        y: GRAPH_AREA.size.height as i32 - (GRAPH_MAX_HEIGHT) - LABEL_SIZE.height as i32 / 2,
    },
    size: LABEL_SIZE
};
const V15_WIDGET: Widget = Widget::new(V15_AREA, GRAPH_AREA.top_left);

const COORDINATES_WIDGET: Widget = Widget::new(GRAPH_AREA, SCREEN_ZERO);
pub struct Coordinates {
    labels: [Label; 3]
}

impl Coordinates {
    pub fn new(_: ()) -> Self {
        Coordinates {
            labels: [
                Label::new(&V5_WIDGET, " 5V"),
                Label::new(&V10_WIDGET, "10V"),
                Label::new(&V15_WIDGET, "15V"),
            ]
        }
    }
}

impl View for Coordinates {
    type DrawInput<'a> = ();
    type DrawOutput = ();
    type TapInput<'a> = ();
    type TapOutput = ();

    fn bounding_box(&self) -> Rectangle {
        COORDINATES_WIDGET.bounding_box()
    }

    fn bounding_box_absolut(&self) -> Rectangle {
        COORDINATES_WIDGET.bounding_box_absolute()
    }

    fn draw_view<'a, D>(&mut self, target: &mut DrawView<D>, _: Self::DrawInput<'a>) -> Result<Self::DrawOutput,D::Error>
        where 
            D: DrawTarget<Color = BinaryColor>,
            Self: 'a,
        {
        for label in self.labels.iter_mut() {
            for x_dot in 0..(GRAPH_AREA.size.width / 4) {
                Pixel::<BinaryColor>(
                    Point::new(
                        x_dot as i32 * 4,
                        label.bounding_box().top_left.y + LABEL_SIZE.height as i32 / 2
                    ),
                    BinaryColor::On
                ).draw(target)?;
            }
            label.draw(target, ())?;
        }

        Ok(())
    }

    fn handle_tap_view<'a>(&mut self, _: Point, _: ()) -> Self::TapOutput
    where Self: 'a {
    }
}