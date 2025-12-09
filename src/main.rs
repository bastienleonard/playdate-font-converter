mod ttf;

use std::path::Path;
use std::str::FromStr;

const SPACE_WIDTH: i32 = 6;

struct Args {
    font_path: String,
    font_size: f32
}

struct RenderedChar {
    c: char,
    surface: ttf::Surface
}

fn main() -> Result<(), String> {
    ttf::init()?;
    let args = parse_args()?;

    {
        let font = ttf::open_font(&args.font_path, args.font_size)?;

        let chars_range = ('!' as u32)..=('~' as u32);
        let color = sdl3_sys::pixels::SDL_Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255
        };
        let chars: Vec<RenderedChar> = chars_range.map(|i| {
            let c = char::from_u32(i).unwrap();
            let surface = font.render_shaded(i, color)?;
            Ok(RenderedChar { c, surface })
        }).collect::<Result<Vec<_>, String>>()?;
        let font_stem = Path::new(&args.font_path)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        generate_image(
            &chars,
            font_stem,
            args.font_size
        )?;
        let fnt_path = Path::new(
            &format!("{}-{}", font_stem, args.font_size)
        ).with_added_extension("fnt");
        generate_fnt(&chars, fnt_path.to_str().unwrap())?;
    }

    ttf::quit();
    Ok(())
}

fn parse_args() -> Result<Args, String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        Err("Usage: <program> <font path> <font size>".to_string())
    } else {
        let font_path: String = args[1].to_string();
        let font_size = f32::from_str(&args[2])
            .map_err(|err| err.to_string())?;
        Ok(Args { font_path, font_size })
    }
}

fn generate_image(
    chars: &[RenderedChar],
    font_name: &str,
    font_size: f32
) -> Result<(), String> {
    if chars.is_empty() {
        return Err("chars is empty".to_string());
    }

    let max_width = chars.iter()
        .map(|c| c.surface.width())
        .max()
        .unwrap();
    let max_height = chars.iter()
        .map(|c| c.surface.height())
        .max()
        .unwrap();
    let mut image = ttf::Surface::create(
        max_width * (chars.len() + 1) as i32,
        max_height
    )?;

    for (i, c) in chars.iter().enumerate() {
        c.surface.blit(
            Some(
                sdl3_sys::rect::SDL_Rect {
                    x: 0,
                    y: 0,
                    w: max_width,
                    h: max_height
                }
            ),
            &mut image,
            Some(
                sdl3_sys::rect::SDL_Rect {
                    x: (i as i32 + 1) * max_width,
                    y: 0,
                    w: max_width,
                    h: max_height
                }
            )
        )?;
    }

    let image_path = format!(
        "{}-{}-table-{}-{}.png",
        font_name,
        font_size,
        max_width,
        max_height
    );

    println!("{:?}", image_path);

    if std::fs::exists(&image_path).unwrap() {
        eprintln!("Skipped {}: already exists", image_path);
    } else {
        image.save_png(&image_path)?;
        println!("Wrote {}", image_path);
    }

    Ok(())
}

fn generate_fnt(chars: &[RenderedChar], path: &str) -> Result<(), String> {
    use std::io::Write;

    if std::fs::exists(path).unwrap() {
        eprintln!("Skipped {}: already exists", path);
        return Ok(())
    }

    let file = std::fs::File::create(path)
        .map_err(|err| err.to_string())?;
    let mut writer = std::io::BufWriter::new(file);

    fn write_line<W: std::io::Write>(
        c: &str,
        width: i32,
        writer: &mut std::io::BufWriter<W>
    ) -> Result<(), String> {
        let line = format!("{}	{}\n", c, width);
        writer.write_all(line.as_bytes())
            .map_err(|err| err.to_string())
    }

    write_line("space", SPACE_WIDTH, &mut writer)?;

    for c in chars {
        write_line(&c.c.to_string(), c.surface.width(), &mut writer)?;
    }

    writer.flush().map_err(|err| err.to_string())?;
    println!("Wrote {}", path);
    Ok(())
}
