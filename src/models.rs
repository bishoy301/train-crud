use chrono::{DateTime, Utc};
use failure::Error;
use postgres::Connection;

mod timestamp {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

trait CRUD {
    fn create_table_if_not_exits(conn: Connection) -> Result<(), Error>;
    fn insert_item(&self, conn: Connection) -> Result<(), Error>;
    // TODO: make optional query params
    fn create_query_string() -> Result<String, Error>;
    fn update(&self, conn: Connection) -> Result<(), Error>;
    fn delete(&self, conn: Connection) -> Result<(), Error>;
}

#[derive(Debug, Deserialize)]
pub struct BusPositions {
    #[serde(rename = "BusPositions")]
    bus_positions: Vec<BusPosition>,
}

#[derive(Debug, Deserialize)]
struct BusPosition {
    #[serde(rename = "DateTime", with = "timestamp")]
    date_time: DateTime<Utc>,
    #[serde(rename = "Deviation")]
    deviation: f32,
    #[serde(rename = "DirectionNum")]
    direction_num: u32,
    #[serde(rename = "DirectionText")]
    direction_text: String,
    #[serde(rename = "Lat")]
    lat: f32,
    #[serde(rename = "Lon")]
    lon: f32,
    #[serde(rename = "RouteID")]
    route_id: String,
    #[serde(rename = "TripEndTime", with = "timestamp")]
    trip_end_time: DateTime<Utc>,
    #[serde(rename = "TripHeadsign")]
    trip_headsign: String,
    #[serde(rename = "TripID")]
    trip_id: String,
    #[serde(rename = "TripStartTime", with = "timestamp")]
    trip_start_time: DateTime<Utc>,
    #[serde(rename = "VehicleID")]
    vehicle_id: String,
}

impl CRUD for BusPositions {
    fn create_table_if_not_exits(conn: Connection) -> Result<(), Error> {
        conn.execute(
            "CREATE TABLE BusPositions (
            
        )",
            &[],
        )?;
        Ok(())
    }
    fn insert_item(&self, conn: Connection) -> Result<(), Error> {
        Ok(())
    }
    // TODO: make optional query params
    fn create_query_string() -> Result<String, Error> {
        let ret = "".to_string();
        Ok(ret)
    }
    fn update(&self, conn: Connection) -> Result<(), Error> {
        Ok(())
    }
    fn delete(&self, conn: Connection) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Routes {
    #[serde(rename = "Routes")]
    routes: Vec<Route>,
}

#[derive(Debug, Deserialize)]
struct Route {
    #[serde(rename = "RouteID")]
    route_id: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "LineDescription")]
    line_description: String,
}

impl CRUD for Routes {
    fn create_table_if_not_exits(conn: Connection) -> Result<(), Error> {
        Ok(())
    }
    fn insert_item(&self, conn: Connection) -> Result<(), Error> {
        Ok(())
    }
    // TODO: make optional query params
    fn create_query_string() -> Result<String, Error> {
        let ret = "".to_string();
        Ok(ret)
    }
    fn update(&self, conn: Connection) -> Result<(), Error> {
        Ok(())
    }
    fn delete(&self, conn: Connection) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct StationToStationInfos {
    #[serde(rename = "StationToStationInfos")]
    station_to_station_info: Vec<StationToStationInfo>,
}

#[derive(Debug, Deserialize)]
pub struct StationToStationInfo {
    #[serde(rename = "CompositeMiles")]
    composite_miles: f32,
    #[serde(rename = "DestinationStation")]
    destination_station: String,
    #[serde(rename = "RailFare")]
    rail_fare: RailFare,
    #[serde(rename = "RailTime")]
    rail_time: u32,
    #[serde(rename = "SourceStation")]
    source_station: String,
}

impl CRUD for StationToStationInfos {
    fn create_table_if_not_exits(conn: Connection) -> Result<(), Error> {
        Ok(())
    }
    fn insert_item(&self, conn: Connection) -> Result<(), Error> {
        Ok(())
    }
    // TODO: make optional query params
    fn create_query_string() -> Result<String, Error> {
        let ret = "".to_string();
        Ok(ret)
    }
    fn update(&self, conn: Connection) -> Result<(), Error> {
        Ok(())
    }
    fn delete(&self, conn: Connection) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct RailFare {
    #[serde(rename = "OffPeakTime")]
    off_peak_time: f32,
    #[serde(rename = "PeakTime")]
    peak_time: f32,
    #[serde(rename = "SeniorDisabled")]
    senior_disabled: f32,
}

#[derive(Debug, Deserialize)]
struct Schedule {
    #[serde(rename = "DirectionNum")]
    direction_num: u32,
    #[serde(rename = "EndTime", with = "timestamp")]
    end_time: DateTime<Utc>,
    #[serde(rename = "RouteID")]
    RouteID: String,
    #[serde(rename = "StartTime", with = "timestamp")]
    start_time: DateTime<Utc>,
    #[serde(rename = "StopTimes")]
    stop_times: Vec<StopTimes>,
    #[serde(rename = "TripDirectionText")]
    trip_direction_text: String,
    #[serde(rename = "TripHeadsign")]
    trip_headsign: String,
    #[serde(rename = "TripID")]
    trip_id: String,
}

#[derive(Debug, Deserialize)]
struct StopTimes {
    #[serde(rename = "StopID")]
    stop_id: String,
    #[serde(rename = "StopID")]
    stop_name: String,
    #[serde(rename = "StopSeq")]
    stop_seq: u32,
    #[serde(rename = "Time", with = "timestamp")]
    time: DateTime<Utc>,
}

impl CRUD for Schedule {
    fn create_table_if_not_exits(conn: Connection) -> Result<(), Error> {
        Ok(())
    }
    fn insert_item(&self, conn: Connection) -> Result<(), Error> {
        Ok(())
    }
    // TODO: make optional query params
    fn create_query_string() -> Result<String, Error> {
        let ret = "".to_string();
        Ok(ret)
    }
    fn update(&self, conn: Connection) -> Result<(), Error> {
        Ok(())
    }
    fn delete(&self, conn: Connection) -> Result<(), Error> {
        Ok(())
    }
}
