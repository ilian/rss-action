mod lastseen;

use rss;

use std::fs;
use std::process;
use std::io::{BufReader, BufRead};
use std::iter::Iterator;
use self::lastseen::LastSeen;
use clap::{Arg, App};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("rss-action")
                    .version("0.1.0")
                    .author("nocent <nocent@protonmail.ch>")
                    .about("Perform an action for each unprocessed RSS item link")
                    .arg(Arg::with_name("feeds")
                        .short("f")
                        .long("feeds-path")
                        .takes_value(true)
                        .default_value("feeds.txt")
                        .help("Set path to feeds list containing RSS URIs seperated by new lines"))
                    .arg(Arg::with_name("state")
                        .short("s")
                        .long("state-path")
                        .takes_value(true)
                        .default_value("rss-action.dat")
                        .help("Set path to file containing already processed items"))
                    .arg(Arg::with_name("replace")
                        .short("r")
                        .long("replacement-string")
                        .takes_value(true)
                        .help("Replace all occurences of the specified string in the arguments of <command> with each RSS item link"))
                    .arg(Arg::with_name("ignore-failure")
                        .short("i")
                        .long("ignore-failure")
                        .help("Do not abort when the specified command terminated with a non-zero exit code"))
                    .arg(Arg::with_name("command")
                        .required(true)
                        .multiple(true)
                        .last(true)
                        .help("The command to execute. The last argument will be the RSS item link unless overriden with an option"))
                    .get_matches();
    let feeds_path = matches.value_of("feeds").ok_or("Invalid argument")?;
    let state_path = matches.value_of("state").ok_or("Invalid argument")?;
    // List of arguments without substituting or appending RSS item links
    let command_template: Vec<&str> = matches.values_of("command").unwrap().collect();

    let feeds_file = fs::File::open(feeds_path)?;
    let mut ls = LastSeen::new(state_path)?;
    let feeds_reader = BufReader::new(feeds_file);

    for line in feeds_reader.lines() {
        let feed_url = line?;
        // Ignore newlines in feeds list
        if feed_url.is_empty() {
            continue;
        }
        let channel = rss::Channel::from_url(&feed_url)?;
        let mut links = channel.items().iter()
                                   .filter_map(|x| x.link())
                                   .collect::<Vec<&str>>();
        links.reverse(); // list from old to new
        
        let links_to_visit = ls.get_unvisited(&feed_url, &links);
        for link in links_to_visit {
            let mut action_cmd = process::Command::new(&command_template[0]);

            let mut args: Vec<String> = command_template[1..].iter().map(|s| s.to_string()).collect();
            if matches.is_present("replace") {
                let replace_string = matches.value_of("replace").ok_or("Invalid argument")?;
                for arg in args.iter_mut() {
                    *arg = arg.replace(replace_string, link)
                }
            } else {
                args.push(link.to_string());
            }

            action_cmd.args(&args);
            let exit_status = action_cmd.spawn()?.wait()?;
            if exit_status.success() {
                // Record rss link as completed
                ls.set_last_seen(&feed_url, &link)?;
            } else {
                eprintln!("Command failed to execute!");
                if !matches.is_present("ignore-failure") {
                    // Pass status code of failed command or 1 in case of signal
                    process::exit(exit_status.code().unwrap_or(1));
                }
            }
        }
    }
    Ok(())
}
