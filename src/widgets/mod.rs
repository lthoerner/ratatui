mod block;
mod text;
mod list;
mod gauge;
mod sparkline;
mod chart;
mod barchart;
mod tabs;
mod table;
pub mod canvas;

pub use self::block::Block;
pub use self::text::Text;
pub use self::list::List;
pub use self::gauge::Gauge;
pub use self::sparkline::Sparkline;
pub use self::chart::{Chart, Axis, Dataset, Marker};
pub use self::barchart::BarChart;
pub use self::tabs::Tabs;
pub use self::table::Table;

use buffer::Buffer;
use layout::Rect;
use terminal::Terminal;
use style::Color;

pub mod border {
    bitflags! {
        pub flags Flags: u32 {
            const NONE  = 0b00000001,
            const TOP   = 0b00000010,
            const RIGHT = 0b00000100,
            const BOTTOM = 0b0001000,
            const LEFT = 0b00010000,
            const ALL = TOP.bits | RIGHT.bits | BOTTOM.bits | LEFT.bits,
        }
    }
}

pub trait Widget {
    fn draw(&self, area: &Rect, buf: &mut Buffer);
    fn background(&self, area: &Rect, buf: &mut Buffer, color: Color) {
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                buf.set_bg(x, y, color);
            }
        }
    }
    fn render(&self, area: &Rect, t: &mut Terminal)
        where Self: Sized
    {
        t.render(self, area);
    }
}
