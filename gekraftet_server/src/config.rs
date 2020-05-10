use std::fs::File;
use std::io::{ BufReader, BufRead, BufWriter, Write, Result as IoResult };
use std::net::{ IpAddr, Ipv4Addr };

const CONFIG_PATH: &'static str = "./server.conf";

pub struct Config {
    pub ip: IpAddr,
    pub players: u32,
    pub port: u16,
}

impl Config {
    pub fn try_read() -> IoResult<Self> {
        if !std::path::Path::new(CONFIG_PATH).exists() {
            Self::default().try_write()?;
            return Ok(Self::default())
        }

        let config_file = File::open(CONFIG_PATH)
            .map(|x| BufReader::new(x))?;
        
        let mut result = Self::default();
        
        for line in config_file.lines() {
            let line = line?;
            let mut values = line.splitn(2, '=');
            let name = values.next().unwrap_or("").trim();
            let value = values.next().unwrap_or("").trim();

            match (name, value) {
                ("ip", ip) => 
                    result.ip = ip.parse().expect("config: invalid ip address"),
                ("players", players) =>
                    result.players = players.parse().expect("config: invalid players count"),
                ("port", port) =>
                    result.port = port.parse().expect("config: invalid port"),
                
                (_, _) => { }, // we ignore them
            }
        }

        Ok(result)
    }

    pub fn try_write(&self) -> IoResult<()> {
        let mut config_file = File::create(CONFIG_PATH)
            .map(|x| BufWriter::new(x))?;
        
        writeln!(config_file, "ip={}", self.ip)?;
        writeln!(config_file, "port={}", self.port)?;
        writeln!(config_file, "players={}", self.players)?;

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip: IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)),
            players: 20,
            port: 25565,
        }
    }
}
