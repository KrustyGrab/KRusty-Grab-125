use converter_buddy::{format::Format, io::ConvertibleFile};

fn save_convert(src_path: &str, dst_path: &str, src_format: Format, dst_format: Format){
    let file = ConvertibleFile::new(src_path);
    let format = file.format().expect("No format found");

    println!("Found source format: {}", format);
    println!("Converting to {} format", dst_format);

    match file.convert(dst_format) {
        Ok(_) => println!("Conversion successful"),
        Err(e) => println!("Conversion failed: {:?}", e),
    }

}

fn main() {
    let src_path = "assets/Plankton_and_Karen.png";

    save_convert(src_path, src_path, Format::Png, Format::Jpeg);


    // If i have an u8 vector, it can be done as:
    // to check what "screenshots" library gives us

/*    let input = get_input_data();
    let mut output = Vec::<u8>::new();

    PngConverter.process(&input, &mut output, JpegConfig::default()).expect("Conversion error");

    // or in a more generic way
    let source_format = Format::Png;
    let target_format = Format::Jpeg;

    let converter = Converter::try_from(source_format).expect("This format cannot be converted");
    converter.process(&input, &mut output, target_format.into()).expect("Conversion error");

    // use output ...
*/

}