use crate::prelude::*;
#[system]
#[write_component(TurnState)]
pub fn main_menu_input(
    _ecs: &mut SubWorld,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        match key {
            VirtualKeyCode::P => {
                *turn_state = TurnState::StartGame;
            }
            VirtualKeyCode::Q => {
                *turn_state = TurnState::Quit;
            }
            _ => return,
        }
    }
}

pub fn pause_menu_input(
    _ecs: &mut SubWorld,
    key: &Option<VirtualKeyCode>,
    turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        match key {
            VirtualKeyCode::C => {
                *turn_state = TurnState::AwaitingInput;
            }
            VirtualKeyCode::Q => {
                *turn_state = TurnState::Quit;
            }
            _ => return,
        }
    }
}

pub fn display_main_game_menu(ctx: &mut BTerm) {
    ctx.set_active_console(MENU_LAYER);
    ctx.set_active_font(2, true);
    ctx.print_color_centered(9, GREEN, BLACK, "Press P to play");
    ctx.print_color_centered(10, GREEN, BLACK, "Press Q to leave game");
}

pub fn display_pause_game_menu(ctx: &mut BTerm) {
    ctx.set_active_console(MENU_LAYER);
    ctx.set_active_font(2, true);
    ctx.print_color_centered(9, GREEN, BLACK, "Press C to continue");
    ctx.print_color_centered(10, GREEN, BLACK, "Press Q to leave game");
}

//
// Game_over menu display
//
pub fn game_lost(ctx: &mut BTerm) {
    ctx.set_active_console(MENU_LAYER);
    ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
    ctx.print_color_centered(
        4,
        WHITE,
        BLACK,
        "Slain by a monster, your hero's journey has come to a \
        premature end.",
    );
    ctx.print_color_centered(
        5,
        WHITE,
        BLACK,
        "The Amulet of Yala remains unclaimed, and your home town \
        is not saved.",
    );
    ctx.print_color_centered(
        8,
        WHITE,
        BLACK,
        "Don't worry, you can always try again with a new hero.",
    );
}

pub fn game_victory(ctx: &mut BTerm) {
    ctx.set_active_console(MENU_LAYER);
    ctx.print_color_centered(2, GREEN, BLACK, "Your have won!");
    ctx.print_color_centered(
        4,
        WHITE,
        BLACK,
        "You put on the Amulet of Yala and feel its power course through \
        your veins",
    );
    ctx.print_color_centered(
        5,
        WHITE,
        BLACK,
        "Your town is saved and you can return to your normal life.",
    );
}
