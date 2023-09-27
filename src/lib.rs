use reqwest;

#[tokio::main]
async fn find_ip() -> String {
    let response = reqwest::get("https://api64.ipify.org/").await;
    let result = match response {
        Ok(res) => res.text().await.unwrap(),
        Err(_) => "error".to_owned(),
    };
    result
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let result = find_ip();
        assert_ne!(result, "error")
    }
}
