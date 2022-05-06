use std::{fs, io, io::Read};

use anyhow::{bail, Result};
use clap::{CommandFactory, Parser};
use gtfs_realtime_bindings_transit::{prost::Message, FeedMessage, JsonFormat};
use reqwest::blocking::Client;

mod config;
mod filters;

use config::Config;
use filters::Filter;

fn main() -> Result<()> {
    let cfg = Config::parse();

    // Load message
    let mut message = if let Some(input) = cfg.input {
        // Process input
        if input.starts_with("http") {
            let mut request = Client::new()
                .get(&input)
                .header("Accept", "application/x-protobuf");

            if let Some(headers) = cfg.headers {
                for header in headers {
                    let (key, value) = parse_header(&header)?;
                    request = request.header(key, value);
                }
            }

            let buf = request.send()?.bytes()?;
            FeedMessage::decode(buf)?
        } else {
            let buf = fs::read(&input)?;
            FeedMessage::decode(&buf[..])?
        }
    } else if atty::isnt(atty::Stream::Stdin) {
        // Process stdin
        let stdin = io::stdin();
        let mut buffer = vec![];

        stdin.lock().read_to_end(&mut buffer)?;

        // redis-cli output has extra line feed byte at the end, remove it
        if let Some(0x0A) = buffer.last() {
            buffer.truncate(buffer.len() - 1)
        }

        if buffer.is_empty() {
            bail!("stdin buffer is empty!")
        }

        FeedMessage::decode(&buffer[..])?
    } else {
        Config::command().print_long_help()?;
        bail!("no file, url, or stdin detected!")
    };

    // Filter
    if let Some(route_id) = cfg.route_id {
        message.filter_route(&route_id)
    }

    if let Some(trip_id) = cfg.trip_id {
        message.filter_trip(&trip_id);
    }

    if let Some(stop_id) = cfg.stop_id {
        message.filter_stop(&stop_id);
    }

    // Print
    println!("{}", message.to_json_string_pretty());

    Ok(())
}

fn parse_header(s: &str) -> Result<(&str, &str)> {
    let pair: Vec<&str> = s.split(':').collect();
    match &pair[..] {
        [key, value] => Ok((key, value)),
        _ => bail!("header format is key:value"),
    }
}
