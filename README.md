
<div align="center">

[![Crate Badge]][Crate] [![Docs Badge]][API Docs] [![Deps.rs
Badge]][Deps.rs]<br> ![Crates.io Total Downloads](https://img.shields.io/crates/d/ass_parser)

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
 
 AssParser is based on the principle of easy to read write and modify `.ass` files. This is the first version of `ass_parser`and now currently only have the features to modify `.ass` file.
 
 # Example
 
 Creating a simple `Advanced SubStation Alpha` `(.ass)` file with default values

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
 
