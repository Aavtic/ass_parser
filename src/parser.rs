struct SrtFile;
struct SrtData {
    index: String,
    start: String,
    end: String,
    text: String,
}

type Segments = Vec<Vec<String>>;
type SrtContent = Vec<SrtData>;

impl SrtData {
    fn get_srt_segments(&self, srt_data: String) -> Segments {
        let splitted = srt_data.lines();
        let mut segments = Vec::new();
        let mut current_buffer: Vec<String> = Vec::new();

        for line in splitted {
            // println!("line: {}, {}", line, line.is_empty());
            if line.is_empty() {
                    segments.push(current_buffer);
                    current_buffer = Vec::new();
            } else {
                current_buffer.push(line.to_owned());
            }
        }

        return segments;
    }

    fn parse_timestamps(&self, timestamps: String) -> Vec<String> {
        let timestamp_splitted: Vec<&str> = timestamps.split(" --> ").collect();
        let timestamps_str: Vec<String> = timestamp_splitted.iter().map(|s| s.to_string()).collect();
        assert_eq!(2, timestamps.len());

        return timestamps_str;
    }

    fn get_srt(&self, srt_data: Segments) -> SrtContent {
        let mut text = String::new();
        for data in srt_data{
            let mut srt_data = data.iter();
            let index = srt_data.next().unwrap();
            let timestamps = &self.parse_srt(srt_data.next().unwrap().to_string());

            for srt_text in srt_data {
                text.push_str(srt_text.as_str());
            }

            let srt_data = SrtData {
                index: index,

            }

        }
    }
}

impl SrtData {
    pub fn parse_srt(&self, contents: String) {
        let mut text = String::new();
        let segments = self.get_srt_segments(contents);

    }
}
