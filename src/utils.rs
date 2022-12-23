pub mod image {
    use image::ImageFormat;
    use std::fs;
    use std::io::Cursor;

    /// Loads an image from disk and returns it as a base64 encoded string
    ///
    /// # Arguments
    ///
    /// * `file_name` - A string representing the file name of the image to be loaded
    ///
    /// # Returns
    ///
    /// A string of base64 encoded text representing the image
    ///
    /// # Example
    ///
    /// ```
    /// let base64_text = load_image_from_disk("example.png");
    /// ```
    pub fn load_image_from_disk(file_name: String) -> String {
        let image_buffer = fs::read(file_name).expect("Unable to read file");
        base64::encode(image_buffer)
    }

    /// Converts a base64 encoded string to a PNG image
    ///
    /// # Arguments
    ///
    /// * `base64_text` - A string of base64 encoded text
    ///
    /// # Returns
    ///
    /// A `Result` type with an `image::DynamicImage` as the Ok variant and a `Box` type of a `dyn std::error::Error` trait object as the Err variant.
    ///
    /// # Example
    ///
    /// ```
    /// use image;
    ///
    /// let base64_text = "iVBORw0KGg....";
    /// let image = base64_to_png(base64_text).unwrap();
    /// assert_eq!(image.color(), image::ColorType::Rgba8);
    /// ```
    pub fn base64_to_png(
        base64_text: String,
    ) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
        // Decode the base64 text
        match base64::decode(base64_text) {
            Ok(decoded) => {
                // Create a cursor from the decoded data
                let cursor = Cursor::new(decoded);

                // Read the image from the cursor
                match image::load(cursor, ImageFormat::Png) {
                    Ok(image) => Ok(image),
                    Err(err) => {
                        println!("Error loading image: {:?}", err);
                        Err(Box::new(err))
                    }
                }
            }
            Err(err) => {
                println!("Error decoding base64: {:?}", err);
                Err(Box::new(err))
            }
        }
    }

    /// Saves an image to disk
    ///
    /// # Arguments
    ///
    /// * `image` - An `image::DynamicImage` to be saved
    /// * `file_name` - A string representing the file name to save the image as
    ///
    /// # Example
    ///
    /// ```
    /// use image;
    ///
    /// let image = image::open("example.png").unwrap();
    /// save_image_to_disk(image, "saved_image.png");
    /// ```
    pub fn save_image_to_disk(image: image::DynamicImage, file_name: String) {
        image.save(file_name).unwrap();
    }
}
