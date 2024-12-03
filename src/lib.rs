pub mod cli;
pub mod day01;

pub mod run {
    use crate::cli::Part;
    use crate::day01;
    pub fn run(data: &str, day: u8, part: Option<Part>) {
        let day_fn = match day {
            1 => day01::day01,
            _ => unimplemented!(),
        };
        day_fn(data, part);
    }
}

pub mod helpers {
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
        format!("data/day{:02}.txt", day)
    }
}
