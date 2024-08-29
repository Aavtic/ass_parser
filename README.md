
<div align="center">

[![Crate Badge]][Crate] [![Docs Badge]][API Docs] [![Deps.rs
Badge]][Deps.rs]<br> ![Crates.io Total Downloads](https://img.shields.io/crates/d/ass_parser) [![Join Matrix Group](https://img.shields.io/badge/Join%20Matrix%20Group-Invite-brightgreen)](https://matrix.to/#/#ass_parser:matrix.org)

</div>


 

 # AssParser
 
 [ass_parser] is a crate to parse .ass (Advanced SubStation Alpha) files. which is a subtitle file for creating and displaying subtitles in video files. It is widely used due to it's complex text formatting, positioning and styling. The Advanced SubStation Alpha is a successor
 to the SubStation Alpha .ssa file.
 
 ## Installation
 
 Add `ass_parser` as a dependency to your cargo.toml:
 
  ```shell
  cargo add ass_parser
  ```

 # Introduction
 
 AssParser is based on the principle of easy to read write and modify `.ass` files. You can create, modify and edit ASS files however you want using this tool.

 # Example
 
Creating a simple `Advanced SubStation Alpha` `(.ass)` file with default values!

```rust
use ass_parser::{self, AssFile, ScriptInfo, V4Format, Events, AssFileOptions};
use hex_color::HexColor;

fn main() {
    let mut ass_file = AssFile::new();
    let hexcolor = AssFileOptions::get_ass_color(HexColor::YELLOW);

    ass_file.components.script
        .set_script(ScriptInfo::default());

    ass_file.components.v4
        .set_v4(V4Format::default())
        .set_primarycolour(&hexcolor);

    ass_file.components.events
        .set_events(Events::default());

    AssFile::save_file(&ass_file, "new_subtitles.ass")

}

```
Here we create an .ass file with default values and When you open the .ass file you can see the
following content.
```
ScriptType: v4.00+
PlayResX: 384
PlayResY: 288
ScaledBorderAndShadow: yes
YCbCr Matrix: None


[V4+ Styles]
Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding
Style: Default,Arial,16,&H00ff,&Hffffff,&H0,&H0,0,0,0,0,100,100,0,0,1,1,0,2,10,10,10,1


[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
Dialogue: 0,0:00:00.00,0:00:01.00,Default,,0,0,0,,Hello Friend
```

# Add Dialogues

```rust
use ass_parser::{self, AssFile, ScriptInfo, V4Format, Events, AssFileOptions, Dialogue};
use ass_parser::IndexNotFound;
use hex_color::HexColor;

fn main() -> Result<(), IndexNotFound>{
    let mut ass_file = AssFile::new();
    let hexcolor = AssFileOptions::get_ass_color(HexColor::YELLOW);

    let first_dialogue = Dialogue::default()
        .set_text("Hello There!")
        .set_start("0:00:00.10")
        .set_end("0:00:00.50");

    let second_dialogue = Dialogue::default()
        .set_text("Hello Friend!")
        .set_start("00:00.50")
        .set_end("00:00.58");

    let third_dialogue = Dialogue::default()
        .set_text("Hello World!!")
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
```

## This will generate an ASS file which would be similiar to this

```
ScriptType: FFMPEG
PlayResX: 384
PlayResY: 288
ScaledBorderAndShadow: yes
YCbCr Matrix: None


[V4+ Styles]
Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding
Style: Default,Arial,16,&H0ffff,&Hffffff,&H0,&H0,0,0,0,0,100,100,0,0,1,1,0,2,10,10,10,1


[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
Dialogue: 0,0:00:00.10,0:00:00.50,Default,,0,0,0,,Hello There!
Dialogue: 0,00:00.50,00:00.58,Default,,0,0,0,,Hello Friend!
Dialogue: 0,0:00:00.58,0:00:01.01,Default,,0,0,0,,Hello World!!
```
# Events can also be created like this


```rust
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
```

You can burn this subtitle file to a video or use any video player to select a video file along
with this subtitle file.

# Using [FFmpeg] to burn the video with the subtitles file.

You will first have to download and install [FFmpeg] on your system to try this. Once you have
downloaded you can use the following command to burn the video file `video.avi` and the
generated subtitle file `new_subtitles.ass` to a single output video file `output.avi`

```shell
ffmpeg -i video.avi -vf "ass=new_subtitles.ass" output.avi
```
 
[FFmpeg]: https://www.ffmpeg.org/about.html
[ass_parser]: https://github.com/Aavtic/ass_parser
[Crate Badge]: https://img.shields.io/crates/v/ass_parser?logo=rust&style=flat-square&logoColor=E05D44&color=E05D44
[Docs Badge]: https://img.shields.io/docsrs/ass_parser?logo=rust&style=flat-square&logoColor=E05D44
[Crate]: https://crates.io/crates/ass_parser/
[Api Docs]: https://docs.rs/ass_parser/latest/ass_parser/
[Deps.rs Badge]: https://deps.rs/repo/github/aavtic/ass_parser/status.svg?style=flat-square
[Deps.rs]: https://deps.rs/crate/ass_parser
[Matrix Badge]: https://img.shields.io/matrix/ass_parser:matrix.org.svg?style=flat-square&logo=matrix&label=Matrix&color=C43AC3
[Matrix]: https://matrix.to/#/#ass_parser:matrix.org
