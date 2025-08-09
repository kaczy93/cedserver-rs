use cedserver_rs::map::{Map, MapConfig};
use cedserver_rs::server::CedServer;

fn main() {
    let map_config = MapConfig::new(
        0,
        896,
        512,
        r"C:\Nel\Ultima Online Classic_7_0_95_0_modified");
    let map = Map::build(map_config);
    println!("Map initialized!");
    let server = CedServer::run(map);
}
