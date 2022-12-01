#![feature(drain_filter)]

use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use utils::read_last_action;

mod utils;
mod year2021;
mod year2022;

fn main() -> std::io::Result<()> {
    let (year, day) = date_selection();
    // save to last_action.txt
    utils::save_last_action(year, day);

    match year {
        2022 => match day {
            1 => year2022::day_1::main(),
            // 1 => year2022::day_2::main(),
            // 2 => year2022::day_3::main(),
            // 3 => year2022::day_4::main(),
            _ => println!("User did not select a day"),
        },
        2021 => match day {
            1 => year2021::day_1::main(),
            2 => year2021::day_2::main(),
            3 => year2021::day_3::main(),
            4 => year2021::day_4::main(),
            _ => println!("User did not select day"),
        },
        _ => println!("User did not select a year"),
    }

    Ok(())
}

// fn last_action_selection() -> bool {
//     let term = Term::stdout();
//     let theme = ColorfulTheme::default();
//     let selection = Confirm::with_theme(&theme)
//         .with_prompt("Do you want to use the last action?")
//         .default(true)
//         .interact_on_opt(&term)
//         .unwrap()
//         .unwrap();
//     selection
// }

fn date_selection() -> (u16, u32) {
    let is_last_action_file_present = utils::is_last_action_file_present();
    match is_last_action_file_present {
        true => {
            let (year, day) = utils::read_last_action();
            let year = year_selection(year);
            let day = day_selection(day);
            (year, day)
        }
        false => {
            let year = year_selection(2022);
            let day = day_selection(1);
            (year, day)
        }
    }
}

fn year_selection(default_year: u16) -> u16 {
    const AVAILABLE_YEARS: [u16; 2] = [2022, 2021];
    // find the index of the default year
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&AVAILABLE_YEARS)
        .default(default_year as usize)
        .interact_on_opt(&Term::stderr())
        .unwrap()
        .unwrap();

    AVAILABLE_YEARS[selection]
}

fn day_selection(default_day: u32) -> u32 {
    let mut days = [1; 25];
    for (i, v) in days.iter_mut().enumerate() {
        *v = i as u32;
        *v += 1;
    }
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&days)
        .default(default_day as usize)
        .interact_on_opt(&Term::stderr())
        .unwrap()
        .unwrap();

    days[selection]
}
