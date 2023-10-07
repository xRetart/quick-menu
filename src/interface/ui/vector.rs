use ratatui::prelude::Rect;

#[derive(Clone, Copy)]
pub struct Vector {
    pub x: u16,
    pub y: u16,
}
impl Vector {
    pub fn center_in(self, outer: Self) -> Rect {
        let width = self.x.min(outer.x);
        let height = self.y.min(outer.y);

        let x = (outer.x - width) / 2;
        let y = (outer.y - height) / 2;

        Rect { x, y, width, height }
    }
}
