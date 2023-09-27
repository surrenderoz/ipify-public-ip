//! # ipfy-public-ip
//!
//! `ipfy-public-ip` is providing the public ip using IPIFY

//!
//! # Examples
//!
//! ```
//! let response = ipfy-public-ip::find_ip();
//! println!("", response) 
//! print results 83.83.83.83
//!
//! ```


use reqwest;

#[tokio::main]
pub async fn find_ip() -> String {
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
