extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate failure;

use failure::Error;
use reqwest::header;
use std::fs::File;
use std::io::prelude::*;

mod api;
mod models;

fn main() -> Result<(), Error> {
    let api = api::API::new("apiKeys.json")?;

    const BUS_POSITIONS_URL: &str = "https://api.wmata.com/Bus.svc/json/jBusPositions";
    const ROUTES_URL: &str = "https://api.wmata.com/Bus.svc/json/jRoutes";
    const STATIONS_URL: &str = "https://api.wmata.com/Rail.svc/json/jSrcStationToDstStationInfo";

    // find routes (routes)
    // find distances (position, schedule, train positions)
    // find prices - hard code for now, requires html parsing of shady sites
    // find timings for distaces (position, schedule)
    // 1km radius per bus stop/ metro station
    let bus_json = &api.get_json(BUS_POSITIONS_URL)?;
    let bus_positions: models::BusPositions = serde_json::from_str(bus_json)?;

    println!("{:#?}", bus_positions);
    let routes: models::Routes = serde_json::from_str(&api.get_json(ROUTES_URL)?)?;
    println!("{:#?}", routes);

    let stations: models::StationToStationInfos =
        serde_json::from_str(&api.get_json(STATIONS_URL)?)?;
    println!("{:#?}", stations);

    Ok(())
}
