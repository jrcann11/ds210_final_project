use std::error::Error;
use std::io::Read;
use csv::Reader;

#[derive(Debug)]
pub struct TvShow {
    pub title: String,
    pub netflix: bool,
    pub age: String,
    pub hulu: bool,
    pub prime_video: bool,
    pub disney: bool,
}

pub fn parse_tv_shows<R: Read>(reader: &mut Reader<R>) -> Result<Vec<TvShow>, Box<dyn Error>> {
    let mut tv_shows: Vec<TvShow> = Vec::new();

    for result in reader.records() {
        let record = result?;
        let tv_show = TvShow {
            title: record[2].to_string(),
            age: record[4].to_string(),
            netflix: record[7].parse::<i32>()? == 1,
            hulu: record[8].parse::<i32>()? == 1,
            prime_video: record[9].parse::<i32>()? == 1,
            disney: record[10].parse::<i32>()? == 1,
        };
        tv_shows.push(tv_show);
    }

    Ok(tv_shows)
}