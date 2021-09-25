use image::jpeg::JpegDecoder;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::io::Cursor;

fn main() {
    let (doc, page, layer1) = PdfDocument::new("Test", Mm(327.0), Mm(455.0), "Layer 1");

    let mut rr: Vec<_> = std::fs::read_dir("../")
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap().path().to_str().unwrap().to_owned())
        .filter(|v| v.contains(".jpg"))
        .collect();

    rr.sort_by(|a, b| {
        let sa: isize = a
            .split("../")
            .nth(1)
            .unwrap()
            .split(".jpg")
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let sb: isize = b
            .split("../")
            .nth(1)
            .unwrap()
            .split(".jpg")
            .next()
            .unwrap()
            .parse()
            .unwrap();

        sa.partial_cmp(&sb).unwrap()
    });

    let mut first = true;
    for ii in rr {
        let page = if first {
            page
        } else {
            let (page, _) = doc.add_page(Mm(327.0), Mm(455.0), "Page 2, Layer 1");
            page
        };
        first = false;

        let current_layer = doc.get_page(page).get_layer(layer1);

        let file = std::fs::read(ii).unwrap();
        let mut reader = Cursor::new(file);
        let decoder = JpegDecoder::new(&mut reader).unwrap();
        let image2 = Image::try_from(decoder).unwrap();

        // layer,
        image2.add_to_layer(
            current_layer.clone(),
            None,
            None,
            None,
            None,
            None,
            Some(100.),
        );
    }

    doc.save(&mut BufWriter::new(File::create("test_image.pdf").unwrap()))
        .unwrap();
}
