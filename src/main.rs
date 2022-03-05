use anyhow::Result;
use image::Pixel;
use image::{GenericImage, GenericImageView, Rgba, RgbaImage};
use imageproc::drawing::{draw_hollow_circle_mut, draw_line_segment_mut, draw_text_mut};
use rusttype::{Font, Scale};
use serde::Deserialize;
use std::fmt::{Display, Formatter};

/// pixel per km
const B: u32 = 800;
/// open topo image width
const I_W: u32 = 8_000;
/// open topo image height
const I_H: u32 = 10_000;

const ROOD: Rgba<u8> = Rgba([255, 0, 0, 255]);
const BLAUW: Rgba<u8> = Rgba([0, 0, 255, 255u8]);

#[derive(Debug, Deserialize)]
struct Posten {
    name: String,
    icon: char,
    x_rd: f32,
    y_rd: f32,
    show: bool,
}

fn main() -> Result<()> {
    let mut img = RgbaImage::new(B * 11, B * 9);

    println!("dimensions {:?}", img.dimensions());

    let img_23d = image::open("opentopo/800pixkm/jpg/800-32D.jpg")?;
    let img_23g = image::open("opentopo/800pixkm/jpg/800-32G.jpg")?;
    let img_39b = image::open("opentopo/800pixkm/jpg/800-39B.jpg")?;
    let img_39e = image::open("opentopo/800pixkm/jpg/800-39E.jpg")?;

    let fontbytes = Vec::from(include_bytes!("Ubuntu-Regular.ttf") as &[u8]);
    let size = fontbytes.len();
    let font = Font::try_from_vec(fontbytes).ok_or(anyhow::anyhow!("font error"))?;
    println!("font file size: {}, count={}", size, font.glyph_count());

    let v_23d = img_23d.view(I_W - (B * 4), I_H - B, B * 4, B).to_image();
    img.copy_from(&v_23d, 0, 0)?;

    let v_23g = img_23g.view(0, I_H - B, B * 7, B).to_image();
    img.copy_from(&v_23g, B * 4, 0)?;

    let v_39b = img_39b.view(I_W - (B * 4), 0, B * 4, B * 8).to_image();
    img.copy_from(&v_39b, 0, B)?;

    let v_39e = img_39e.view(0, 0, B * 7, B * 8).to_image();
    img.copy_from(&v_39e, B * 4, B)?;

    let img = raster(img, &font);

    let img = posten(img, &font, false)?;
    // img.save("map.png")?;

    let reoders_a4: Vec<ReOder> = vec![
        ReOder::new((0, 0), (0, 0)),
        ReOder::new((0, 1), (0, 1)),
        ReOder::new((0, 2), (0, 2)),
        ReOder::new((1, 0), (1, 0)),
        ReOder::new((1, 1), (1, 1)),
        ReOder::new((1, 2), (1, 2)),
        ReOder::new((2, 0), (2, 0)),
        ReOder::new((2, 1), (2, 1)),
        ReOder::new((2, 2), (2, 2)),
        ReOder::new((3, 0), (3, 0)),
        ReOder::new((3, 1), (3, 1)),
        ReOder::new((3, 2), (3, 2)),
    ];

    let m = vec![
        (1, 1),
        (2, 1),
        (7, 4),
        (6, 5),
        (4, 2),
        (9, 6),
        (2, 2),
        (8, 5),
        (4, 3),
        (3, 2),
        (9, 4),
        (7, 6),
        (3, 1),
        (6, 4),
        (10, 8),
        (0, 1),
        (10, 7),
        (2, 0),
        (7, 5),
        (10, 6),
        (0, 2),
        (5, 3),
        (1, 0),
        (5, 2),
        (6, 3),
        (1, 2),
        (9, 5),
        (9, 7),
        (7, 3),
        (3, 3),
        (8, 4),
        (8, 7),
        (5, 4),
        (4, 4),
        (6, 2),
        (8, 6),
    ];

    let reoders_1: Vec<ReOder> = vec![
        ReOder::new((1, 1), (0, 0)),
        ReOder::new((2, 1), (0, 1)),
        ReOder::new((7, 4), (0, 2)),
        ReOder::new((6, 5), (1, 0)),
        ReOder::new((4, 2), (1, 1)),
        ReOder::new((9, 6), (1, 2)),
        ReOder::new((2, 2), (2, 0)),
        ReOder::new((8, 5), (2, 1)),
        ReOder::new((4, 3), (2, 2)),
        ReOder::new((3, 2), (3, 0)),
        ReOder::new((9, 4), (3, 1)),
        ReOder::new((7, 6), (3, 2)),
    ];

    let reoders_2: Vec<ReOder> = vec![
        ReOder::new((3, 1), (0, 0)),
        ReOder::new((6, 4), (0, 1)),
        ReOder::new((10, 8), (0, 2)),
        ReOder::new((0, 1), (1, 0)),
        ReOder::new((10, 7), (1, 1)),
        ReOder::new((2, 0), (1, 2)),
        ReOder::new((7, 5), (2, 0)),
        ReOder::new((10, 6), (2, 1)),
        ReOder::new((0, 2), (2, 2)),
        ReOder::new((5, 3), (3, 0)),
        ReOder::new((1, 0), (3, 1)),
        ReOder::new((5, 2), (3, 2)),
    ];

    let reoders_3: Vec<ReOder> = vec![
        ReOder::new((6, 3), (0, 0)),
        ReOder::new((1, 2), (0, 1)),
        ReOder::new((9, 5), (0, 2)),
        ReOder::new((9, 7), (1, 0)),
        ReOder::new((7, 3), (1, 1)),
        ReOder::new((3, 3), (1, 2)),
        ReOder::new((8, 4), (2, 0)),
        ReOder::new((8, 7), (2, 1)),
        ReOder::new((5, 4), (2, 2)),
        ReOder::new((4, 4), (3, 0)),
        ReOder::new((6, 2), (3, 1)),
        ReOder::new((8, 6), (3, 2)),
    ];

    to_a4(&img, &reoders_1, "map_p1.png".to_string())?;
    to_a4(&img, &reoders_2, "map_p2.png".to_string())?;
    to_a4(&img, &reoders_3, "map_p3.png".to_string())?;

    Ok(())
}

fn posten(mut img: RgbaImage, font: &Font, show_all: bool) -> Result<RgbaImage> {
    println!("- posten");
    let data = String::from(include_str!("posten.csv"));

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(data.as_bytes());

    for post in rdr.deserialize() {
        let post: Posten = post?;

        if post.show || show_all {
            let x_pix = (post.x_rd - 156.0) * (B as f32);
            let y_pix = (9.0 * (B as f32)) - ((post.y_rd - 442.0) * (B as f32));
            draw_hollow_circle_mut(&mut img, (x_pix as i32, y_pix as i32), 50, ROOD);
            draw_hollow_circle_mut(&mut img, (x_pix as i32, y_pix as i32), 49, ROOD);
            draw_hollow_circle_mut(&mut img, (x_pix as i32, y_pix as i32), 51, ROOD);
            draw_text_mut(
                &mut img,
                BLAUW,
                x_pix as u32 + 20,
                y_pix as u32 + 20,
                Scale { x: 32.0, y: 32.0 },
                &font,
                &format!("{:02}", post.name),
            );
        }
    }

    Ok(img)
}

fn raster(mut img: RgbaImage, font: &Font) -> RgbaImage {
    println!("- raster");
    let w = img.width() as f32;
    for x in (0..img.height()).step_by(400) {
        draw_line_segment_mut(&mut img, (0.0, x as f32), (w, x as f32), ROOD);
    }

    let h = img.height() as f32;
    for y in (0..img.width()).step_by(400) {
        draw_line_segment_mut(&mut img, (y as f32, 0.0), (y as f32, h), ROOD);
    }

    let scale = Scale { x: 16.0, y: 16.0 };

    for x in (400..img.width()).step_by(400) {
        for y in (400..img.height()).step_by(400) {
            let x_rd = x as f32 / 800.0 + 156.0;
            let x_p = (x_rd - 150.0) * 2.0;
            let y_rd = (img.height() - y) as f32 / 800.0 + 442.0;
            let y_p = (y_rd - 410.0) * 2.0;
            draw_text_mut(
                &mut img,
                Rgba([255, 0, 0, 255u8]),
                x + 1,
                y + 1,
                scale,
                &font,
                &format!("{:2.0},{:2.0}", x_p, y_p),
            );
        }
    }

    img
}

struct ReOder {
    from: (u32, u32),
    to: (u32, u32),
}

impl ReOder {
    const WIT_RUIMTE: u32 = 10;
    const RAND: u32 = 40;

    pub fn new(from: (u32, u32), to: (u32, u32)) -> ReOder {
        ReOder { from, to }
    }

    fn to_size(i: u32) -> u32 {
        B * i + Self::RAND * i * 2 + Self::WIT_RUIMTE * i.saturating_sub(1)
    }

    fn to_pos(i: u32) -> u32 {
        B * i + Self::RAND * ((i * 2) + 1) + Self::WIT_RUIMTE * i
    }

    fn from_x_pix(&self) -> u32 {
        B * self.from.0
    }

    fn from_y_pix(&self) -> u32 {
        B * self.from.1
    }

    fn to_x_pix(&self) -> u32 {
        Self::to_pos(self.to.0)
    }

    fn to_y_pix(&self) -> u32 {
        Self::to_pos(self.to.1)
    }
}

impl Display for ReOder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?}, {:?}) => (({}, {}),({}, {}))",
            self.from,
            self.to,
            self.from_x_pix(),
            self.from_y_pix(),
            self.to_x_pix(),
            self.to_y_pix()
        )
    }
}

fn to_a4(img: &RgbaImage, reoders: &Vec<ReOder>, name: String) -> Result<()> {
    let mut p1 = RgbaImage::new(ReOder::to_size(4), ReOder::to_size(3));

    println!("dimensions p1 {:?}", p1.dimensions());

    for reoder in reoders {
        println!("reoder {}", reoder);
        let blok = img
            .view(reoder.from_x_pix(), reoder.from_y_pix(), B, B)
            .to_image();
        p1.copy_from(&blok, reoder.to_x_pix(), reoder.to_y_pix())?;
    }

    // blend overlay
    let overlay = image::open("map_p1.overlay.png")?;
    println!("dimensions overlay {:?}", overlay.dimensions());
    // p1.copy_from(&overlay, 0, 0)?;
    assert_eq!(overlay.dimensions(), p1.dimensions());
    for y in 0..overlay.height() {
        for x in 0..overlay.width() {
            let p = overlay.get_pixel(x, y);
            p1.get_pixel_mut(x, y).blend(&p);
        }
    }

    p1.save(name)?;

    Ok(())
}
