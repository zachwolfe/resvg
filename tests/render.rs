use once_cell::sync::Lazy;
use rgb::FromSlice;

const IMAGE_SIZE: u32 = 300;

static GLOBAL_OPT: Lazy<std::sync::Mutex<usvg::Options>> = Lazy::new(|| {
    let mut opt = usvg::Options::default();
    opt.font_family = "Noto Sans".to_string();
    opt.fontdb.load_fonts_dir("tests/fonts");
    opt.fontdb.set_serif_family("Noto Serif");
    opt.fontdb.set_sans_serif_family("Noto Sans");
    opt.fontdb.set_cursive_family("Yellowtail");
    opt.fontdb.set_fantasy_family("Sedgwick Ave Display");
    opt.fontdb.set_monospace_family("Noto Mono");
    opt.path = Some(std::path::PathBuf::from("tests/svg/a-alignment-baseline-001.svg"));
    std::sync::Mutex::new(opt)
});

pub fn render(name: &str) -> usize {
    let svg_path = format!("tests/svg/{}.svg", name);
    let png_path = format!("tests/png/{}.png", name);

    // Do not unwrap on the from_file line, because panic will poison GLOBAL_OPT.
    let tree = {
        let tree = usvg::Tree::from_file(&svg_path, &GLOBAL_OPT.lock().unwrap());
        tree.unwrap()
    };

    let img = resvg::render(&tree, usvg::FitTo::Width(IMAGE_SIZE), None).unwrap();

    let expected_data = load_png(&png_path);
    assert_eq!(expected_data.len(), img.data().len());

    let mut pixels_d = 0;
    for (a, b) in expected_data.as_slice().as_rgba().iter().zip(img.data().as_rgba()) {
        if is_pix_diff(*a, *b) {
            pixels_d += 1;
        }
    }

    if pixels_d != 0 {
        // Save diff if needed.
        // img.save_png(&format!("tests/{}.png", name)).unwrap();
        // gen_diff(&name, &expected_data, img.data()).unwrap();
    }

    pixels_d
}

fn load_png(path: &str) -> Vec<u8> {
    let data = std::fs::read(path).unwrap();
    let decoder = png::Decoder::new(data.as_slice());
    let (info, mut reader) = decoder.read_info().unwrap();

    let mut img_data = vec![0; info.buffer_size()];
    reader.next_frame(&mut img_data).unwrap();

    match info.color_type {
        png::ColorType::RGB => {
            panic!("RGB PNG is not supported.");
        }
        png::ColorType::RGBA => {
            img_data
        }
        png::ColorType::Grayscale => {
            let mut rgba_data = Vec::with_capacity(img_data.len() * 4);
            for gray in img_data {
                rgba_data.push(gray);
                rgba_data.push(gray);
                rgba_data.push(gray);
                rgba_data.push(255);
            }

            rgba_data
        }
        png::ColorType::GrayscaleAlpha => {
            let mut rgba_data = Vec::with_capacity(img_data.len() * 2);
            for slice in img_data.chunks(2) {
                let gray = slice[0];
                let alpha = slice[1];
                rgba_data.push(gray);
                rgba_data.push(gray);
                rgba_data.push(gray);
                rgba_data.push(alpha);
            }

            rgba_data
        }
        png::ColorType::Indexed => {
            panic!("Indexed PNG is not supported.");
        }
    }
}

// TODO: remove
fn is_pix_diff(c1: rgb::RGBA8, c2: rgb::RGBA8) -> bool {
    (c1.r as i32 - c2.r as i32).abs() > 1 ||
        (c1.g as i32 - c2.g as i32).abs() > 1 ||
        (c1.b as i32 - c2.b as i32).abs() > 1 ||
        (c1.a as i32 - c2.a as i32).abs() > 1
}

#[allow(dead_code)]
fn gen_diff(name: &str, img1: &[u8], img2: &[u8]) -> Result<(), png::EncodingError> {
    assert_eq!(img1.len(), img2.len());

    let mut img3 = Vec::with_capacity((img1.len() as f32 * 0.75).round() as usize);
    for (a, b) in img1.as_rgba().iter().zip(img2.as_rgba()) {
        if is_pix_diff(*a, *b) {
            img3.push(255);
            img3.push(0);
            img3.push(0);
        } else {
            img3.push(255);
            img3.push(255);
            img3.push(255);
        }
    }

    let path = std::path::PathBuf::from(format!("tests/{}-diff.png", name));
    let file = std::fs::File::create(path)?;
    let ref mut w = std::io::BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, IMAGE_SIZE, IMAGE_SIZE);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&img3)
}