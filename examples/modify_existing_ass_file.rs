use ass_parser::{AssFile, Dialogue, AssFileOptions};
use hex_color::HexColor;

fn main() -> Result<(), std::io::Error>{
    let mut ass_file = AssFile::from_file("subtitles.ass")?;
    let dialogue = Dialogue::default()
        .set_text("Hello Friend!");
    let primary_color = AssFileOptions::get_ass_color(HexColor::RED);


    ass_file.components.v4
        .set_primarycolour(&primary_color);
        
    ass_file.components.events
        .add_dialogue(dialogue);

    AssFile::save_file(&ass_file, "new_subtitles.ass");

    Ok(())
}
