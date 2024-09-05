use image::ImageError;

// enum for Error handling
#[derive(Debug)]
#[allow(dead_code)]
pub enum ImageDataError{
    DifferentImageFormats,
    BufferTooSmall,
    UnableToOpenImageFromPath(std::io::Error),
    UnableToFormatImage(String),
    UnableToDecodeImage(ImageError),
    UnableToSaveImage(ImageError)
}

// struct to store the merged image
pub struct ImageObject {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub name: String
}

impl ImageObject {
    pub fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = height * width * 4;
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());

        ImageObject {
            width,
            height,
            data: buffer,
            name
        }
    }
    pub fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataError> {
        if data.len() > self.data.capacity() { return Err(ImageDataError::BufferTooSmall);}

        self.data = data;
        Ok(())
    }
}