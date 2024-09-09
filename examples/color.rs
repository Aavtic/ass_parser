use hex_color::HexColor;
use ass_parser::AssFile;
use ass_parser::{ScriptInfo, V4Format, Events, Dialogue};

fn main() {
    let mut ass_file = AssFile::new();

    let dialogue = Dialogue::new()
        .set_start("0:00:00:00")
        .set_end("0:00:02:00")
        .set_text("Hello Friend!")
        .set_colour(HexColor::YELLOW);

    ass_file.components.script
        .set_script(ScriptInfo::default());

    ass_file.components.v4
        .set_v4(V4Format::default());

    ass_file.components.events
        .set_events(Events::default())
        .add_dialogue(dialogue);

    AssFile::save_file(&ass_file, "new_subtitles.ass");
}

