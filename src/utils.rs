pub mod image {
    use image::ImageFormat;
    use std::fs;
    use std::io::Cursor;

    pub fn load_image_from_disk(file_name: String) -> String {
        let image_buffer = fs::read(file_name).expect("Unable to read file");
        base64::encode(image_buffer)
        
    }

    pub fn base64_to_png(base64_text: String) -> image::DynamicImage {
        // Decode the base64 text
        let decoded = base64::decode(base64_text).unwrap();

        // Create a cursor from the decoded data
        let cursor = Cursor::new(decoded);

        // Read the image from the cursor
        image::load(cursor, ImageFormat::Png).unwrap()
    }

    pub fn save_image_to_disk(image: image::DynamicImage, file_name: String) {
        image.save(file_name).unwrap();
    }

}
