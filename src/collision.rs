use crate::entity::{Bounds, Speed};

#[derive(Debug)]
pub enum Collision<'a, E: Bounds> {
    SIDE { x: Option<&'a E>, y: Option<&'a E> },
    CORNER,
}

pub fn check_collision<'a, 'b: 'a, E: Bounds + Speed, F: Bounds + 'a>(
    e: &E,
    cells: &'a [F],
    dt: f64,
) -> Option<Collision<'a, F>> {
    let new_x = e.x() + e.dx() * dt;
    let new_y = e.y() + e.dy() * dt;
    let moved_x = [new_x, e.y(), e.w(), e.h()];
    let moved_y = [e.x(), new_y, e.w(), e.h()];

    let mut collides_x = None;
    let mut collides_y = None;

    if let Some(cell) = cells.iter().find(|cell| collides(moved_x, cell.bounds())) {
        collides_x = Some(cell);
    }
    if let Some(cell) = cells.iter().find(|cell| collides(moved_y, cell.bounds())) {
        collides_y = Some(cell);
    }

    if collides_x.is_some() || collides_y.is_some() {
        return Some(Collision::SIDE {
            x: collides_x,
            y: collides_y,
        });
    } else {
        let moved_xy = [new_x, new_y, e.w(), e.h()];

        if cells.iter().any(|cell| collides(moved_xy, cell.bounds())) {
            return Some(Collision::CORNER);
        }
    }

    None
}

fn collides(a: [f64; 4], b: [f64; 4]) -> bool {
    a[0] < b[0] + b[2] && a[0] + a[2] > b[0] && a[1] < b[1] + b[3] && a[1] + a[3] > b[1]
}
