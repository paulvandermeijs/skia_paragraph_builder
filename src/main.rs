use std::{fs::File, io::Write};

use skia_safe::{
    textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle},
    Color, ColorSpace, FontMgr, Paint, Surface,
};

fn main() {
    let mut surface = Surface::new_raster_n32_premul((600, 600)).expect("No canvas");
    surface.canvas().clear(Color::WHITE);

    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean sapien odio, feugiat nec felis in, tincidunt dignissim enim. Pellentesque venenatis scelerisque finibus. Nam ut ex dictum, finibus lacus eu, tincidunt magna. Pellentesque orci nisi, vestibulum et libero ac, consectetur mattis sem. Nunc tincidunt et risus vitae viverra. Vestibulum diam massa, aliquet a nunc rutrum, viverra mattis diam. Cras laoreet tortor accumsan leo posuere venenatis.";

    let mut text_style = TextStyle::new();

    text_style
        .set_color(Color::BLACK)
        .set_font_size(32.0)
        .set_background_color(&Paint::new(
            skia_safe::colors::MAGENTA,
            Some(&ColorSpace::new_srgb()),
        ));

    let mut paragraph_style = ParagraphStyle::new();

    paragraph_style.set_text_style(&text_style);

    let mut font_collection = FontCollection::new();

    font_collection.set_default_font_manager(FontMgr::default(), "sans-serif");

    let mut builder = ParagraphBuilder::new(&paragraph_style, font_collection);

    builder.add_text(text);

    let mut paragraph = builder.build();

    paragraph.layout(540.0);

    println!("{}", paragraph.height());

    paragraph.paint(surface.canvas(), (16, 16));

    let image = surface.image_snapshot();

    let data = image
        .encode_to_data(skia_safe::EncodedImageFormat::PNG)
        .unwrap();

    let mut file = File::create("test.png").unwrap();
    let bytes = data.as_bytes();
    file.write_all(bytes).unwrap();
}
