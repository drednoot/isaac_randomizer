use crate::randomizer::pool::Unlocks;
use crate::randomizer::targets::Target;
use crate::toml_parse::savefile::Savefile;
use crate::toml_parse::savefile::Error;

pub fn no_args() {
    let mut save_required = false;
    let unlocks: Unlocks = match Savefile::read_from_file("sf.toml".to_string()) {
        Err(Error::IoError(ref e)) if e.kind() == std::io::ErrorKind::NotFound => {
            save_required = true;
            Unlocks::default()
        }
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
        Ok(s) => {
            match s.try_into() {
                Err(e) => {
                    eprintln!("{:?}", e);
                    return;
                }
                Ok(u) => u,
            }
        }
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
            println!("couldn't roll ):");
        }
    }

    if save_required {
        let savefile: Savefile = Into::into(unlocks);
        match savefile.write_to_file("sf.toml".to_string()) {
            Err(e) => {
                eprintln!("{:?}", e);
                return;
            }
            Ok(_) => {},
        }
    }
}
