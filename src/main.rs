#![feature(drain_filter)]
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

mod utils;
mod year2021;
mod year2022;

fn main() -> std::io::Result<()> {
    // read arguments
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    // if argument 'setup' is passed, run setup
    if args.contains(&"--setup".to_owned()) {
        utils::setup();
        return Ok(());
    }
    let (year, day) = date_selection();
    // save to last_action.txt
    utils::save_last_action(year, day);

    match year {
        2022 => match day {
            1 => year2022::day_1::main(),
            2 => year2022::day_2::main(),
            3 => year2022::day_3::main(),
            4 => year2022::day_4::main(),
            5 => year2022::day_5::main(),
            6 => year2022::day_6::main(),
            // 7 => year2022::day_7::main(),
            // 8 => year2022::day_8::main(),
            // 9 => year2022::day_9::main(),
            // 10 => year2022::day_10::main(),
            // 11 => year2022::day_11::main(),
            // 12 => year2022::day_12::main(),
            // 13 => year2022::day_13::main(),
            // 14 => year2022::day_14::main(),
            // 15 => year2022::day_15::main(),
            // 16 => year2022::day_16::main(),
            // 17 => year2022::day_17::main(),
            // 18 => year2022::day_18::main(),
            // 19 => year2022::day_19::main(),
            // 20 => year2022::day_20::main(),
            // 21 => year2022::day_21::main(),
            // 22 => year2022::day_22::main(),
            // 23 => year2022::day_23::main(),
            // 24 => year2022::day_24::main(),
            // 25 => year2022::day_25::main(),
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

fn date_selection() -> (u16, u32) {
    let is_last_action_file_present = utils::is_last_action_file_present();
    match is_last_action_file_present {
        true => {
            println!("Last action file present");
            let (year, day) = utils::read_last_action();
            println!("Last action: {} {}", year, day);
            let year = year_selection(year);
            let day = day_selection(day);
            (year, day)
        }
        false => {
            println!("Last action file not present");
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
        .default(
            AVAILABLE_YEARS
                .iter()
                .position(|&r| r == default_year)
                .unwrap(),
        )
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
        .default((default_day - 1) as usize)
        .interact_on_opt(&Term::stderr())
        .unwrap()
        .unwrap();

    days[selection]
}
