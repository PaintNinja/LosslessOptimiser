use anyhow::anyhow;

const STATIC_OXIPNG_OPTS: &str = "-p -b --fix ";

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
        8 => oxipng_options.push_str("--zc 1-9 --zs 0-3 -f 0-5 -a --zw 16k,32k"),
        9 => oxipng_options.push_str("--zc 1-9 --zs 0-3 -f 0-5 -a -Z"),
        10..=std::u8::MAX => return Err(anyhow!("Invalid range, opt_level must be between 0 and 9 inclusive."))
    }

    Ok(oxipng_options)
}
