use cairo::surface::Surface;
use cairo::surface::format::Format;
use cairo::{Cairo, Status};
use cairo::font::slant::Slant;
use cairo::font::weight::Weight;

use std::fs;

pub fn create_for(lang: &String) -> Status{
    fs::create_dir("/tmp/i3langlayout");

    let mut image = Surface::create_image(Format::ARGB32, 64, 64);
    let mut g = Cairo::create(&mut image);

    g.set_source_rgba(0., 0., 0., 1.);
    g.rectangle(0., 0., 64., 64.);
    g.fill();

    g.select_font_face("Marker Felt", Slant::Normal, Weight::Normal);
    g.set_font_size(40.);
    g.set_source_rgb(1., 1., 1.);
    g.move_to(7., 43.);
    g.show_text(lang);

    let mut path = "/tmp/i3langlayout/".to_owned();
    path.push_str(lang);
    path.push_str(".png");
    image.write_to_png(&path)
}