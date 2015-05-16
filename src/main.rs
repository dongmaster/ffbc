#![feature(path_ext)] 
#![feature(fs_walk)]
#![feature(convert)]

extern crate rustc_serialize;

use std::path::Path;
use std::fs::PathExt;
use std::env;
use std::fs;
use std::process;

use std::fs::File;
use std::io::Write;
use std::io::Read;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
    config: Vec<String>,
}

const CONFIG : &'static str = ".ffbc-config.json";

fn main() {
    let args: Vec<String> = env::args().collect();
    
    first_run();
    handle_arguments(args);
}

fn first_run() {
    let config_file = Path::new("./").join(CONFIG);
    
    if config_file.exists() == false {
        let default_config = Config {
            config: vec!("".to_string()),
        };
        
        save_config(default_config);
    }
}

fn handle_arguments(arguments: Vec<String>) {
    match arguments[1].as_ref() {
        "-c" | "--config"   => { change_config(&arguments); return;},
        "-h" | "--help"     => help(),
        _                   => (),
    }
    
    let mut args: Vec<&String> = Vec::new();
    let real_arguments = &arguments[1..arguments.len()];
    
    for arg in real_arguments {
        args.push(arg);
    }
    
    let mut dirs_and_exts: Vec<&String> = Vec::new();
    
    for x in 0..args.len() {
        if x % 2 == 0 {
            dirs_and_exts.push(args[x]);
        } else if x % 3 == 0 {
            dirs_and_exts.push(args[x]);
        } else {
            dirs_and_exts.push(args[x]);
        }
    }
    
    if dirs_and_exts.len() != 0 {
        handle_directories(dirs_and_exts);
    } else {
        println!("You didn't supply enough arguments.
Did you forget to specify the old extension and the new extension for the files?");

        return;
    }
}

fn convert(file: String, ext_from: &String, ext_to: &String, options: &Config) {
    let file_str = file.as_str();
    
    let file_path = Path::new(file_str);
    
    let fuckyou = Path::with_extension(file_path, ext_to);
    let new_file = fuckyou.as_path().to_str().unwrap();
    
    if file_path.extension().unwrap().to_str().unwrap() == ext_from {
        let mut ffmpeg = process::Command::new("ffmpeg");
                ffmpeg.args(&["-i", file.as_ref()]);
                //ffmpeg.args(&["-b:a", "128k"]);
                
                for x in &options.config {
                    ffmpeg.arg(x);
                }
                
                ffmpeg.arg(new_file);
                
        match ffmpeg.output() {
            Ok(r)   => {
                if String::from_utf8_lossy(r.stdout.as_ref()) == "" {
                    //println!("Failed to convert: {}", file);
                    //println!("stderr: {}", String::from_utf8_lossy(r.stderr.as_ref()));
                    println!("Converted: {}", file);
                } else {
                    println!("{}", String::from_utf8_lossy(r.stdout.as_ref()));
                    //println!("{}\n{}", f, String::from_utf8_lossy(r.stdout.as_ref()));
                }
            },
            Err(e)  => panic!("Failed to convert, here's why: {}", e),
        }
    }
}

fn handle_directories(dir: Vec<&String>) {
    // DIRECTORY
    // Handles what files in the directory should be uploaded.
    let options = load_config();
    
    for d in 0..dir.len() {
        if d % 2 == 0 {
            let path = Path::new(dir[d]);
            let mut fls: Vec<_> = vec!();
            
            if path.is_dir() == true {
                match fs::walk_dir(&path) {
                    Err(why) => println!("! {:?}", why.kind()),
                    Ok(paths) => for path in paths {
                        fls.push(path.unwrap().path());
                    },
                }
                
                for x in fls {
                    // dir[d + 1] is the extension you want to convert FROM
                    // dir[d + 2] is the extensions you want to convert TO
                    
                    //let filename = x.as_path().file_name().unwrap().to_str().unwrap().to_string();
                    let file = x.to_str().unwrap().to_string();
                    
                    convert(file, dir[d + 1], dir[d + 2], &options);
                }
            }
        } 
    }
}

/*
fn convert_temp(file: String, ext_from: &String, ext_to: &String, options: &Config) {
    println!("{} : {} : {}", file, ext_from, ext_to);
    
    //let option = options.clone();
    
    for x in &options.config {
        println!("{}", x);
    }
}
* */

fn change_config(args: &Vec<String>) {
    // Changes config options
    // TODO: Make it take several values at once.
    // Example: ./uguupload -c options1 true option2 false option3 false
    
    let config_arguments_collection = &args[2..args.len()];
    let mut config_arguments: Vec<String> = Vec::new();
    
    for x in config_arguments_collection {
        config_arguments.push(x.clone());
    }
    
    let mut current_config: Config = load_config();
    
    current_config.config = config_arguments;
    
    save_config(current_config);
}

fn load_config() -> Config {
    // Reads the config files, decodes it and then returns a Config struct
    
    let config_file = Path::new("./").join(CONFIG);
    
    let mut boop = File::open(&config_file).unwrap();
    let mut output_from_config = "".to_string();
    
    let _ = match File::read_to_string(&mut boop, &mut output_from_config) {
        Ok(o)   => o,
        Err(e)  => panic!("HELP: {}", e),
    };
    
    let current_config: Config = json::decode(&output_from_config).unwrap();
    
    return current_config;
}

fn save_config(new_config: Config) {
    // Takes a Config as input and encodes the input to JSON and then writes it to the config file
    
    let config_file = Path::new("./").join(CONFIG);
    
    let conf_file = File::create(&config_file);

    let _ = conf_file.unwrap().write_all(json::encode(&new_config).unwrap().as_bytes());
}

fn help() {
    println!("Usage: ./ffbc [DIRECTORY] [OLD_EXT] [NEW_EXT]

[OLD_EXT] and [NEW_EXT] are file extensions.

USAGE
    ./ffbc some_directory flac mp3
    The first argument that gets passed is the target directory (some_directory).
    The second argument that gets passed is the current file extension you want to convert FROM (flac).
    The third argument that gets passed is the new file extension you want to convert TO (mp3).
    
    This will convert everything in that directory (recursively, so directories in the first directory will also be targeted).
    It targets flac files and converts them to mp3 using ffmpeg.
    
    You can also use the -c argument to pass some options to ffmpeg, like -b:a 320k to set the audio quality to 320kbps.
    ./ffbc -c -b:a 320k
    ./ffbc directory wav ogg
    
    The file formats you can convert from and to is limited by ffmpeg.
    The original files will NOT be replaced.
    
    Technically, you're supposed to be able to be able to supply the program with several directories and it will work.
    I haven't tried this though. ./ffbc dir1 wav flac dir2 wav flac dir3 mkv mp4

OPTIONS
    -h, --help
        Prints out this help message.
        
    -c, --config
        Changes the ffmpeg command-line arguments,
        See example 1,

EXAMPLES
    Example 1
        ./ffbc -c b:a 320k
        This will set the audio quality to 320kbps.
        
        After you've set that option, do this: ./ffbc some_directory flac mp3");
        
    return;
}
