struct SrtFile;
struct SrtData {
    index: String,
    start: String,
    end: String,
}




pub fn parse_srt(contents: String) {
    let splitted: Vec<&str> = contents.split("\r\n").collect();
    let mut segments = Vec::new();
    let mut start = false;
    let mut current_buffer = Vec::new();

    for line in splitted {
        println!("line: {}, {}", line, line == "");
        if line == "" {
            if !start {
                start = true;
            } else {
                start = false;
                segments.push(current_buffer);
                current_buffer = Vec::new();
            }
        } else {
            current_buffer.push(line);
        }
    }
    println!("s{:?}e", segments[0]);
}
