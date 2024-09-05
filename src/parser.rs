struct SrtFile;

#[derive(Debug, PartialEq)]
pub struct SrtData {
    pub index: String,
    pub start: String,
    pub end: String,
    pub text: String,
}

type Segments = Vec<Vec<String>>;
pub type SrtContent = Vec<SrtData>;


impl SrtData {
    pub fn new() -> SrtData {
        SrtData {
            index: String::new(),
            start: String::new(),
            end:  String::new(),
            text:  String::new(),
        }
    }
}


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

    fn parse_timestamps(&self, timestamps: String) -> [String; 2] {
        let timestamp_splitted: Vec<&str> = timestamps.split(" --> ").collect();
        let timestamps_str: Vec<String> = timestamp_splitted.iter().map(|s| s.to_string()).collect();
        assert_eq!(2, timestamps_str.len());

        let mut start_timestamp = timestamps_str[0].replace(",", ".");
        let mut end_timestamp = timestamps_str[1].replace(",", ".");

        start_timestamp = start_timestamp.chars().skip(1).collect();
        start_timestamp.pop();
        end_timestamp = end_timestamp.chars().skip(1).collect();
        end_timestamp.pop();

        return [start_timestamp, end_timestamp];
    }

    fn get_srt(&self, srt_data: Segments) -> SrtContent {
        let mut text = String::new();
        let mut srt_datas = Vec::<SrtData>::new();
        for data in srt_data{
            let mut srt_data = data.iter();
            let index = srt_data.next().unwrap();
            let timestamps = &self.parse_timestamps(srt_data.next().unwrap().to_string());
            let start = &timestamps[0];
            let end = &timestamps[1];

            for srt_text in srt_data {
                text.push_str(&(srt_text.to_owned() + " "));
            }

            let srt_data = SrtData {
                index: index.to_string(),
                start: start.to_string(),
                end: end.to_string(),
                text: text.clone(),
            };

            srt_datas.push(srt_data);

            text.clear();
        }

        return srt_datas;
    }
}

impl SrtData {
    pub fn parse_srt(&self, contents: String) -> SrtContent{
        // let mut text = String::new();
        let segments = self.get_srt_segments(contents);
        return self.get_srt(segments);
    }
}
