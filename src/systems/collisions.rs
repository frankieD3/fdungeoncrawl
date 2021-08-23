use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // Get players position
    let mut player_pos = Point::zero();
    let mut players = <&Point>::query().filter(component::<Player>());

    players.iter(ecs).for_each(|pos| player_pos = *pos);

    // Get list of enemies
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    // remove enemies that are at the same location as the player
    enemies
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_pos)
        .for_each(|(entity, _)| {
            commands.remove(*entity);
        });
}
