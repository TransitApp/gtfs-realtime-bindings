use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "gtfs-rt-printer", about = "Print gtfs-rt")]
pub struct Config {
    /// Input, either url or path
    pub input: Option<String>,

    #[clap(short, long)]
    /// Pass headers to HTTP request, by key:value
    pub headers: Option<Vec<String>>,

    #[clap(short, long)]
    /// Filter trip id
    pub trip_id: Option<String>,

    #[clap(short, long)]
    /// Filter stop id
    pub stop_id: Option<String>,

    #[clap(short, long)]
    /// Filter route_id
    pub route_id: Option<String>,
}
