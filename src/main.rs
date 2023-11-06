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
    println!("--export-midi : export midi track to mid files (TODO)");
    println!("--export-wave : export wave track to wav files (TODO)");
}


fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    println!("mmftool v{}", env!("CARGO_PKG_VERSION"));

    if env::args().count() == 0 || env::args().nth(1).is_none() {
        bail!("No file argument. Please insert mmf file path first.")
    }

    let arg_file_path = env::args().nth(1).unwrap();
    let file_path = std::path::PathBuf::from(&arg_file_path);
    if !file_path.exists() {
        bail!("File does not exist");
    }
    let arg_order_main = env::args().nth(2);
    
    let mmf_file_info = mmf_parser::parse(get_file_as_byte_vec(arg_file_path));
    match mmf_file_info {
        Ok(result) => {
            if arg_order_main.is_none() {
                print_mmf_info(&result, false);
            }
            else {
                //TODO: Parsing main order and execute some functions, If not, Place some bail!
                if let Some(order) = arg_order_main {
                    if order == "--export-midi" || order == "--export-wave"  {
                        bail!("TODO Command")
                    }
                    else if order == "--help" {
                        print_help();
                    }
                    else {
                        bail!("Unknown argument")
                    }
                }
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
