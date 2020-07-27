use anyhow::anyhow;

const STATIC_OXIPNG_OPTS: &str = "-p -b --fix ";
const STATIC_ECT_OPTS: &str = "--strict -progressive --mt-deflate --reuse -keep ";

// todo: dynamically adjust settings based on input file size and file type

pub async fn get_oxipng_options(opt_level: u8) -> Result<String, anyhow::Error> {
    let mut oxipng_options = String::from(STATIC_OXIPNG_OPTS);
    match opt_level {
        0 => oxipng_options.push_str("-o 0"),
        1 => oxipng_options.push_str("-o 1"),
        2 => oxipng_options.push_str("--zc 9 --zs 0-3 -f 0,5 --nz"),
        3 => oxipng_options.push_str("-o 2"),
        4 => oxipng_options.push_str("-o 3"),
        5 => oxipng_options.push_str("--zc 9 --zs 0-3 -f 0-5 -a"),
        6 => oxipng_options.push_str("-o 5"),
        7 => oxipng_options.push_str("--zc 3-9 --zs 0-3 -f 0-5 -a"),
        8 | 9 => oxipng_options.push_str("--zc 1-9 --zs 0-3 -f 0-5 -a"),
        //9 => oxipng_options.push_str("-f 0-5 -a -Z"), not needed as ECT can sometimes outperform Zopfli in compression ratio while being much faster
        10..=std::u8::MAX => {
            return Err(anyhow!(
                "Invalid range, opt_level must be between 0 and 9 inclusive."
            ))
        }
    }

    Ok(oxipng_options)
}

pub async fn get_ect_options(opt_level: u8) -> Result<String, anyhow::Error> {
    let mut ect_options = String::from(STATIC_ECT_OPTS);
    match opt_level {
        0 => ect_options.push_str("-1"),
        1 => ect_options.push_str("-1"),
        2 => ect_options.push_str("-2"),
        3 => ect_options.push_str("-3"),
        4 => ect_options.push_str("-4"),
        5 => ect_options.push_str("-5"),
        6 => ect_options.push_str("-6"),
        7 => ect_options.push_str("-7"),
        8 => ect_options.push_str("-8"),
        9 => ect_options.push_str("-9"),
        10..=std::u8::MAX => {
            return Err(anyhow!(
                "Invalid range, opt_level must be between 0 and 9 inclusive."
            ))
        }
    }

    Ok(ect_options)
}
