use image;
use std::io::prelude::*;
use image::imageops;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const MAX_IMAGE_SIZE: u32 = 1024;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    for elem in args {
        do_stuff(elem.replace("\\", "/"));
    }
}

fn do_stuff(input_path: String) {
    println!("Converting => {:?}", input_path);
    // Open the path in read-only mode, returns `io::Result<File>`
    // Create a path to the desired file
    let path = Path::new(&input_path);
    let display = path.display();

    //Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let metadata = file.metadata().unwrap();
    let filesize = metadata.len() as usize;
    let mut data = vec![0u8; filesize];

    let file_loaded_in_memory = match file.read_exact(&mut data) {
        Err(why) => panic!("couldn't load the file into memory, reason => {}", why),
        Ok(file_loaded_in_memory) => file_loaded_in_memory,
    };

    let image_processed = match image::load_from_memory(&mut data) {
        Err(why) => panic!("couldn't process the image, reason => {}", why),
        Ok(process_image_from_memory) => process_image_from_memory,
    };

    let resized_image = image_processed.resize(
        MAX_IMAGE_SIZE,
        MAX_IMAGE_SIZE,
        imageops::FilterType::Lanczos3,
    );

    let rgba_image = resized_image.to_rgba8();
    
    let mut content  = String::new();

    for height in 0..rgba_image.height() {

        let mut line = String::new();

        let mut prev_color = String::new();

        let mut prev_color_cont = 0;

        for width in 0..rgba_image.width() {

            let px = rgba_image.get_pixel(width, height);
            
            let hex = format!("#{:02x}{:02x}{:02x}", px[0], px[1], px[2]);

            if prev_color == hex {
                prev_color_cont+=1;
            }else{
                if prev_color_cont > 0 {
                    let cell = format!("<td colspan=\"{}\" bgcolor=\"{}\"></td>", prev_color_cont, prev_color);
                    line+=&cell;
                }
                prev_color = hex.clone();
                prev_color_cont = 1;
            }

            // Last pixel bug workaround     
            if width == rgba_image.width() - 1 {
                if prev_color != hex {
                    prev_color_cont = 0;
                    prev_color = hex;
                }

                prev_color_cont += 1;

                let cell = format!("<td colspan=\"{}\" bgcolor=\"{}\"></td>", prev_color_cont, prev_color);
                line+=&cell;
            }
        }

        content+=&format!("<tr>{}</tr>", line);
    }

    let result = format!("<html>
                <body>
                    <table border=\"0\" cellpadding=\"1\" cellspacing=\"0\">
                    {}
                    </table>
                </body>
            </html>", content);
    
    let target_file = format!("{}/{}size{}.html", path.parent().unwrap().to_str().unwrap(), path.file_stem().unwrap().to_str().unwrap(), MAX_IMAGE_SIZE);

    let mut f = File::create(target_file).expect("Unable to create file");
    f.write(result.as_bytes()).expect("Unable to write data");

    println!("Done");
}