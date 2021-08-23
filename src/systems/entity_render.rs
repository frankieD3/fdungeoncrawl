use crate::prelude::*;

#[system] // among other things macro appends _system to function name
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    // Start new batch of entities to render on the map
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    draw_batch.submit(5000).expect("Batch error");
}
