use std::env;

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

fn print_help(mmf_info:&mmf_parser::MmfFileInfo, show_track_info:bool) {
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

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    println!("mmftool v{}", env!("CARGO_PKG_VERSION"));
    let path = env::args().nth(1).ok_or_else(|| anyhow::anyhow!("No filename argument"))?;
    let mmf_file_info = mmf_parser::parse(get_file_as_byte_vec(path));
    match mmf_file_info {
        Ok(result) => {
            print_help(&result, false);
        }
        Err(e) => {
            
        }
    }

    Ok(())
}
