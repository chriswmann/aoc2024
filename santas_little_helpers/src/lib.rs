use std::fmt;
pub mod error {
    use std::fmt;

    #[derive(Clone, Debug, PartialEq)]
    pub enum AocError {
        FromStrError(String),
        ParseDirectionError(char),
        ParsePointTypeError(char),
    }

    impl fmt::Display for AocError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::FromStrError(e) => writeln!(f, "Couldn't get rule from str, {}", e),
                Self::ParseDirectionError(e) => {
                    writeln!(f, "Couldn't parse direction from char, {}", e)
                }
                Self::ParsePointTypeError(e) => {
                    writeln!(f, "Couldn't parse point type from char, {}", e)
                }
            }
        }
    }
    impl std::error::Error for AocError {}
}

#[derive(Clone, Debug)]
pub enum Part {
    One,
    Two,
}

impl std::convert::TryFrom<u8> for Part {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == Part::One as u8 => Ok(Part::One),
            x if x == Part::Two as u8 => Ok(Part::Two),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Part::One => write!(f, "1"),
            Part::Two => write!(f, "2"),
        }
    }
}

pub mod data {
    use std::fs;
    use std::path;

    pub fn load_data(day: u8) -> String {
        let local_path = get_local_path(day);
        if let Some(data) = try_loading_cached_data(&local_path) {
            return data;
        }
        let data = get_remote_data(day);
        let local_path = get_local_path(day);
        fs::write(local_path, &data).expect("Write data to local cache");
        data
    }

    fn try_loading_cached_data(local_path: &str) -> Option<String> {
        if path::Path::new(local_path).exists() {
            println!("Reading from local file");
            return Some(fs::read_to_string(local_path).expect("Error reading file"));
        }
        None
    }

    fn get_remote_data(day: u8) -> String {
        println!("Reading from adventofcode.com");
        let url = format!("https://adventofcode.com/2024/day/{}/input", day);
        let session_id = std::env::var("AOC_2024_SESSION_ID").expect("AOC_2024_SESSION_ID not set");
        let mut request_headers = reqwest::header::HeaderMap::new();
        let header_string = format!("session={}", session_id);
        request_headers.insert(
            reqwest::header::COOKIE,
            reqwest::header::HeaderValue::from_str(&header_string).unwrap(),
        );

        let client = reqwest::blocking::ClientBuilder::new()
            .default_headers(request_headers)
            .cookie_store(true)
            .build()
            .expect("couldn't build blocking client");
        let response = client.get(&url).send().expect("couldn't send request");
        response.text().expect("couldn't get text from response")
    }

    fn get_local_path(day: u8) -> String {
        format!("cached_data/day{:02}.txt", day)
    }

    pub fn get_day_number(package_name: &str) -> u8 {
        let (_, day_number) = package_name.split_once("day").expect("'day' and number");
        day_number
            .parse::<u8>()
            .expect("Day should have a number with it")
    }
}
