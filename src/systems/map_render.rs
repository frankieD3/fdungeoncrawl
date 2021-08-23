use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();

    // Start a new batch of drawings
    draw_batch.target(0);
    for y in camera.top_y..camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.in_bounds(pt) {
                let idx = map_idx(x, y);
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                    TileType::TunnelFloor => to_cp437(';'),
                };

                // add current tile to batch queue
                draw_batch.set(pt - offset, ColorPair::new(WHITE, BLACK), glyph);
            }
        }
    }

    // Submit the current queue to be rendered first in the render cycle.
    draw_batch.submit(0).expect("Batch error");
}
