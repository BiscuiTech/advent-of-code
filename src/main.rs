#![feature(drain_filter)]

use console::Term;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use utils::delete_last_action;

mod utils;
mod year2021;
mod year2022;

fn main() -> std::io::Result<()> {
    let is_last_action_file_present = utils::is_last_action_file_present();

    let mut selected_year: u16 = 0;
    let mut selected_day: u32 = 0;
    let mut should_continue_last_action: bool = false;
    // if is_last_action_file_present is true
    if is_last_action_file_present {
        should_continue_last_action = last_action_selection();
    }
    if should_continue_last_action {
        let (year, day) = utils::read_last_action();
        selected_year = year;
        selected_day = day;
    } else {
        if is_last_action_file_present {
            delete_last_action();
        }
        selected_year = year_selection();
        selected_day = day_selection();
    }
    // save to last_action.txt
    utils::save_last_action(selected_year, selected_day);

    match selected_year {
        2022 => match selected_day {
            1 => year2022::day_1::main(),
            // 1 => year2022::day_2::main(),
            // 2 => year2022::day_3::main(),
            // 3 => year2022::day_4::main(),
            _ => println!("User did not select a day"),
        },
        2021 => match selected_day {
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

fn last_action_selection() -> bool {
    let term = Term::stdout();
    let theme = ColorfulTheme::default();
    let selection = Confirm::with_theme(&theme)
        .with_prompt("Do you want to use the last action?")
        .default(true)
        .interact_on_opt(&term)
        .unwrap()
        .unwrap();
    selection
}

fn year_selection() -> u16 {
    const AVAILABLE_YEARS: [u16; 2] = [2022, 2021];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&AVAILABLE_YEARS)
        .default(0) // default to 2022
        .interact_on_opt(&Term::stderr())
        .unwrap()
        .unwrap();

    AVAILABLE_YEARS[selection]
}

fn day_selection() -> u32 {
    let mut days = [1; 25];
    for (i, v) in days.iter_mut().enumerate() {
        *v = i as u32;
        *v += 1;
    }
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&days)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap()
        .unwrap();

    days[selection]
}
