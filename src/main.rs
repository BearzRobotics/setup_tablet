// This program is to setup my tablet so I don't 
// have to do it manually every time.


// Furtue plans mnake code to auto detecte new monitor and set that up.
// That would evole using clap.
#[macro_use]
extern crate clap;
use clap::App;
use std::process::Command;
use std::io::stdin;
use std::env;

// I need to parse these two strings plus a 2 digit number after it
// UGEE TABLET MONITOR Mouse               	id=
// UGEE TABLET MONITOR Pen                 	id=

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut debug_flag: u8 = 0; // 0 = false, 1 = true

    let monitor = matches.value_of("monitor").unwrap();
    let debug = matches.value_of("debug").unwrap();
    if debug == "1" {
        debug_flag = 1;
    }

    // pull out substring 
    let ugee_pen_string = get_sub_string(debug_flag, "UGEE TABLET MONITOR Mouse".to_string());
    let ugee_mouse_string = get_sub_string(debug_flag, "UGEE TABLET MONITOR Pen".to_string());

    // get id from sub string
    let ugee_pen_id = get_device_id(debug_flag, ugee_pen_string);    
    let ugee_mouse_id = get_device_id(debug_flag, ugee_mouse_string);

    //pause();

    // run the xinput command to set pen and mouse to other monitor
    run_xinput(debug_flag, monitor.to_string(), ugee_pen_id);
    run_xinput(debug_flag, monitor.to_string(), ugee_mouse_id);

}

fn get_sub_string(debug_flag: u8, sub_string: String) -> String{
    let xinput_command =  Command::new("xinput")
        .arg("--list")
        .output()
        .expect("Failed to run xinput");

    // we need to get the output in bits to build a parseable string
    let s = String::from_utf8(xinput_command.stdout).unwrap();

    if debug_flag == 1 {
        println!("{}", &s);
    } 
    

    let start_bytes = s.find(&sub_string).unwrap();
    let mut result = &s[start_bytes..];

    if let Some(end) = result.find("id") {
            result = &s[start_bytes.. start_bytes+end+5];
    }

    if debug_flag == 1 {
        println!("{}", result.clone());
    }
    result.to_string()
}

fn get_device_id(debug_flag: u8, sub_string: String) -> u8{

    let id: Vec<char> = sub_string.chars().rev().take(2).collect();
    let id: String = id.into_iter().rev().collect();
    let id = id.parse::<u8>().unwrap();
    if debug_flag == 1 {
        println!("{}", id.clone());
    }
    id
}

fn run_xinput(debug_flag: u8, monitor: String, id: u8){
    let id = id.to_string();

    if debug_flag == 1 {
        println!("This is the command that is ran:");
        println!("xinput map-to-output {} {}", &id, &monitor);
    }

    let c = Command::new("xinput")
        .arg("map-to-output")
        .arg(id)
        .arg(monitor)
        .output()
        .expect("Failed to run command");
        
    // Prints an error message if presents
    let s = String::from_utf8(c.stderr);
    println!("{:?}", s);

}

/// This fucntion here is for debug purposes.
/// It acts as a sudo brake point, becaue I don't know how to use brake
/// points on system boot.
fn pause(){
    println!("You must type something");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

}
