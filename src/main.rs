mod randomizer;
mod toml_parse;
mod tui;

use randomizer::pool::Unlocks;
use randomizer::targets::Target;
use toml_parse::savefile::Savefile;

fn main() {
    let sf2: Savefile = match Savefile::read_from_file("sf.toml".to_string()) {
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
        Ok(s) => s,
    };
    let unlocks: Unlocks = match sf2.try_into() {
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
        Ok(u) => u,
    };

    match unlocks.get_random_pick() {
        Some((ch, targs_set)) => {
            print!("{}\n\nVS\n\n", ch);
            let mut targs: Vec<&Target> = targs_set.iter().collect();
            targs.sort();
            for targ in targs {
                println!("{}", targ);
            }
        }
        None => {
            println!("couldn't roll :(");
        }
    }
}
