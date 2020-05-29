// author: hzh
// date: 28 May 2020
// version: 1.0

extern crate winreg;
use std::io;
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;
use std::process::Command; 
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use encoding_rs::UTF_16LE;
use encoding_rs_io::DecodeReaderBytesBuilder;

fn main() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let ppath = hkcu.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Windows Messaging Subsystem\\Profiles\\")?;
    let dp: String = ppath.get_value("DefaultProfile")?;
    let dpstr: &str = dp.as_str(); 
    let fullpath = ["\"HKEY_CURRENT_USER\\Software\\Microsoft\\Windows NT\\CurrentVersion\\Windows Messaging Subsystem\\Profiles\\", dpstr].join("");
    let fullpathstr: &str = fullpath.as_str(); 
    {
        Command::new("reg.exe") 
            .args(&["export", fullpathstr, "H:\\old.reg.txt"])
            .output();
    }
    {
        let oldreg = File::open("H:\\old.reg.txt")?;
        let transcoded = DecodeReaderBytesBuilder::new()
            .encoding(Some(UTF_16LE))
            .build(oldreg);
        let mut buffered = BufReader::new(transcoded);
        let mut newreg = File::create("H:\\new.reg")?;
        let mut vec = Vec::new(); 

        for line in buffered.lines() {
            vec.push(line?.replace("SOPA-MAIL16", "SOPA-MAIL01"));
        }
        for line in &vec {
            writeln!(newreg, "{}", line)?;
        }
    }
    {
        Command::new("reg.exe")
            .args(&["import", "H:\\new.reg"])
            .output();
    }
    std::fs::remove_file("H:\\old.reg.txt")?;
    std::fs::remove_file("H:\\new.reg")?;
    Ok(())
}
