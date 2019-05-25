use crate::entity::{Bounds, Speed};

#[derive(Debug)]
pub enum Collision<'a, B: Bounds> {
    SIDE { x: Option<&'a B>, y: Option<&'a B> },
    CORNER { cell: &'a B },
}

pub fn check<'a, E: Bounds + Speed, B: Bounds + 'a>(
    entity: &E,
    obstacles: &'a [B],
    dt: f64,
) -> Option<Collision<'a, B>> {
    let new_x = entity.x() + entity.dx() * dt;
    let new_y = entity.y() + entity.dy() * dt;

    let moved_x = [new_x, entity.y(), entity.w(), entity.h()];
    let collides_x = obstacles.iter().find(|&cell| collides(&moved_x, cell));

    let moved_y = [entity.x(), new_y, entity.w(), entity.h()];
    let collides_y = obstacles.iter().find(|&cell| collides(&moved_y, cell));

    if collides_x.or(collides_y).is_some() {
        Some(Collision::SIDE {
            x: collides_x,
            y: collides_y,
        })
    } else {
        let moved_xy = [new_x, new_y, entity.w(), entity.h()];
        obstacles
            .iter()
            .find(|&cell| collides(&moved_xy, cell))
            .map(|cell| Collision::CORNER { cell })
    }
}

pub fn collides<A: Bounds, B: Bounds>(a: &A, b: &B) -> bool {
    a.x() < b.x() + b.w() && a.x() + a.w() > b.x() && a.y() < b.y() + b.h() && a.y() + a.h() > b.y()
}
