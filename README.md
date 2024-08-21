<details>
<summary>Table of Contents</summary>

- [AssParser](#AssParser)
- [Installation](#Installation)
- [Introduction](#introduction)
- [Example](#example)

</details>



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



[ass_parser]: https://github.com/Aavtic/ass_parser


# Example

Modifying a `.ass` subtitle file.

```rust
use hex_color::HexColor;
use ass_parser;
use ass_parser::{AssFile, V4Format, AssFileOptions};


fn main() {
   let mut ass_file = ass_parser::AssFile::from_file("subtitles.ass".to_string());
   let color  = AssFileOptions::get_ass_color(HexColor::YELLOW);
   ass_file.components.script 
       .set_scripttype("v4.00+".to_string())
       .set_playresx("384".to_string())
       .set_playresy("288".to_string())
       .set_scaledborderandshadow("yes".to_string())
       .set_ycbcr_matrix("None".to_string());

   ass_file.components.v4.set_v4(V4Format::default());

   AssFile::save_file(&ass_file, "modified_subtitles.ass");
}
```
