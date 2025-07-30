use cedserver_rs::{LandTile, StaticTile, LandChunk, StaticsChunk};

fn main() {
    let landTile = LandTile::new(0,1,2,3);
    println!("{landTile:#?}");
    let staticTile = StaticTile::new(0,1,2,3,4);
    println!("{staticTile:#?}");

    let landChunk = LandChunk::new(0,0);
    println!("{:#?}", landChunk.tiles[0][0]);

    let mut staticChunk = StaticsChunk::new(0,0,);
    staticChunk.tiles[0][0].push(StaticTile::new(1,1,1,1,1));
    println!("{:#?}", staticChunk.tiles[0][0]);
}
