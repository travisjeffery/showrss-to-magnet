use clap::{App, Arg};
use log::info;
use quick_xml::de::from_str;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::{str, thread, time};

#[derive(Debug, Deserialize, PartialEq)]
struct Item {
    title: String,
    link: String,
    guid: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Channel {
    #[serde(rename = "item", default)]
    items: Vec<Item>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Rss {
    channel: Channel,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let matches = App::new("showrss-to-magnet")
        .version("0.1.0")
        .author("Travis Jeffery <tj@travisjeffery.com>")
        .about("Download magnet files from showrss feed")
        .arg(
            Arg::with_name("dst")
                .short("d")
                .long("dst")
                .takes_value(true)
                .default_value("/tmp")
                .env("DST")
                .help("Directory to write magnet files"),
        )
        .arg(
            Arg::with_name("rss")
                .short("r")
                .long("rss")
                .takes_value(true)
                .env("RSS")
                .help("showrss rss url"),
        )
        .arg(
            Arg::with_name("interval")
                .short("i")
                .long("interval")
                .default_value("5")
                .takes_value(true)
                .env("INTERVAL")
                .help("interval to process rss in seconds"),
        )
        .get_matches();

    let dst = matches.value_of("dst").unwrap();
    let dir = Path::new(dst);
    let url = matches.value_of("rss").unwrap();
    let i = matches
        .value_of("interval")
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let interval = time::Duration::from_secs(i);
    let xml = reqwest::blocking::get(url)?.text()?;
    let rss: Rss = from_str(&xml)?;

    info!("running with dst: {}, rss: {}", dst, url);

    loop {
        info!("fetching rss");
        for item in rss.channel.items.iter() {
            let path = dir.join(format!("{}.magnet", item.guid));
            if path.exists() {
                continue;
            }
            let mut file = File::create(&path)?;
            file.write_all(item.link.as_bytes())?;
            info!("wrote: {}", path.display());
        }
        thread::sleep(interval);
    }
}
