extern crate regex;
use regex::Regex;
use std::env;
use std::process::{Command, exit};
use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use std::io::BufReader;


fn main() {
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
    check_pacman_conf();
}

fn gpg_import_key_one() -> bool{
    let output = Command::new("gpg")
        .arg("--receive-keys")
        .arg("AEFB411B072836CD48FF0381AE252C284B5DBA5D")
        .output()
        .expect("Process Failed to Execute!");
    let check = String::from_utf8(output.stdout).is_ok();
    return check
}

fn gpg_import_key_two() -> bool{

    let output = Command::new("gpg")
        .arg("--receive-keys")
        .arg("9E4F11C6A072942A7B3FD3B0B81EB14A09A25EB0")
        .output()
        .expect("Process Failed to Execute!");
    let check = String::from_utf8(output.stdout).is_ok();
    return check

}
fn gpg_import_key_three() -> bool{

    let output = Command::new("gpg")
        .arg("--receive-keys")
        .arg("35F52A02854DCCAEC9DD5CC410443C7F54B00041")
        .output()
        .expect("Process Failed to Execute!");
    let check = String::from_utf8(output.stdout).is_ok();
    return check

}

fn gpg_init() -> bool{

    let output = Command::new("pacman-key")
        .arg("--init")
        .output()
        .expect("Process Failed to Execute!");
    let check = String::from_utf8(output.stdout).is_ok();
    return check

}

fn gpg_remove_key_one() -> bool{
    let output = Command::new("gpg")
        .arg("-r")
        .arg("AEFB411B072836CD48FF0381AE252C284B5DBA5D")
        .output()
        .expect("Process Failed to Execute!");
    let check = String::from_utf8(output.stdout).is_ok();
    return check
}

fn gpg_remove_key_two() -> bool{

    let output = Command::new("gpg")
        .arg("-r")
        .arg("9E4F11C6A072942A7B3FD3B0B81EB14A09A25EB0")
        .output()
        .expect("Process Failed to Execute!");
    let check = String::from_utf8(output.stdout).is_ok();
    return check

}

fn gpg_remove_key_three() -> bool {
    let output = Command::new("gpg")
        .arg("-r")
        .arg("35F52A02854DCCAEC9DD5CC410443C7F54B00041")
        .output()
        .expect("Process Failed to Execute!");
    let check = String::from_utf8(output.stdout).is_ok();
    return check;
}

fn modify_pacman_conf(){

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
    let filename = r"/etc/pacman.conf.test";
    let mut f =
        File::open(filename).expect("File Not Found!");
    for line in BufReader::new(f).lines(){
        contents.push(line.unwrap());
        count +=1;
    }

    while recount <= (count - 1) {

        if check_str_one.eq(contents.get(recount).unwrap()){
            one = recount;
            println!("{:?}", contents.get(one).unwrap());
            recount += 1;
        }else if check_str_two.eq(contents.get(recount).unwrap()){
            one = recount;
            println!("{:?}", contents.get(one).unwrap());
            recount += 1;
        }else {
            recount += 1;
        }

        if check_str_three.eq(contents.get(recount).unwrap()){
            two = recount;
            println!("{:?}", contents.get(two).unwrap());
            recount += 1;
        }else if check_str_four.eq(contents.get(recount).unwrap()){
            two = recount;
            println!("{:?}", contents.get(two).unwrap());
            recount += 1;
        }else {
            recount += 1;
        }

        if check_str_five.eq(contents.get(recount).unwrap()){
            three = recount;
            println!("{:?}", contents.get(three).unwrap());
            recount += 1;
        }else if check_str_six.eq(contents.get(recount).unwrap()){
            three = recount;
            println!("{:?}", contents.get(three).unwrap());
            recount += 1;

        }else {
            recount += 1;
        }

        if check_str_seven.eq(contents.get(recount).unwrap()){
            four = recount;
            let str:&str = contents.get(four).unwrap();
            let modstr = str.replace("#", "");

            println!("{:?}", contents.get(four).unwrap());
            recount += 1;
        }else if check_str_eight.eq(contents.get(recount).unwrap()){
            four = recount;
            println!("{:?}", contents.get(four).unwrap());
            recount += 1;

        }else {
            recount += 1;
        }

    }
    /*
    let re = Regex::new(r"![[:punct:]]|\bSigLevel = Never").unwrap();
    let caps = re.captures(contents.as_str()).unwrap();
    println!("{:?}", caps.get(0).unwrap().as_str().replace
    ("SigLevel = Never", "SigLevel = Optional Trustall"));*/

}