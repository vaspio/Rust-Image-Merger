mod args;
mod image_data;

use args::Args;
use image_data::{ImageDataError, ImageObject};

use image::{imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat};

fn main() -> Result<(), ImageDataError>{
    
    let args = Args::new();
    println!("{:?}",args);

    let (image1, image1_format) = find_image(args.image1)?;
    let (image2, image2_format) = find_image(args.image2)?;

    if image1_format != image2_format {
        return Err(ImageDataError::DifferentImageFormats);
    }

    let (image1, image2) = standardise_size(image1, image2);
    let mut output = ImageObject::new(image1.width(), image1.height(), args.output);

    let combine_data = combine_images(image1, image2);

    output.set_data(combine_data)?;

    if let Err(e) = image::save_buffer_with_format(output.name, &output.data, output.width, output.height, image::ColorType::Rgba8, image1_format) {
        Err(ImageDataError::UnableToSaveImage(e))
    }else{
        Ok(())
    }
}

fn find_image(path: String) -> Result<(DynamicImage, ImageFormat), ImageDataError>{
    // match gia Result kai let some gia Option
    match Reader::open(&path){
        Ok(image_reader) => {

            if let Some(image_format) = image_reader.format(){

                match image_reader.decode(){
                    Ok(image) => Ok((image, image_format)),
                    Err(e) => Err(ImageDataError::UnableToDecodeImage(e))
                }
            }
            else{
                return Err(ImageDataError::UnableToFormatImage(path));
            }

        },
        Err(e) => Err(ImageDataError::UnableToOpenImageFromPath(e))
    }
}

fn get_smallest_image(dim_image1: (u32, u32), dim_image2: (u32, u32)) -> (u32, u32) {
    let pixels_1 = dim_image1.0 * dim_image1.1;
    let pixels_2 = dim_image2.0 * dim_image2.1;

    return if pixels_1 < pixels_2 {dim_image1} else {dim_image2};
}

fn standardise_size(image1: DynamicImage, image2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_image(image1.dimensions(), image2.dimensions());
    println!("width: {}, height: {}", width, height);

    if image2.dimensions() == (width, height) {
        // image 2 is smaller
        (image1.resize_exact(width, height, Triangle),image2)
    }
    else{
        (image1,image2.resize_exact(width, height, Triangle))
    }
}

fn combine_images(image1: DynamicImage, image2: DynamicImage) -> Vec<u8> {
    let vec_img1 = image1.to_rgba8().into_vec();
    let vec_img2 = image2.to_rgba8().into_vec();

    alternate_pixels(vec_img1, vec_img2)
}

fn alternate_pixels(vec1: Vec<u8>, vec2: Vec<u8>) -> Vec<u8> {
    // initialize
    let mut combine_data = vec![0u8; vec1.len()];

    let mut i = 0;

    // for the 1st pixel before modulo works
    combine_data.splice(i..=i+3, set_rgba(&vec1, i, i+3));
    i += 4;

    while i < vec1.len() {

        if i % 8 == 0 { combine_data.splice(i..=i+3, set_rgba(&vec1, i, i+3));} // set pixel from image1
        else{ combine_data.splice(i..=i+3, set_rgba(&vec2, i, i+3));}           // set pixel from image2

        i += 4;
    }
    combine_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();

    for i in start..=end {
        let val = match vec.get(i){
            Some(d) => *d,
            None => panic!("Index is out of bounds")};
        rgba.push(val);
    }
    rgba
}

