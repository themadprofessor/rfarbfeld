use nom::Needed;
use nom::Err;

error_chain! {
    foreign_links {
        NomError(Err);
        IoError(::std::io::Error);
    }

    errors {
        NotEnoughDataError(needed: Needed) {
            description("Not enough data to parse!")
            display("Need {} more bytes to successfully parse!", match *needed {
                Needed::Unknown => "Unknown".to_string(),
                Needed::Size(size) => size.to_string()
            })
        }

        InvalidFarbfeldDimensions {
            description("Pixel count doesn't match image dimensions!")
        }
    }
}