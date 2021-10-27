use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[read_component(Player)]
#[read_component(Entity)]
#[write_component(Weapon)]
#[write_component(Health)]
#[read_component(ProvidesDungeonMap)]
pub fn use_items(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();

    <(Entity, &ActivateItem)>::query()
        .iter(ecs)
        .for_each(|(entity, activate)| {
            // entry_ref used to perform a query a single entry and the result is used to query
            // components of that entity
            let item = ecs.entry_ref(activate.item);
            if let Ok(item) = item {
                if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                    healing_to_apply.push((activate.used_by, healing.amount));
                    commands.remove(activate.item);
                    commands.remove(*entity);
                }
                if let Ok(_mapper) = item.get_component::<ProvidesDungeonMap>() {
                    map.revealed_tiles.iter_mut().for_each(|t| *t = true);
                    commands.remove(activate.item);
                    commands.remove(*entity);
                }
                if let Ok(_weapon) = item.get_component::<Weapon>() {
                    let player_entity = <(Entity, &Player)>::query()
                        .iter(ecs)
                        .find_map(|(entity1, _player)| Some(*entity1))
                        .unwrap();

                    // equip the item
                    commands.add_component(activate.item, Equipped(player_entity));
                    // remove from the carried.
                    commands.remove_component::<Carried>(activate.item);
                    // need to remove activate message but not the item
                    commands.remove_component::<ActivateItem>(*entity);
                }
            }
        });

    for heal in healing_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(heal.0) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + heal.1);
            }
        }
    }
}

pub fn use_item(
    n: usize, // item key  Warning mind indexing
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer, // Command Buffer for adding actions
) -> Point {
    // [1] Find player entity
    let player_entity = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap();

    // [2] find all items carried by the player
    let item_entity = <(Entity, &Item, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player_entity)
        .enumerate()
        .filter(|(item_count, (_, _, _))| *item_count == n) // (counter, (Entity, Item, Carrried))
        .find_map(|(_, (item_entity, _, _))| Some(*item_entity));

    if let Some(item_entity) = item_entity {
        commands.push((
            (),
            ActivateItem {
                used_by: player_entity,
                item: item_entity,
            },
        ));
    }
    Point::zero()
}
