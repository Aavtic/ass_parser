use hex_color::HexColor;
use ass_parser::{AssFile, AssFileOptions};
use ass_parser::{ScriptInfo, V4Format, Events, Dialogue};
use rand;

fn main() {
    let hexcolor = AssFileOptions::get_ass_color(HexColor::YELLOW);
    let srt_file = AssFile::from_srt("RapGod.srt");
    let mut ass_file = AssFile::new();
    let mut event = Events::default();

    for srt_seg in srt_file.iter() {
        let start = &srt_seg.start;
        let end = &srt_seg.end;
        let text = &srt_seg.text;

        let random_color:HexColor = rand::random();

        let dialogue = Dialogue::default()
            .set_start(&start)
            .set_end(&end)
            .set_text(&text)
            .set_colour(random_color);

        event.add_dialogue(dialogue);
    }
    

    ass_file.components.script
        .set_script(ScriptInfo::default());



    ass_file.components.v4
        .set_v4(V4Format::default())
        .set_primarycolour(&hexcolor);
    ass_file.components.events
        .set_events(event);

    AssFile::save_file(&ass_file, "new_subtitle.ass");
}
