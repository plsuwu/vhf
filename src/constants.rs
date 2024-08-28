use std::path::PathBuf;

use dirs::data_dir;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SECLIST_BASE_URL: &'static str = "https://raw.githubusercontent.com/danielmiessler/SecLists/master/Discovery/DNS";
    pub static ref VHF_DATA: PathBuf = data_dir().unwrap().join("vhfuzz");
    pub static ref WORDLISTS: [&'static str; 3] = [
        "subdomains-top1million-110000.txt",
        "subdomains-top1million-20000.txt",
        "subdomains-top1million-5000.txt",
    ];

    pub static ref WORDLIST_PATHS: [PathBuf; 3] = [
        VHF_DATA.join(WORDLISTS[0]),
        VHF_DATA.join(WORDLISTS[1]),
        VHF_DATA.join(WORDLISTS[2]),
    ];

    pub static ref USERAGENTS: [&'static str; 3] = [
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.3",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.3",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.3",
    ];

}

// take either the index (0 through 3) and match the arg as an index to the array, or
// take a raw string and use it as a raw arg
pub fn index_arg(arg: &str, arr: Vec<&str>) -> String {
     return match arg.parse::<usize>() {
        Ok(num) => {
            if num < arr.len() {
                arr[num].to_string()
            } else {
                arg.to_string()
            }
        },
        Err(_) => arg.to_string()
     };
}
