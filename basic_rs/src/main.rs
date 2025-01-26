use gst::{ffi::gst_parse_launch, prelude::*};

fn main() {
    gst::init()?;

    let pipeline = gst::Pipeline::new();

    let playbin = gst::ElementFactory::make("playbin")
        .property("uri", "https://gstreamer.freedesktop.org/data/media/sintel_trailer-480p.webm")
        .build()
        .unwrap();

    pipeline.add_many([]);

    
    // let pipeline = gst_parse_launch(
    //     "playbin uri=https://gstreamer.freedesktop.org/data/media/sintel_trailer-480p.webm",
    //     None)?
}
