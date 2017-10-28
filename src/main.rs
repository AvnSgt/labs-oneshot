extern crate regex;
use regex::Regex;
use std::env;
use std::process::{Command, exit};
use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use std::io::{BufReader,BufRead};


fn main() {
    //Rewrite to Main is needed.
    // This is needed for the end user to know what is happening.
    let pass = "PASS!";
    let fail = "FAIL!";
    let key_one = gpg_import_key_one();
    let key_two = gpg_import_key_two();
    let key_three = gpg_import_key_three();

    if key_one && key_two && key_three{
        println!("1st Key Import {}", pass);
        println!("2nd Key Import {}", pass);
        println!("3rd Key Import {}", pass);
    }else if !key_one {
        println!("1st Key Import {}", fail);
        exit(1);
    }else if key_one &! key_two{
        println!("2nd Key Import {}", fail);
        exit(1);
    }else if key_one && key_two &!key_three {
        println!("3rd Key Import {}", fail);
        exit(1);
    }

    let init_check = gpg_init();

    if init_check {
        println!("Pacman Key Init {}", pass);
    }else {
        println!("Pacman Key Init {}", fail);
    }
    //modify_pacman_conf();
    //check_pacman_conf();
    let check = create_pacman_conf_old();
    println!("{}", check);
}

fn gpg_import_key_one() -> bool{
    let check:bool = true;
    let output = Command::new("gpg")
        .arg("--receive-keys")
        .arg("AEFB411B072836CD48FF0381AE252C284B5DBA5D")
        .output()
        .expect("Process Failed to Execute!");
    if output.stderr.is_empty(){
        return check;
    }else {
        return !check;
    }
}

fn gpg_import_key_two() -> bool{
    let check:bool = true;
    let output = Command::new("gpg")
        .arg("--receive-keys")
        .arg("9E4F11C6A072942A7B3FD3B0B81EB14A09A25EB0")
        .output()
        .expect("Process Failed to Execute!");
    if output.stderr.is_empty(){
        return check;
    }else {
        return !check;
    }
}

fn gpg_import_key_three() -> bool{
    let check:bool = true;
    let output = Command::new("gpg")
        .arg("--receive-keys")
        .arg("35F52A02854DCCAEC9DD5CC410443C7F54B00041")
        .output()
        .expect("Process Failed to Execute!");
    if output.stderr.is_empty(){
        return check;
    }else {
        return !check;
    }
}

fn gpg_init() -> bool{
    let check:bool = true;
    let output = Command::new("pacman-key")
        .arg("--init")
        .output()
        .expect("Process Failed to Execute!");
    if output.stderr.is_empty(){
        return check;
    }else {
        return !check;
    }
}

fn gpg_remove_key_one() -> bool{
    let check:bool = true;
    let output = Command::new("gpg")
        .arg("-r")
        .arg("AEFB411B072836CD48FF0381AE252C284B5DBA5D")
        .output()
        .expect("Process Failed to Execute!");
    if output.stderr.is_empty(){
        return check;
    }else {
        return !check;
    }
}

fn gpg_remove_key_two() -> bool{
    let check:bool = true;
    let output = Command::new("gpg")
        .arg("-r")
        .arg("9E4F11C6A072942A7B3FD3B0B81EB14A09A25EB0")
        .output()
        .expect("Process Failed to Execute!");
    if output.stderr.is_empty(){
        return check;
    }else {
        return !check;
    }
}

fn gpg_remove_key_three() -> bool {
    let check:bool = true;
    let output = Command::new("gpg")
        .arg("-r")
        .arg("35F52A02854DCCAEC9DD5CC410443C7F54B00041")
        .output()
        .expect("Process Failed to Execute!");
    if output.stderr.is_empty(){
        return check;
    }else {
        return !check;
    }
}

fn create_pacman_conf_old() -> bool{
    let check:bool = true;
    let output = Command::new("mv")
        .arg("/etc/pacman.conf")
        .arg("/etc/pacman.conf.old")
        .output()
        .expect("Process Failed to Execute!");
    if output.stderr.is_empty(){
        return check;
    }else {
        return !check;
    }

}

fn modify_pacman_conf(){
    //Function simply appends lines to the end of a file.
    //Used to test idea, and with and without privilege escalation.
    //Does not solve duplicates or allow for overwrite/clobber.
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/etc/pacman.conf.test")
        .unwrap();
    if let Err(e) =
    writeln!(f, "[archlabs-repo]"){
        println!("{}", e);
    }
    if let Err(e) =
    writeln!(f, "SigLevel = Never"){
        println!("{}", e);
    }
    if let Err(e) =
    writeln!(f, "Server = https://archlabs.github.io/archlabs_repo/$arch"){
        println!("{}", e);
    }
    if let Err(e) =
    writeln!(f, "Server = https://downloads.sourceforge.net/project/archlabs-repo/archlabs_repo/$arch"){
        println!("{}", e);
    }
}

fn check_pacman_conf(){
    let mut one = 0;
    let mut two = 0;
    let mut three = 0;
    let mut four = 0;
    let mut count = 0;
    let mut recount = 0;
    let mut contents = vec![];
    let check_str_one:String =
        "#[archlabs_repo]".to_string();
    let check_str_two:String =
        "[archlabs_repo]".to_string();
    let check_str_three:String =
        "#SigLevel = Never".to_string();
    let check_str_four:String =
        "SigLevel = Never".to_string();
    let check_str_five:String =
        "#Server = https://archlabs.github.io/archlabs_repo/$arch".to_string();
    let check_str_six:String =
        "Server = https://archlabs.github.io/archlabs_repo/$arch".to_string();
    let check_str_seven:String =
        "#Server = https://downloads.sourceforge.net/project/\
        archlabs-repo/archlabs_repo/$arch".to_string();
    let check_str_eight:String =
        "Server = https://downloads.sourceforge.net/project/\
        archlabs-repo/archlabs_repo/$arch".to_string();
    let not_found = "Not Found";
    let found = "found";
    let filename = r"/etc/pacman.conf.old";
    let mut f =
        BufReader::new(File::open(filename).unwrap());
    for line in f.lines(){
        match line {
            Ok(line) => if line.contains(&check_str_one){
                let str_replace =
                    line.replace(&check_str_one, &check_str_two);
                contents.push(str_replace);
                //println!("{}", str_replace);
                }else if line.contains( &check_str_three){
                let str_replace =
                    line.replace(&check_str_three, &check_str_four);
                contents.push(str_replace);
                //println!("{}", str_replace);
                }else if line.contains(&check_str_five) {
                let str_replace =
                    line.replace(&check_str_five, &check_str_six);
                contents.push(str_replace);
                //println!("{}", str_replace);
                }else if line.contains(&check_str_seven){
                    let str_replace =
                        line.replace(&check_str_seven, &check_str_eight);
                contents.push(str_replace);
                //println!("{}", str_replace);
                }else {
                contents.push(line.to_string());
                //println!("{}", line.to_string());
            }
            //Error logic is flawed.. need to workout error check
            //to determine if pacman.conf needs to be modified.
            Err(_) => if check_str_two.eq(line.unwrap().as_str()){
                println!("No change needed");
            }else {
                println!("Something Happened");
            }
        };
    }
}