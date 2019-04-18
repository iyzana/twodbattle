use crate::entity::{Bounds, Speed};

#[derive(Debug)]
pub enum Collision<'a, B: Bounds> {
    SIDE { x: Option<&'a B>, y: Option<&'a B> },
    CORNER { cell: &'a B },
}

pub fn check_collision<'a, E: Bounds + Speed, B: Bounds + 'a>(
    e: &E,
    cells: &'a [B],
    dt: f64,
) -> Option<Collision<'a, B>> {
    let new_x = e.x() + e.dx() * dt;
    let new_y = e.y() + e.dy() * dt;

    let moved_x = [new_x, e.y(), e.w(), e.h()];
    let collides_x = cells.iter().find(|cell| collides(&moved_x, *cell));

    let moved_y = [e.x(), new_y, e.w(), e.h()];
    let collides_y = cells.iter().find(|cell| collides(&moved_y, *cell));

    if collides_x.or(collides_y).is_some() {
        Some(Collision::SIDE {
            x: collides_x,
            y: collides_y,
        })
    } else {
        let moved_xy = [new_x, new_y, e.w(), e.h()];
        cells
            .iter()
            .find(|cell| collides(&moved_xy, *cell))
            .map(|cell| Collision::CORNER { cell })
    }
}

fn collides<A: Bounds, B: Bounds>(a: &A, b: &B) -> bool {
    a.x() < b.x() + b.w() && a.x() + a.w() > b.x() && a.y() < b.y() + b.h() && a.y() + a.h() > b.y()
}
