pub mod helpers {
    use std::fs;
    use std::path;

    pub fn load_data(day: u8, part: u8) -> String {
        let local_path = format!("data/day{:02}_part{:02}.txt", day, part);
        if path::Path::new(&local_path).exists() {
            println!("Reading from local file");
            return fs::read_to_string(local_path).expect("Error reading file");
        }
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
        let data = response.text().expect("couldn't get text from response");
        fs::write(local_path, &data).expect("Write data to local cache");
        data
    }
}
