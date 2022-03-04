use anyhow::Result;
use image::{GenericImage, GenericImageView, Rgba, RgbaImage};
use imageproc::drawing::{draw_hollow_circle_mut, draw_line_segment_mut, draw_text_mut};
use rusttype::{Font, Scale};

// opentopo/800pixkm/jpg/800-32D.jpg
// 150.000 - 160.000
// 462.500 - 475.000

// 1.25
// 0.0
// 0.0
// -1.25
// 150000.625
// 462499.375
//
//
// 1.25
// 0.0
// 0.0
// -1.25
// 160000.625
// 462499.375

const B: u32 = 800;
const I_W: u32 = 8_000;
const I_H: u32 = 10_000;

const C: [(f32, f32); 38] = [
    (156.905, 449.373),
    (156.706, 449.355),
    (157.134, 449.412),
    (158.019, 449.844),
    (158.029, 450.314),
    (158.554, 450.417),
    (158.487, 450.032),
    (158.577, 449.646),
    (158.978, 449.320),
    (159.357, 448.860),
    (159.134, 448.485),
    (158.760, 448.545),
    (159.137, 448.182),
    (159.580, 448.194),
    (159.911, 447.745),
    (160.367, 447.534),
    (161.033, 447.661),
    (161.227, 446.885),
    (161.572, 446.641),
    (161.900, 446.359),
    (162.103, 447.457),
    (162.273, 446.807),
    (162.269, 446.024),
    (163.007, 447.133),
    (163.274, 445.947),
    (163.686, 445.752),
    (163.870, 445.041),
    (164.630, 446.382),
    (164.338, 445.157),
    (163.720, 444.431),
    (164.660, 444.348),
    (165.674, 444.895),
    (165.767, 444.309),
    (165.893, 444.921),
    (166.585, 443.818),
    (166.717, 442.842),
    (166.036, 443.287),
    (166.069, 443.195),
];

fn main() -> Result<()> {
    let mut img = RgbaImage::new(B * 11, B * 9);

    println!("dimensions {:?}", img.dimensions());

    let img_23d = image::open("opentopo/800pixkm/jpg/800-32D.jpg").unwrap();
    let img_23g = image::open("opentopo/800pixkm/jpg/800-32G.jpg").unwrap();
    let img_39b = image::open("opentopo/800pixkm/jpg/800-39B.jpg").unwrap();
    let img_39e = image::open("opentopo/800pixkm/jpg/800-39E.jpg").unwrap();

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

    for (i, (x, y)) in C.iter().enumerate() {
        let x_pix = (x - 156.0) * (B as f32);
        let y_pix = (9.0 * (B as f32)) - ((y - 442.0) * (B as f32));
        draw_hollow_circle_mut(
            &mut img,
            (x_pix as i32, y_pix as i32),
            50,
            Rgba([255, 0, 0, 255]),
        );
        draw_hollow_circle_mut(
            &mut img,
            (x_pix as i32, y_pix as i32),
            49,
            Rgba([255, 0, 0, 255]),
        );
        draw_hollow_circle_mut(
            &mut img,
            (x_pix as i32, y_pix as i32),
            51,
            Rgba([255, 0, 0, 255]),
        );
        draw_text_mut(
            &mut img,
            Rgba([0, 0, 255, 255u8]),
            x_pix as u32 + 25,
            y_pix as u32 + 25,
            Scale { x: 22.0, y: 22.0 },
            &font,
            &format!("{:02}", i + 1),
        );
    }

    let w = img.width() as f32;
    for x in (0..img.height()).step_by(400) {
        draw_line_segment_mut(
            &mut img,
            (0.0, x as f32),
            (w, x as f32),
            Rgba([255, 0, 0, 255]),
        );
    }

    let h = img.height() as f32;
    for y in (0..img.width()).step_by(400) {
        draw_line_segment_mut(
            &mut img,
            (y as f32, 0.0),
            (y as f32, h),
            Rgba([255, 0, 0, 255]),
        );
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

    // Write the contents of this image to the Writer in PNG format.
    img.save("map.png")?;

    Ok(())
}
