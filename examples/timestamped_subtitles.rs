use ass_parser::{self, AssFile, ScriptInfo, V4Format, Events, AssFileOptions, Dialogue};
use ass_parser::IndexNotFound;
use hex_color::HexColor;

fn main() -> Result<(), IndexNotFound>{
    let mut ass_file = AssFile::new();
    let hexcolor = AssFileOptions::get_ass_color(HexColor::YELLOW);

    let first_dialogue = Dialogue::default()
        .set_start("0:00:00.10")
        .set_end("0:00:00.50");

    let second_dialogue = Dialogue::default()
        .set_start("00:00.50")
        .set_end("00:00.58");

    let third_dialogue = Dialogue::default()
        .set_start("0:00:00.58")
        .set_end("0:00:01.01");

    let events = Events::new()
        .add_first_dialogue(first_dialogue)?
        .add_dialogue(second_dialogue)
        .add_dialogue(third_dialogue)
        .create();


    ass_file.components.script
        .set_script(ScriptInfo::default())
        .set_scripttype("FFMPEG");

    ass_file.components.v4
        .set_v4(V4Format::default())
        .set_primarycolour(&hexcolor);

    ass_file.components.events
        .set_events(events);

    AssFile::save_file(&ass_file, "new_subtitles.ass");

    Ok(())

}
