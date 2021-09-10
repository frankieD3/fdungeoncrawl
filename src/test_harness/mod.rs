use crate::prelude::*;

pub fn display_cave(
    title: &str,
    map: &Map,
    player_start: &Point,
    amulet_start: &Point,
    monster_spawns: &[Point],
) {
    use colored::*;
    use std::io::stdin;
    const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
    let mut output = vec!['.'; NUM_TILES];

    map.tiles.iter().enumerate().for_each(|(idx, t)| match *t {
        TileType::Floor => output[idx] = '.',
        TileType::Wall => output[idx] = '#',
        TileType::Exit => output[idx] = '>',
    });

    output[map.point2d_to_index(*player_start)] = '@';
    output[map.point2d_to_index(*amulet_start)] = 'A';
    monster_spawns.iter().for_each(|p| {
        output[map.point2d_to_index(*p)] = 'M';
    });

    //print!("\x1B[2J"); // CLS!
    println!(
        "----------------------\n{}\n----------------------",
        title.bright_yellow()
    );
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            match output[map_idx(x, y)] {
                '#' => print!("{}", "#".bright_green()),
                '@' => print!("{}", "@".bright_yellow()),
                'M' => print!("{}", "M".bright_red()),
                'A' => print!("{}", "A".bright_magenta()),
                '>' => print!("{}", ">".bright_magenta()),
                _ => print!("{}", ".".truecolor(64, 64, 64)),
            }
        }
        println!("");
    }

    //let mut ignore_me = String::new();
    //stdin()
    //.read_line(&mut ignore_me)
    //.expect("Failed to read line");
}
