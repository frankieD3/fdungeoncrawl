use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    // querey the health of the player
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).nth(0).unwrap();

    // draw to the HUD console layer
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");
    // bar_horizontal help method to draw health bars.
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );

    draw_batch.print_color_centered(
        0,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, RED),
    );
    // Draw hud with a Z buffer order of 10000
    draw_batch.submit(10000).expect("Batch error");
}
