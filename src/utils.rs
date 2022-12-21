pub mod image {
    use image::ImageFormat;
    use std::fs;
    use std::io::Cursor;

    pub fn load_image_from_disk(file_name: String) -> String {
        let image_buffer = fs::read(file_name).expect("Unable to read file");
        base64::encode(image_buffer)
        
    }

    pub fn base64_to_png(base64_text: String) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
        // Decode the base64 text
        match base64::decode(base64_text) {
            Ok(decoded) => {
                // Create a cursor from the decoded data
                let cursor = Cursor::new(decoded);
        
                // Read the image from the cursor
                match image::load(cursor, ImageFormat::Png) {
                    Ok(image) => {
                        return Ok(image);
                    },
                    Err(err) => {
                        println!("Error loading image: {:?}", err);
                        return Err(Box::new(err));
                    }
                }
            },
            Err(err) => {
                println!("Error decoding base64: {:?}", err);
                return Err(Box::new(err));
            }
        }

    }

    pub fn save_image_to_disk(image: image::DynamicImage, file_name: String) {
        image.save(file_name).unwrap();
    }

}
