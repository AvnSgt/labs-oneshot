use std::process::{Command, exit};
use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use std::io::{BufReader,BufRead};
use std::error::Error;
use std::path::Path;


fn main() {
    let keys:Vec<String> =
        vec!["AEFB411B072836CD48FF0381AE252C284B5DBA5D".to_string(),
             "9E4F11C6A072942A7B3FD3B0B81EB14A09A25EB0".to_string(),
             "35F52A02854DCCAEC9DD5CC410443C7F54B00041".to_string()];
    let keys_clone:Vec<String> = keys.clone();
    //let mut contents:Vec<String> = vec![];
    gpg_import_keys(keys);
    let init_check:bool = gpg_init();
    if !init_check{
        println!("Update has failed, after gpg keys have been imported.");
        println!("Ensure you run update with \"sudo\", or super user privileges");
        exit(1);
    }else {
        println!("GPG Init: Task Completed Successfully!");
    }
    gpg_remove_keys(keys_clone);
    let mut contents:Vec<String> = check_pacman_conf();
    create_pacman_conf_old();
    create_new_pacman_conf(contents);
    archlabs_keyring();
    key_populate();
    contents = pacman_conf_final();
    create_new_pacman_conf(contents);


    println!("The automated update portion of the process has completed!");
    println!("Please visit the ArchLABs Forums:");
    println!("https://www.tapatalk.com/groups/archlabs\
    /upgrade-guide-for-existing-installations-t432.html");
    println!("To continue with the manual steps in the process...");
}

fn gpg_import_keys(k:Vec<String>) {


    let mut check:i8 = 0;
    for item in k.iter(){
        let keys:String =
            item.to_string();

        let output = Command::new("gpg")
            .arg("--receive-keys")
            .arg(keys.to_string())
            .output()
            .expect("Process Failed to Execute!");
        if output.stderr.is_empty(){
            println!("{}", String::from_utf8(output.stdout).unwrap());
            check +=1 ; //or default of true
        }else {
            println!("{}", String::from_utf8(output.stderr).unwrap());
            check -= 1;
        }
    }
    if check != 3{
        // One or more keys were not imported.
        print!("{} ", (check*-1));
        println!("keys were not imported");
    }else if check == 3 {
        print!("{} ", (check));
        println!("keys were imported successfully.");
    }
}

fn gpg_init() -> bool{
    let check:bool;
    let output = Command::new("pacman-key")
        .arg("--init")
        .output()
        .expect("Process Failed to Execute!");
    if output.stderr.is_empty(){
        print!("Success: ");
        println!("{}", String::from_utf8(output.stdout).unwrap());
        check = true;
    }else {
        print!("Error: ");
        println!("{}", String::from_utf8(output.stderr).unwrap());
        check = false;
    }
    return check;
}

fn gpg_remove_keys(k:Vec<String>) {


    let mut check:i8 = 0;
    for item in k.iter() {
        let keys:String =
            item.to_string();

        let output =
            Command::new("pacman-key")
            .arg("-r")
            .arg(keys.to_string())
            .output()
            .expect("Process Failed to Execute!");

        if output.stderr.is_empty(){
            println!("{}", String::from_utf8(output.stdout).unwrap());
            check +=1 ; //or default of true
        }else {
            println!("{}", String::from_utf8(output.stderr).unwrap());
            check -= 1;
        }
    }
    if check != 3{
        // One or more keys were not imported.
        print!("{} ", (check*-1));
        println!("keys were not removed!");
    }else if check == 3 {
        print!("{} ", (check));
        println!("keys were removed successfully.");
    }
}

fn create_pacman_conf_old(){
    let output = Command::new("mv")
        .arg("/etc/pacman.conf")
        .arg("/etc/pacman.conf.old")
        .output()
        .expect("Process Failed to Execute!");
    if output.stderr.is_empty(){
        println!("{}", String::from_utf8(output.stdout).unwrap());
        println!("Back up of \"pacman.conf\" complete!");
    }else {
        println!("{}", String::from_utf8(output.stderr).unwrap());
    }
}
//This function will be called on two separate iterations
fn create_new_pacman_conf(x:Vec<String>) {
    let path = Path::new("/etc/pacman.conf");
    let display = path.display();
    // Open a file in write-only mode, returns `io::Result<File>`
    let _file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(_file) => _file,
    };

    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/etc/pacman.conf")
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
    let f =
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

fn archlabs_keyring() {
    let output =
        Command::new("pacman")
            .arg("-S")
            .arg("archlabs-keyring")
            .output()
            .expect("Process Failed To Execute!");
    if output.stderr.is_empty(){
        println!("{}", String::from_utf8(output.stdout).unwrap());

    }else {
        println!("{}", String::from_utf8(output.stdout).unwrap());
    }
}

fn key_populate() {
    let output =
        Command::new("pacman-key")
            .arg("--populate")
            .arg("archlabs")
            .output()
            .expect("Process Failed To Execute!");
    if output.stderr.is_empty(){
        println!("{}", String::from_utf8(output.stdout).unwrap());
    }else {
        println!("{}", String::from_utf8(output.stderr).unwrap());
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
    let f =
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