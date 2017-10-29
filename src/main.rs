extern crate regex;
//use regex::Regex;
//use std::env;
use std::process::{Command};
use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use std::io::{BufReader,BufRead};
use std::error::Error;
use std::path::Path;


fn main() {
    //Rewrite to Main is needed.
    // This is needed for the end user to know what is happening.
    let mut holder:Vec<String> = vec![];
    let mut hold:Vec<String> = vec![];
    /*let pass = "PASS!";
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
    }*/
    //modify_pacman_conf();
    holder =  check_pacman_conf();
    check_pacman_conf();
    let check_one = create_pacman_conf_old();;
    println!("Created backup of \"/etc/pacman.conf:\" {}", check_one);
    create_new_pacman_conf(holder);
    hold = pacman_conf_final();
    create_new_pacman_conf(hold);

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
        .arg("out/pacman.conf")
        .arg("out/pacman.conf.old")
        .output()
        .expect("Process Failed to Execute!");
    if output.stderr.is_empty(){
        return check;
    }else {
        return !check;
    }
}
//This function will be called on two separate iterations
fn create_new_pacman_conf(x:Vec<String>) {
    let path = Path::new("out/pacman.conf");
    let display = path.display();
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .open("out/pacman.conf")
        .unwrap();
    for item in x.iter(){
        if let Err(e) =
        writeln!(f, "{}",item.to_string()){
            println!("{}", e);
        }
    }
}

fn check_pacman_conf() ->Vec<String> {

    let mut contents:Vec<String> = vec![];
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

    let filename = r"/etc/pacman.conf";
    let mut f =
        BufReader::new(File::open(filename).unwrap());
    for line in f.lines() {
        match line {
            Ok(line) => if line.contains(&check_str_one) {
                let str_replace =
                    line.replace(&check_str_one, &check_str_two);
                contents.push(str_replace.to_string());
                //println!("{}", str_replace);
            } else if line.contains(&check_str_three) {
                let str_replace =
                    line.replace(&check_str_three, &check_str_four);
                contents.push(str_replace.to_string());
                //println!("{}", str_replace);
            } else if line.contains(&check_str_five) {
                let str_replace =
                    line.replace(&check_str_five, &check_str_six);
                contents.push(str_replace.to_string());
                //println!("{}", str_replace);
            } else if line.contains(&check_str_seven) {
                let str_replace =
                    line.replace(&check_str_seven, &check_str_eight);
                contents.push(str_replace.to_string());
                //println!("{}", str_replace);
            } else {
                contents.push(line.to_string());
                //println!("{}", line.to_string());
            }
            //Error logic is flawed.. need to workout error check
            //to determine if pacman.conf needs to be modified.
            Err(_) => if check_str_two.eq(line.unwrap().as_str()) {
                println!("No change needed");
            } else {
                println!("Something Happened");
            }
        };
    };
    return contents;
}

fn archlabs_keyring() -> bool {
    let check:bool = true;
    let output =
        Command::new("pacman")
            .arg("-S")
            .arg("archlabs-keyring")
            .output()
            .expect("Process Failed To Execute!");
    if output.stderr.is_empty(){
        return check;
    }else {
        return !check;
    }
}

fn key_populate() -> bool {
    let check:bool = true;
    let output =
        Command::new("pacman-key")
            .arg("--populate")
            .arg("archlabs")
            .output()
            .expect("Process Failed To Execute!");
    if output.stderr.is_empty(){
        return check;
    }else {
        return !check;
    }
}

fn pacman_conf_final() -> Vec<String>{

    let mut contents:Vec<String> = vec![];
    let check_str_one:String =
        "[archlabs_repo]".to_string();
    let check_str_two:String =
        "SigLevel = Never".to_string();
    let check_str_three:String =
        "SigLevel = Optional TrustAll".to_string();
    let filename = r"/etc/pacman.conf";
    let mut f =
        BufReader::new(File::open(filename).unwrap());
    for line in f.lines() {
        match line {
            Ok(line) => if line.contains(&check_str_one) {
                let str_replace =
                    line.replace(&check_str_one, &check_str_one);
                contents.push(str_replace.to_string());
                //println!("{}", str_replace);
            } else if line.contains(&check_str_two) {
                let str_replace =
                    line.replace(&check_str_two, &check_str_three);
                contents.push(str_replace.to_string());
                //println!("{}", str_replace);
            } else {
                contents.push(line.to_string());
                //println!("{}", line.to_string());
            }
            //Error logic is flawed.. need to workout error check
            //to determine if pacman.conf needs to be modified.
            Err(_) => if check_str_two.eq(line.unwrap().as_str()) {
                println!("No change needed");
            } else {
                println!("Something Happened");
            }
        };
    };
    return contents;
}