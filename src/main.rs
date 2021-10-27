mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod test_harness;
mod turn_state;

mod prelude {
    // outside crates
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::Schedule;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const NUM_LEVELS: u32 = 1;
    pub const NUM_MONSTERS: usize = 50;
    // Drawing lauyers
    pub const MAP_LAYER: usize = 0;
    pub const PLAY_LAYER: usize = 1;
    pub const HUD_LAYER: usize = 2;
    pub const MENU_LAYER: usize = 3;
    //
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::test_harness::*;
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    //
    // Create minimally populated State info to start with displaying the Main Menu of game
    //
    fn new() -> Self {
        let ecs = World::default();
        let mut resources = Resources::default();
        let map_builder = MapBuilder::default();
        resources.insert(TurnState::MainMenu);
        resources.insert(map_builder.map);
        resources.insert(map_builder.theme);
        resources.insert(Camera::new(map_builder.player_start));

        Self {
            ecs,
            resources,
            input_systems: build_base_input_scheduler(),
            player_systems: build_base_player_scheduler(),
            monster_systems: build_base_monster_scheduler(),
        }
    }

    //
    // generate a fresh game
    //
    fn generate_new_game(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut self.ecs, map_builder.player_start);
        //spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;

        // # of entities spawned equal to the size of NUM_Monsters in the map_builder code
        spawn_level(&mut self.ecs, &mut rng, 0, &map_builder.monster_spawns);
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
        self.input_systems = build_input_scheduler();
        self.player_systems = build_player_scheduler();
        self.monster_systems = build_monster_scheduler();
    }

    //
    // generate next level
    //
    fn generate_next_level(&mut self) {
        //find player
        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&mut self.ecs)
            .next()
            .unwrap();
        use std::collections::HashSet;
        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);

        // find all carried items in inventory for player
        // add them to a hash of what to keep
        <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_e, carry)| carry.0 == player_entity)
            .map(|(e, _carry)| *e)
            .for_each(|e| {
                entities_to_keep.insert(e);
            });
        // find all equipped items in inventory for player
        // add them to a hash of what to keep
        <(Entity, &Equipped)>::query()
            .iter(&self.ecs)
            .filter(|(_e, equipped)| equipped.0 == player_entity)
            .map(|(e, _equipped)| *e)
            .for_each(|e| {
                entities_to_keep.insert(e);
            });

        // delete all other entities in current level
        let mut cb = CommandBuffer::new(&mut self.ecs);
        for e in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(e) {
                cb.remove(*e);
            }
        }
        // added resources arg for legion 4.0
        //cb.flush(&mut self.ecs);
        cb.flush(&mut self.ecs, &mut self.resources);

        // Set players field of vieww to dirty
        <&mut FieldOfView>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| fov.is_dirty = true);

        // Generate new level map
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.x = map_builder.player_start.x;
                pos.y = map_builder.player_start.y;
            });

        // spawn the amulet only on the last Level else spawn an exit to the next level
        if map_level == NUM_LEVELS {
            spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }

        // # of entities spawned equal to the size of NUM_Monsters in the map_builder code
        spawn_level(
            &mut self.ecs,
            &mut rng,
            map_level as usize,
            &map_builder.monster_spawns,
        );
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // clear layers
        ctx.set_active_console(MAP_LAYER);
        ctx.cls();
        ctx.set_active_console(PLAY_LAYER);
        ctx.cls();
        ctx.set_active_console(HUD_LAYER);
        ctx.cls();
        ctx.set_active_console(MENU_LAYER);
        ctx.cls();

        // When you insert ctx.key it replaces the existing key resource
        self.resources.insert(ctx.key);
        // Get mouse position from the map(terminal) layer
        // used for tooltips
        ctx.set_active_console(MAP_LAYER);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        // Execute Systems
        let current_state = self.resources.get::<TurnState>().unwrap().clone();

        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::Pause => {
                menu::display_pause_game_menu(ctx);
                self.input_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::NextLevel => self.generate_next_level(),
            TurnState::GameOver => {
                self.input_systems = build_base_input_scheduler();
                self.player_systems = build_base_player_scheduler();
                self.player_systems = build_base_monster_scheduler();
                menu::game_lost(ctx);
                menu::display_main_game_menu(ctx);
                self.input_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::Victory => {
                self.input_systems = build_base_input_scheduler();
                self.player_systems = build_base_player_scheduler();
                self.player_systems = build_base_monster_scheduler();
                menu::game_victory(ctx);
                menu::display_main_game_menu(ctx);
                self.input_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::MainMenu => {
                menu::display_main_game_menu(ctx);
                self.input_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::StartGame => self.generate_new_game(),
            TurnState::Quit => ctx.quitting = true, // from Bterm::main_loop variable
        }

        // Render Draw Buffer
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let mut context = BTermBuilder::new()
        .with_title("FDungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_font("unicode_16x16.png", 16, 16)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;

    //

    let gs = State::new();
    // initialize font for the HUD_LAYER
    context.set_active_console(HUD_LAYER);
    context.set_active_font(2, true);
    main_loop(context, gs)
}
