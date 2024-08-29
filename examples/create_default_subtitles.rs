use ass_parser::AssFile;
use ass_parser::{ScriptInfo, V4Format, Events};

fn main() {
    let mut ass_file = AssFile::new();

    ass_file.components.script
        .set_script(ScriptInfo::default());

    ass_file.components.v4
        .set_v4(V4Format::default());

    ass_file.components.events
        .set_events(Events::default());

    AssFile::save_file(&ass_file, "default_subtitles.ass");
}
