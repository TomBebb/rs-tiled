use std::fs::File;
use std::path::{Path, PathBuf};
use tiled::{parse, parse_file, parse_tileset, Map, PropertyValue, TiledError};
use ggez::{Context, ContextBuilder};
use std::env;

fn read_from_file(ctx: &mut Context, p: &Path) -> Result<Map, TiledError> {
    let file = ggez::filesystem::open(ctx, p).unwrap();
    return parse(ctx, file);
}

fn read_from_file_with_path(ctx: &mut Context, p: &Path) -> Result<Map, TiledError> {
    return parse_file(ctx, p);
}

fn default_context() -> Context {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        PathBuf::from("./resources")
    };

    let (ctx, _) = ContextBuilder::new("test", "ggez")
        .add_resource_path(&resource_dir)
        .build().unwrap();

    ctx
}

#[test]
fn test_gzip_and_zlib_encoded_and_raw_are_the_same() {
    let mut ctx = default_context();
    let z = read_from_file(&mut ctx, &Path::new("/tiled_base64_zlib.tmx")).unwrap();
    let g = read_from_file(&mut ctx, &Path::new("/tiled_base64_gzip.tmx")).unwrap();
    let r = read_from_file(&mut ctx, &Path::new("/tiled_base64.tmx")).unwrap();
    let c = read_from_file(&mut ctx, &Path::new("/tiled_csv.tmx")).unwrap();
    assert_eq!(z, g);
    assert_eq!(z, r);
    assert_eq!(z, c);
}

#[test]
fn test_external_tileset() {
    let mut ctx = default_context();
    let r = read_from_file(&mut ctx, &Path::new("/tiled_base64.tmx")).unwrap();
    let e = read_from_file_with_path(&mut ctx, &Path::new("/tiled_base64_external.tmx")).unwrap();
    assert_eq!(r, e);
}

#[test]
fn test_just_tileset() {
    let mut ctx = default_context();
    let r = read_from_file(&mut ctx, &Path::new("/tiled_base64.tmx")).unwrap();
    let t = parse_tileset(ggez::filesystem::open(&mut ctx, Path::new("/tilesheet.tsx")).unwrap(), 1).unwrap();
    assert_eq!(r.tilesets[0], t);
}

#[test]
fn test_image_layers() {
    let mut ctx = default_context();
    let r = read_from_file(&mut ctx, &Path::new("/tiled_image_layers.tmx")).unwrap();
    assert_eq!(r.image_layers.len(), 2);
    {
        let first = &r.image_layers[0];
        assert_eq!(first.name, "Image Layer 1");
        assert!(
            first.image.is_none(),
            "{}'s image should be None",
            first.name
        );
    }
    {
        let second = &r.image_layers[1];
        assert_eq!(second.name, "Image Layer 2");
        let image = second
            .image
            .as_ref()
            .expect(&format!("{}'s image shouldn't be None", second.name));
        assert_eq!(image.source, "tilesheet.png");
        assert_eq!(image.width, 448);
        assert_eq!(image.height, 192);
    }
}

#[test]
fn test_tile_property() {
    let mut ctx = default_context();
    let r = read_from_file(&mut ctx, &Path::new("/tiled_base64.tmx")).unwrap();
    let prop_value: String = if let Some(&PropertyValue::StringValue(ref v)) =
        r.tilesets[0].tiles[0].properties.get("a tile property")
    {
        v.clone()
    } else {
        String::new()
    };
    assert_eq!("123", prop_value);
}

#[test]
fn test_object_group_property() {
    let mut ctx = default_context();
    let r = read_from_file(&mut ctx,&Path::new("/tiled_object_groups.tmx")).unwrap();
    let prop_value: bool = if let Some(&PropertyValue::BoolValue(ref v)) = r.object_groups[0]
        .properties
        .get("an object group property")
    {
        *v
    } else {
        false
    };
    assert!(prop_value);
}

#[test]
fn test_flipped_gid() {
    let mut ctx = default_context();
    let r = read_from_file_with_path(&mut ctx, &Path::new("/tiled_flipped.tmx")).unwrap();
    let t1 = r.layers[0].tiles[0][0];
    let t2 = r.layers[0].tiles[0][1];
    let t3 = r.layers[0].tiles[1][0];
    let t4 = r.layers[0].tiles[1][1];
    assert_eq!(t1.gid, t2.gid);
    assert_eq!(t2.gid, t3.gid);
    assert_eq!(t3.gid, t4.gid);
    assert!(t1.flip_d);
    assert!(t1.flip_h);
    assert!(t1.flip_v);
    assert!(!t2.flip_d);
    assert!(!t2.flip_h);
    assert!(t2.flip_v);
    assert!(!t3.flip_d);
    assert!(t3.flip_h);
    assert!(!t3.flip_v);
    assert!(t4.flip_d);
    assert!(!t4.flip_h);
    assert!(!t4.flip_v);
}
