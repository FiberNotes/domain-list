use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use build_domains::{cache::Cache, common::GeoSiteList, parse_file::parse_file};
use glob::glob;
use prost::Message;

fn main() -> std::io::Result<()> {
    let mut cache = Cache::new();
    fs::create_dir_all("./dist")?;

    for entry in
        glob("../../aggregate_configs/*").map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
    {
        let path = entry.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let geo_site = parse_file(&path, &mut cache)?;

        let mut geo_site_list = GeoSiteList::default();
        geo_site_list.entry.push(geo_site);

        let pathname = Path::new(&path)
            .file_stem()
            .and_then(|name| name.to_str().map(|s| s.to_string()))
            .unwrap_or_default();

        let mut buf = Vec::new();
        geo_site_list.encode(&mut buf)?;

        let mut file = File::create(format!("./dist/{pathname}.dat"))?;
        file.write_all(&buf)?;
    }

    Ok(())
}
