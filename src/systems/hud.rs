use crate::prelude::*;

const INVENTORY_HEADER_SIZE: i32 = 2;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Equipped)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld) {
    // draw to the HUD console layer
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(HUD_LAYER);

    // [1] Query and print the health of the player
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).next().unwrap();

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

    // [2] Print level #
    let (_player, map_level) = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, player)| Some((*entity, player.map_level)))
        .unwrap();

    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH, 1),
        format!("Dungeon level: {}", map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );
    // [3] Print Inventory
    //
    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap();

    // print Inventory
    let mut item_query = <(&Item, &Name, &Carried)>::query();
    let mut num_carried = 0;
    draw_batch.print_color(
        Point::new(3, INVENTORY_HEADER_SIZE),
        "Inventory",
        ColorPair::new(YELLOW, BLACK),
    );
    item_query
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .for_each(|(_, name, _)| {
            num_carried += 1;
            draw_batch.print(
                Point::new(3, num_carried + 2),
                format!("{} : {}", num_carried, &name.0),
            );
        });
    let mut num_equipped = 0;
    draw_batch.print_color(
        Point::new(3, num_carried + INVENTORY_HEADER_SIZE * 2),
        "Equipped",
        ColorPair::new(YELLOW, BLACK),
    );
    // print Equipped
    let mut equipped_query = <(&Item, &Name, &Equipped)>::query();
    equipped_query
        .iter(ecs)
        .filter(|(_, _, equipped)| equipped.0 == player)
        .for_each(|(_, name, _)| {
            num_equipped += 1;
            draw_batch.print(
                Point::new(3, num_equipped + num_carried + 4),
                format!("{} : {}", num_equipped, &name.0),
            );
        });
    // Draw hud with a Z buffer order of 10000
    draw_batch.submit(10000).expect("Batch error");
}
