use std::fs::File;
use std::path::Path;
use tiled::parse;
use ggez::ContextBuilder;

fn main() {
    let (mut ctx, _) = ContextBuilder::new("example", "ggez")
        .add_resource_path("assets")
        .build().unwrap();
    let file = ggez::filesystem::open(&mut ctx, &Path::new("tiled_base64_zlib.tmx")).unwrap();
    println!("Opened file");
    let map = parse(&mut ctx, file).unwrap();
    println!("{:?}", map);
    println!("{:?}", map.get_tileset_by_gid(22));
}
