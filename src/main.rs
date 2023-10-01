use std::env;

use anyhow::bail;
use mmf_parser::MmfParseResult;

fn get_file_as_byte_vec(filename: String) -> Vec<u8> {
    match std::fs::read(filename) {
        Ok(bytes) => {
            bytes
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("please run again with appropriate permissions.");
            }
            panic!("{}", e);
        }
    }
}

fn print_mmf_info(mmf_info:&mmf_parser::MmfFileInfo, show_track_info:bool) {
    println!("Title : {}", mmf_info.opda_block.song_title);
    println!("Author : {}", mmf_info.opda_block.author);
    println!("Copyright : {}", mmf_info.opda_block.copyright);

    println!("\nMIDI Track Count : {}", mmf_info.midi_blocks.len());
    if show_track_info {
        for midi_block in &mmf_info.midi_blocks {
            println!("-------------------");
            println!("MIDI Track Number : {}", midi_block.track_no);
            println!("MIDI Track Size : {}", midi_block.size);
            println!("-------------------");
        }
    }
    println!("WAVE Track Count : {}", mmf_info.wave_blocks.len());
    if show_track_info {
        for wave_block in &mmf_info.wave_blocks {
            println!("-------------------");
            println!("WAVE Track Number : {}", wave_block.track_no);
            println!("WAVE Track Size : {}", wave_block.size);
            println!("-------------------");
        }
    }
}

fn print_help() {
    println!("TODO: print help command message");
}


fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    println!("mmftool v{}", env!("CARGO_PKG_VERSION"));
    if env::args().count() == 0 {
        println!("No file argument");
        print_help();
        bail!("No file argument")
    }

    let path = env::args().nth(1).unwrap();
    let order_main = env::args().nth(2);
    
    let mmf_file_info = mmf_parser::parse(get_file_as_byte_vec(path));
    match mmf_file_info {
        Ok(result) => {
            if order_main.is_none() {
                print_mmf_info(&result, false);
            }
            else {
                //TODO: Parsing main order and execute some functions, If not, Place some bail!
            }
        }
        Err(e) => {
            if e == MmfParseResult::NotFoundSmafHeader {
                bail!("Not found SMAF File Header")
            }
            else {
                bail!("Unknown error")
            }
        }
    }

    Ok(())
}
