extern crate clap;
use clap::{Arg, App, SubCommand, ArgMatches};

fn main() {
    let date_arg = Arg::with_name("date")
        .short("d")
        .long("date")
        .value_name("DATE")
        .help("The date that the standup happens on");
    let message_arg = Arg::with_name("message")
        .value_name("MESSAGE")
        .required(true)
        .help("The message to add to the stand up");

    let matches = App::new("standup")
        .version("0.0.1")
        .author("Kevin Bacha <chewbacha@gmail.com>")
        .about("Manages stand up entries and keeps log")
        .subcommand(SubCommand::with_name("today")
                        .about("Manages what you will be working on")
                        .alias("t")
                        .arg(date_arg.clone())
                        .arg(message_arg.clone())
                        )
        .subcommand(SubCommand::with_name("yesterday")
                        .about("Manages what you worked on the day before")
                        .alias("y")
                        .arg(date_arg.clone())
                        .arg(message_arg.clone())
                        )
        .subcommand(SubCommand::with_name("blocked")
                        .about("Manages what is blocking you")
                        .alias("b")
                        .arg(date_arg.clone())
                        .arg(message_arg.clone())
                        )
        .subcommand(SubCommand::with_name("show")
                        .about("Displays the notes from stand up")
                        .alias("s")
                        .arg(date_arg.clone())
                        .arg(message_arg.clone())
                        )
        .get_matches();
    match matches.subcommand() {
        ("today",       Some(sub_args)) => handle_today(sub_args),
        ("yesterday",   Some(sub_args)) => handle_yesterday(sub_args),
        ("blocked",     Some(sub_args)) => handle_blocked(sub_args),
        ("show",        Some(sub_args)) => handle_show(sub_args),
        _ => {},
    }
}

fn handle_today(args: &ArgMatches) {
    println!("today!");
    println!("date: {}", args.value_of("date").unwrap_or(""));
}

fn handle_yesterday(args: &ArgMatches) {
    println!("yesterday!");
    println!("date: {}", args.value_of("date").unwrap_or(""));
}

fn handle_blocked(args: &ArgMatches) {
    println!("blocked!");
    println!("date: {}", args.value_of("date").unwrap_or(""));
}

fn handle_show(args: &ArgMatches) {
    println!("showing!");
    println!("date: {}", args.value_of("date").unwrap_or(""));
}
