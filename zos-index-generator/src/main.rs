/**
 * SPDX-FileCopyrightText: 2023 Jason Mo <jasonmo2009@hotmail.com>
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use std::io;
use std::env;
use std::fs;
use sha256::try_digest;

fn main() {
    let helpmsg = "usage: zig [new][help]";
    let args: Vec<String> = env::args().collect();
    println!("zos-index-generator v0.1.0\n");

    if args.len() > 1 {
        if args[1] == "new" {
            let name = read_name();
            let urls = read_urls();
            let inptimg = read_img();
            let version = read_ver(&inptimg);
            let hash = read_hash(&inptimg);
            println!("Please clone https://github.com/JasonMo1/ZOS-Index-demo.git and add following characters into index.json:");
            println!("");
            println!(",");
            println!("{{");
            println!("    \"name\":\"{}\",", name);
            println!("    \"urls\":\n    [");
            for url in urls {
                println!("        \"{}\"", url);
            }
            println!("    ],");
            println!("    \"version\":\"{}\",", version);
            println!("    \"hash\":\"{}\"", hash);
            println!("}}");
        }
        else if args[1] == "help" {
            println!("{}", helpmsg);
        }
    }
    else{
        println!("{}", helpmsg);
    }

}

fn read_name() -> String {
    println!("The name of your image [less than 25 half-width characters]\nformat [os-${{git tag}}-publisher+program inside]:");
    let mut name :String;
    loop {
        name = read_line();
        if name.len() > 25 {
            println!("\nError: Input a name over 25 characters");
            println!("Please input again:");
            continue;
        }
        else if name.len() <= 25 {
            break
        }
    }
    return name;
    
}

fn read_urls() -> Vec<String> {
    let mut index = 0;
    let mut urllist :Vec<String> = vec![];
    loop {
        println!("\nThe value of \"urls\"({index}):");
        let url = read_line();
        if url != "" {
            urllist.push(url);
        }
        else {
            println!("\nError: Url shouldn't be an empty string");
            println!("Please input again:");
            let urlagain = read_line();
            urllist.push(urlagain);
        }

        println!("\nAre there more urls? (Y/n) [default n]:");
        let moreurls = read_line();
        let murlres = ynprompt(&moreurls);
        if murlres == false {
            break;
        }
        index += 1;
    }
    return urllist;
}

fn read_img() -> String {
    // Download from url in the future.
    println!("\nWhere is your image? [please delete \" in the path]:");
    let inptimg = read_line();
    return inptimg
}

fn read_ver(inptimg :&String) -> String {
    let mut version :String;
    let vers1 = fs::read(inptimg).unwrap();
    let vers2 = &vers1[769..787];

    println!("\nWhat's your name?");
    let usrname = read_line();
    let program :String;
    println!("\nAre there any program in this image? (Y/n):");
    let rdynprogram = read_line();
    let ynprogram = ynprompt(&rdynprogram);
    if ynprogram == true {
        println!("\nPlease input the name of your program:");
        let programname = read_line();
        program = "+".to_string()+&programname;
    }
    else {
        program = "".to_string()
    }
    let vers3 = String::from_utf8_lossy(&vers2).to_string();
    version = "zos-".to_owned()+&vers3+"-main-"+&usrname+&program;


    println!("\nIs this your image version? (Y/n)\n[{}]", version);
    let ynvergetln = read_line();
    let ynverget = ynprompt(&ynvergetln);
    if ynverget == false {
        println!("\nPlease input your version in this format [os-${{git describe --tags}}-branch-publisher]:");
        version = read_line();
    }
    return version;
}

fn read_hash(inptimg :&String) -> String {
    let computed_hash = try_digest(std::path::Path::new(&inptimg)).unwrap();
    return computed_hash;
}

///////////
// Utils //
///////////
fn read_line() -> String {
    let mut inptsrc: String = String::new();

    io::stdin()
        .read_line(&mut inptsrc)
        .expect("Error: Failed to read input");

    inptsrc.pop();
    inptsrc.pop();

    return inptsrc;
}

fn ynprompt(inputstr:&String) -> bool {
    let res: bool;
    if inputstr == "Y" {
        res = true;
    }
    else if inputstr == "Yes" {
        res = true;
    }
    else if inputstr == "y" {
        res = true;
    }
    else if inputstr == "yes" {
        res = true;
    }
    else if inputstr == "N" {
        res = false;
    }
    else if inputstr == "No" {
        res = false;
    }
    else if inputstr == "n" {
        res = false;
    }
    else if inputstr == "no" {
        res = false;
    }
    else if inputstr == "" {
        res = false;
    }
    else {
        println!("\nError: Please input \"Y\" or \"n\"");
        println!("Please input again:");
        let readagain = read_line();
        res = ynprompt(&readagain);
    }

    return res;
}