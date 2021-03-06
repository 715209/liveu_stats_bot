use error::Error;
use read_input::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::error;

const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Liveu {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Twitch {
    pub bot_username: String,
    pub bot_oauth: String,
    pub channel: String,
    pub commands: Vec<String>,
    pub command_cooldown: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rtmp {
    pub url: String,
    pub application: String,
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub liveu: Liveu,
    pub twitch: Twitch,
    pub rtmp: Option<Rtmp>,
    pub custom_port_names: Option<CustomUnitNames>,
}

impl Config {
    /// Loads the config
    pub fn load<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let config = fs::read_to_string(path)?;
        Ok(serde_json::from_str::<Config>(&config)?)
    }

    /// Asks the user to enter settings and save it to disk
    pub fn ask_for_settings() -> Result<Self, Error> {
        println!("Please enter your Liveu details below");
        let liveu = Liveu {
            email: input().msg("Email: ").get(),
            password: input().msg("Password: ").get(), // FIXME: Change password input?
        };

        println!("\nPlease enter your Twitch details below");
        let mut twitch = Twitch {
            bot_username: input().msg("Bot username: ").get(),
            bot_oauth: input()
                .msg("(You can generate an Oauth here: https://twitchapps.com/tmi/)\nBot oauth: ")
                .get(),
            channel: input().msg("Channel name: ").get(),
            commands: vec![
                "!lustats".to_string(),
                "!liveustats".to_string(),
                "!lus".to_string(),
            ],
            command_cooldown: input()
                .msg("Command cooldown (seconds): ")
                .err("Please enter a number")
                .get(),
        };

        if let Some(oauth) = twitch.bot_oauth.strip_prefix("oauth:") {
            twitch.bot_oauth = oauth.to_string();
        }

        let q: String = input()
            .msg("\nAre you using nginx and would you like to display its bitrate as well (y/n): ")
            .add_test(|x: &String| x.to_lowercase() == "y" || x.to_lowercase() == "n")
            .err("Please enter y or n: ")
            .get();

        let mut rtmp = None;

        if q == "y" {
            rtmp = Some(Rtmp {
                url: input().msg("Please enter the stats page URL: ").get(),
                application: input().msg("Application name: ").get(),
                key: input().msg("Stream key: ").get(),
            });
        }

        let q: String = input()
            .msg("\nWould you like to use a custom name for each port? (y/n): ")
            .add_test(|x: &String| x.to_lowercase() == "y" || x.to_lowercase() == "n")
            .err("Please enter y or n: ")
            .get();

        let mut custom_unit_names = None;

        if q == "y" {
            println!("Press enter to keep using the default value");

            let mut un = CustomUnitNames::default();

            un.ethernet = input().msg("Ethernet: ").default(un.ethernet).get();
            un.wifi = input().msg("WiFi: ").default(un.wifi).get();
            un.usb1 = input().msg("USB1: ").default(un.usb1).get();
            un.usb2 = input().msg("USB2: ").default(un.usb2).get();

            custom_unit_names = Some(un);
        }

        let config = Config {
            liveu,
            twitch,
            rtmp,
            custom_port_names: custom_unit_names,
        };
        fs::write(CONFIG_FILE_NAME, serde_json::to_string_pretty(&config)?)?;

        print!("\x1B[2J");

        let mut path = std::env::current_dir()?;
        path.push(CONFIG_FILE_NAME);
        println!(
            "Saved settings to {} in {}",
            CONFIG_FILE_NAME,
            path.display()
        );

        Ok(config)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomUnitNames {
    pub ethernet: String,
    pub wifi: String,
    pub usb1: String,
    pub usb2: String,
}

impl Default for CustomUnitNames {
    fn default() -> Self {
        CustomUnitNames {
            ethernet: "ETH".to_string(),
            wifi: "WiFi".to_string(),
            usb1: "USB1".to_string(),
            usb2: "USB2".to_string(),
        }
    }
}
