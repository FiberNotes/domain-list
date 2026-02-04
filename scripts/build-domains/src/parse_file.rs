use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;

use crate::cache::Cache;
use crate::common::Domain;
use crate::common::GeoSite;
use crate::common::domain;

pub fn parse_file<P: AsRef<Path>>(path: P, cache: &Cache) -> Result<GeoSite, io::Error> {
    cache.get_or_load(path, |path| {
        let mut geo_site = GeoSite::default();
        let file = fs::File::open(path)?;

        geo_site.country_code = Path::new(path)
            .file_stem()
            .and_then(|name| name.to_str().map(|s| s.to_string()))
            .unwrap_or_default();

        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.is_empty() || line.starts_with("#") {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            let first = parts[0];

            // Случай 1: есть тип (include:, full:, domain:)
            if let Some((kind, value)) = first.split_once(':') {
                match kind {
                    "include" => {
                        let file = parse_file(format!("../../domains/{value}"), &cache);

                        match file {
                            Ok(file) => {
                                geo_site.domain.extend(file.domain);
                            }
                            Err(_) => {
                                // Добавить проверку что ошибка вызвана отсутсвием файлы
                                geo_site.domain.extend(
                                    parse_file(format!("../../aggregate_configs/{value}"), &cache)?
                                        .domain,
                                );
                            }
                        }
                    }
                    "full" => {
                        geo_site.domain.push(Domain {
                            r#type: domain::Type::Full.into(),
                            value: value.to_string(),
                            ..Domain::default()
                        });
                    }
                    "keyword" => {
                        geo_site.domain.push(Domain {
                            r#type: domain::Type::Plain.into(),
                            value: value.to_string(),
                            ..Domain::default()
                        });
                    }
                    "domain" => {
                        geo_site.domain.push(Domain {
                            r#type: domain::Type::RootDomain.into(),
                            value: value.to_string(),
                            ..Domain::default()
                        });
                    }
                    _ => {
                        // Handle other cases
                    }
                }
            } else {
                geo_site.domain.push(Domain {
                    r#type: domain::Type::RootDomain.into(),
                    value: first.to_string(),
                    ..Domain::default()
                });
            }
        }

        return Ok(geo_site);
    })
}
