use std::io::Write;
use std::io::Read;
use std::fs::File;
use std::path::Path;

mod config;
mod bbio;

fn main() {
	let yaml_config = clap::load_yaml!("cli.yml");
	let matches = clap::App::from_yaml(yaml_config).get_matches();

	match matches.subcommand_name() {
		Some("config") => {
			//safe to unwrap as is only None if "config" isn't the given subcommand
			let subcommand_matches = matches.subcommand_matches("config").unwrap(); 
			if subcommand_matches.is_present("clear") {
				println!("{}", cli_config(true))
			} else {
				println!("{}", cli_config(false));
			}
		}
		Some("set_work_dir") => {
			//Safe to use unwraps because directory is a required argument
			let subcommand_matches = matches.subcommand_matches("set_work_dir").unwrap(); 
			let dir = subcommand_matches.value_of("directory").unwrap(); 
			println!("{}", cli_set_work_dir(dir));
		}
		Some("set_export_dir") => {
			//Safe to use unwraps because directory is a required argument
			let subcommand_matches = matches.subcommand_matches("set_export_dir").unwrap(); 
			let dir = subcommand_matches.value_of("directory").unwrap(); 
			println!("{}", cli_set_export_dir(dir));
		}
		Some("set_mod_dir") => {
			//Safe to use unwraps because directory is a required argument
			let subcommand_matches = matches.subcommand_matches("set_mod_dir").unwrap(); 
			let dir = subcommand_matches.value_of("directory").unwrap(); 
			println!("{}", cli_set_mod_dir(dir));
		}
		Some("set_game_dir") => {
			//Safe to use unwraps because directory is a required argument
			let subcommand_matches = matches.subcommand_matches("set_game_dir").unwrap(); 
			let dir = subcommand_matches.value_of("directory").unwrap(); 
			println!("{}", cli_set_game_dir(dir));
		}
		Some("update") => {
			
			let subcommand_matches = matches.subcommand_matches("update");
			let mut mod_dir: Option<&str> = None;
			let mut compile = false;
			let mut delete_nuts = false;

			if subcommand_matches.is_some() {
				//Safe to use unwraps because we check if args exist
				let subcommand_matches = subcommand_matches.unwrap();
				if subcommand_matches.is_present("mod") {
					mod_dir = Some(subcommand_matches.value_of("mod").unwrap());
				}
				if subcommand_matches.is_present("compile") {
					compile = true;
				}
				if subcommand_matches.is_present("remove_nuts") {
					delete_nuts = true;
				}
			}
			println!("{}", cli_update(mod_dir, compile, delete_nuts));
		}
		Some("export") => {
			let subcommand_matches = matches.subcommand_matches("export");
			let mut mod_dir: Option<&str> = None;
			let mut export_dir: Option<&str> = None;
			let mut compile = false;
			let mut delete_nuts = false;

			if subcommand_matches.is_some() {
				//Safe to use unwraps because we check if args exist
				let subcommand_matches = subcommand_matches.unwrap();
				if subcommand_matches.is_present("mod") {
					mod_dir = Some(subcommand_matches.value_of("mod").unwrap());
				}
				if subcommand_matches.is_present("export_dir") {
					export_dir = Some(subcommand_matches.value_of("export_dir").unwrap());
				}
				if subcommand_matches.is_present("compile") {
					compile = true;
				}
				if subcommand_matches.is_present("remove_nuts") {
					delete_nuts = true;
				}
			}
			println!("{}", cli_export(mod_dir, export_dir, compile, delete_nuts));
		}
		Some("import") => {
			let subcommand_matches = matches.subcommand_matches("import");
			let mut mod_dir: Option<&str> = None;
			let mut work_dir: Option<&str> = None;
			let mut delete_cnuts = true;

			if subcommand_matches.is_some() {
				let subcommand_matches = subcommand_matches.unwrap();
				if subcommand_matches.is_present("mod") {
					mod_dir = Some(subcommand_matches.value_of("mod").unwrap())
				}
				if subcommand_matches.is_present("work_dir") {
					work_dir = Some(subcommand_matches.value_of("work_dir").unwrap())
				}
				if subcommand_matches.is_present("keep_cnuts") { // Codebase needs to shift to keep rather than delete
					delete_cnuts = false;
				}
			}
			println!("{}", cli_import(mod_dir, work_dir, delete_cnuts));
		}
		Some("delete") => {}
		Some("log") => {println!("{}", cli_log());}
		Some(_) | None => {println!("Unknown command, use -h for help");}
	}
	
}


fn cli_config(clear: bool) -> String {
	if !clear {
		let config = match config::get_config() {
			Ok(config) => config,
			Err(error) => return format!("Error importing config: {:?}", error),
		};
		return config.to_string();
	} else {
		match config::set_config(config::Config::new(), true){
			Ok(()) => return "Cleared config".to_string(),
			Err(error) => return format!("Error clearing config: {:?}", error),
		}
	}
	
}

fn cli_set_work_dir(work_dir_str: &str) -> String {
	let work_dir_path = Path::new(&work_dir_str);
	if !work_dir_path.exists() { return "Directory does not exist, failed to set directory".to_string() }
	let mut new_config = config::Config::new();
	new_config.work_dir = Some(work_dir_str.to_string());
	match config::set_config(new_config, false) {
		Ok(()) => (),
		Err(error) => return format!("Error setting config: {:?}", error),
	};
	return format!("Set working directory to: {}", &work_dir_str);
}

fn cli_set_export_dir(export_dir_str: &str) -> String {
	let export_dir_path = Path::new(&export_dir_str);
	if !export_dir_path.exists() { return "Directory does not exist, failed to set directory".to_string() }
	let mut new_config = config::Config::new();
	new_config.export_dir = Some(export_dir_str.to_string());
	match config::set_config(new_config, false) {
		Ok(()) => (),
		Err(error) => return format!("Error setting config: {:?}", error),
	};
	return format!("Set Export directory to: {}", &export_dir_str);
}

fn cli_set_mod_dir(mod_dir_str: &str) -> String {
	let mod_dir_path = Path::new(&mod_dir_str);
	if !mod_dir_path.exists() { return "Directory does not exist, failed to set directory".to_string() }
	let mut new_config = config::Config::new();
	new_config.mod_dir = Some(mod_dir_str.to_string());
	match config::set_config(new_config, false) {
		Ok(()) => (),
		Err(error) => return format!("Error setting config: {:?}", error),
	};
	return format!("Set Mod directory to: {}", &mod_dir_str);
}

fn cli_set_game_dir(game_dir_str: &str) -> String {
	let game_dir_path = Path::new(&game_dir_str);
	if !game_dir_path.exists() { return "Directory does not exist, failed to set directory".to_string() }
	let mut new_config = config::Config::new();
	new_config.game_dir = Some(game_dir_str.to_string());
	match config::set_config(new_config, false) {
		Ok(()) => (),
		Err(error) => return format!("Error setting config: {:?}", error),
	};
	return format!("Set Game directory to: {}", &game_dir_str);
}



fn cli_update(mod_dir_option: Option<&str>, compile: bool, delete_nuts: bool) -> String {
	let config = match config::get_config() {
		Ok(config) => (config),
		Err(error) => return format!("Error getting config: {:?}", error),
	};

	let game_dir_path = match config.game_dir {
		Some(string) => string,
		None => return "No game directory specified in config, use subcommand set_game_dir to set mod directory".to_string(),
	};

	return cli_export(mod_dir_option, Some(game_dir_path.as_ref()), compile, delete_nuts);
} 

fn cli_export(mod_dir_option: Option<&str>, export_dir_option: Option<&str>, compile: bool, delete_nuts: bool) -> String {
	let config = match config::get_config() {
		Ok(config) => (config),
		Err(error) => return format!("Error getting config: {:?}", error),
	};

	let mod_dir_path = match mod_dir_option {
		Some(string) => Path::new(string),
		None => {
			match config.mod_dir.as_ref() {
				Some(mod_string) => Path::new(mod_string),
				None => return format!("No mod directory in config, use subcommand set_mod_dir to set mod directory or add -m argument when using export or update"),
			}
		}
	};

	let export_dir_path = match export_dir_option {
		Some(string) => Path::new(string),
		None => {
			match config.export_dir.as_ref() {
				Some(export_string) => Path::new(export_string),
				None => return format!("No export directory in config, use subcommand set_export_dir to set game directory or add -ed argument when using export"),
			}
		}
	};

	match bbio::export_mod(mod_dir_path, export_dir_path, compile, delete_nuts) {
		Ok(()) => return "Mod exported successfully".to_string(),
		Err(error) => return format!("Error exporting mod: {:?}", error),
	}
}

fn cli_import(mod_dir_option: Option<&str>, work_dir_option: Option<&str>, delete_cnuts: bool) -> String{
	let config = match config::get_config() {
		Ok(config) => (config),
		Err(error) => return format!("Error getting config: {:?}", error),
	};

	let mod_dir_path = match mod_dir_option {
		Some(string) => Path::new(string),
		None => return format!("No mod directory in config, use subcommand set_mod_dir to set mod directory or add -m argument when using export or update"),
	};

	let work_dir_path = match work_dir_option {
		Some(string) => Path::new(string),
		None => {
			match config.work_dir.as_ref() {
				Some(work_string) => Path::new(work_string),
				None => return format!("No work directory in config, use subcommand set_work_dir to set work directory or add -wd argument when using import"),
			}
		}
	};

	match bbio::import_mod(mod_dir_path, work_dir_path, delete_cnuts) {
		Ok(()) => return "Mod imported".to_string(),
		Err(error) => return format!("Error Importing mod: {:?}", error),
	}

}

fn cli_log() -> String {
	match bbio::open_log() {
		Ok(()) => return "Log opened".to_string(),
		Err(error) => return format!("Error opening log: {:?}", error)
	};
}