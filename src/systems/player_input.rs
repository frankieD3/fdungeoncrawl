use crate::prelude::*;
use crate::systems::menu;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Equipped)]
#[read_component(Weapon)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    match *turn_state {
        TurnState::Pause => menu::pause_menu_input(ecs, key, turn_state),
        _ => {
            let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

            if let Some(key) = key {
                let mut did_something = false;

                let delta = match key {
                    // Movement keys
                    VirtualKeyCode::Left => Point::new(-1, 0),
                    VirtualKeyCode::Right => Point::new(1, 0),
                    VirtualKeyCode::Up => Point::new(0, -1),
                    VirtualKeyCode::Down => Point::new(0, 1),
                    // Pause key
                    VirtualKeyCode::Escape => {
                        *turn_state = TurnState::Pause;
                        Point::zero()
                    }
                    // Pick up item
                    VirtualKeyCode::G => {
                        //Grab item
                        let (player, player_pos) = players
                            .iter(ecs)
                            .find_map(|(entity, pos)| Some((*entity, *pos)))
                            .unwrap();
                        let mut items = <(Entity, &Item, &Point)>::query();
                        items
                            .iter(ecs)
                            .filter(|(_enity, _item, &item_pos)| item_pos == player_pos)
                            .for_each(|(entity, _item, _item_pos)| {
                                commands.remove_component::<Point>(*entity);
                                commands.add_component(*entity, Carried(player));
                            });

                        Point::new(0, 0)
                    }
                    // Inventory keys
                    VirtualKeyCode::Key1 => use_items::use_item(0, ecs, commands),
                    VirtualKeyCode::Key2 => use_items::use_item(1, ecs, commands),
                    VirtualKeyCode::Key3 => use_items::use_item(2, ecs, commands),
                    VirtualKeyCode::Key4 => use_items::use_item(3, ecs, commands),
                    VirtualKeyCode::Key5 => use_items::use_item(4, ecs, commands),
                    VirtualKeyCode::Key6 => use_items::use_item(5, ecs, commands),
                    VirtualKeyCode::Key7 => use_items::use_item(6, ecs, commands),
                    VirtualKeyCode::Key8 => use_items::use_item(7, ecs, commands),
                    _ => Point::new(0, 0),
                };

                if *turn_state != TurnState::Pause {
                    // find players location and proposed destination
                    let (player_entity, destination) = players
                        .iter(ecs)
                        .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
                        .unwrap();

                    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
                    if delta.x != 0 || delta.y != 0 {
                        let mut hit_something = false;
                        enemies
                            .iter(ecs)
                            // find any enemies at the proposed loccation
                            .filter(|(_, pos)| **pos == destination)
                            .for_each(|(entity, _)| {
                                // post attack command at list of enemies
                                hit_something = true;
                                did_something = true;
                                commands.push((
                                    (),
                                    WantsToAttack {
                                        attacker: player_entity,
                                        victim: *entity,
                                    },
                                ));
                            });

                        if !hit_something {
                            commands.push((
                                (),
                                WantsToMove {
                                    entity: player_entity,
                                    destination,
                                },
                            ));
                        }
                    }
                    *turn_state = TurnState::PlayerTurn;
                }
            }
        }
    }
}
