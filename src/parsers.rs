use super::constants::{self, SECLIST_BASE_URL, USERAGENTS, VHF_DATA, WORDLISTS, WORDLIST_PATHS};
use anyhow::Error;
use regex::Regex;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    sync::Arc,
};

pub struct Wordlist {
    pub path: PathBuf,
}

impl Default for Wordlist {
    fn default() -> Self {
        return Self {
            path: WORDLIST_PATHS[1].to_owned(),
        };
    }
}

impl Wordlist {
    /// Downloads three domain wordlists of different sizes - [SecList's](https://github.com/danielmiessler/SecLists/)
    /// 110k/20k/5k subdomain enum lists - into the respective OS's localdata directory (creating a new vhfuzz
    /// directory if one doesn't exist):
    ///
    ///  |---------------------------------------|------------------------------------|
    ///  | Linux                                 | Windows                            |
    ///  |---------------------------------------|------------------------------------|
    ///  | $HOME/.local/share/vhfuzz/<wordlist>  |  %LOCALAPPDATA%/vhfuzz/<wordlist>  |
    ///  |---------------------------------------|------------------------------------|
    ///
    ///
    pub async fn fetch_seclists(filepath: &PathBuf) -> Result<File, Error> {
        let base_dir = VHF_DATA.to_owned();
        if !base_dir.is_dir() {
            std::fs::create_dir(&base_dir)?;
        }

        for list_name in WORDLISTS.iter() {
            let url = format!("{}/{}", SECLIST_BASE_URL.to_string(), list_name);

            let file = PathBuf::from(&base_dir).join(list_name);
            let mut filewriter = std::fs::File::create(file)?;
            let body = reqwest::get(&*url).await?.bytes().await?;
            filewriter.write_all(&body)?;
        }

        let reader = File::open(filepath)?;
        return Ok(reader);
    }

    pub async fn load_words(filepath: &PathBuf) -> Result<File, Error> {
        let file = if !filepath.exists() {
            if WORDLIST_PATHS.contains(&filepath) {
                Self::fetch_seclists(&filepath).await?
            } else {
                panic!(
                    "[!] Aborting: no readable file found at path '{}'.",
                    filepath.to_str().unwrap_or("[[unreadable filepath]]")
                );
            }
        } else {
            File::open(&filepath)?
        };

        return Ok(file);
    }

    pub async fn from(arg: &str) -> Result<Arc<Vec<String>>, Error> {
        let filepath = constants::index_arg(
            &arg,
            WORDLIST_PATHS
                .iter()
                .map(|v| v.to_str().unwrap())
                .collect::<Vec<&str>>(),
        );

        let mut file = Self::load_words(&PathBuf::from(filepath)).await?;
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)?;
        let result = Arc::new(
            buffer
                .lines()
                .map(|w| w.to_string())
                .collect::<Vec<String>>(),
        );

        return Ok(result);
    }
}

pub struct Agent;
impl Agent {
    pub fn from(arg: &str) -> Result<String, Error> {
        return Ok(constants::index_arg(arg, USERAGENTS.to_vec()));
    }
}

pub struct Url;
impl Url {
    pub fn from(ip_arg: &str, no_tls: bool) -> Result<String, Error> {
        let re = Regex::new(r"(http|https)://(.*?)\.(?:.*)+")?;
        let caps = re.captures(ip_arg);

        let url = match caps {
            Some(_) => {
                if ip_arg.ends_with("/") {
                        String::from(format!("{}", ip_arg))
                    } else {
                        String::from(format!("{}/", ip_arg))
                }
            }
            None => {
                let transformed = match no_tls {
                    true => format!("http://{}/", ip_arg),
                    _ => format!("https://{}/", ip_arg),
                };

                println!(
                    "[*] Auto-transforming IP to target the server at '{}'",
                    transformed
                );

                transformed
            }
        };

        return Ok(url);
    }
}
