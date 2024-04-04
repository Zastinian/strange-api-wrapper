use reqwest::blocking::Client;

pub fn filter_buffer(api_key: String, filter_type: &str, params: String) -> Vec<u8> {
    let url = format!(
        "https://strangeapi.hostz.me/api/filters/{}?{}",
        filter_type, params
    );
    let client = Client::new();
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .expect("Failed to fetch image");

    response.bytes().expect("Failed to read response").to_vec()
}
