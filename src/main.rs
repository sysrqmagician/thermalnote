use anyhow::{Context, Result};
use escpos::{
    driver::UsbDriver,
    printer::Printer,
    utils::{PageCode, Protocol},
};
use std::{
    env,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

const CODE_PAGES: [(&str, PageCode); 38] = [
    ("PC437", PageCode::PC437),
    ("Katakana", PageCode::Katakana),
    ("PC850", PageCode::PC850),
    ("PC860", PageCode::PC860),
    ("PC863", PageCode::PC863),
    ("PC865", PageCode::PC865),
    ("Hiragana", PageCode::Hiragana),
    ("PC851", PageCode::PC851),
    ("PC853", PageCode::PC853),
    ("PC857", PageCode::PC857),
    ("PC737", PageCode::PC737),
    ("ISO8859_7", PageCode::ISO8859_7),
    ("WPC1252", PageCode::WPC1252),
    ("PC866", PageCode::PC866),
    ("PC852", PageCode::PC852),
    ("PC858", PageCode::PC858),
    ("PC720", PageCode::PC720),
    ("WPC775", PageCode::WPC775),
    ("PC855", PageCode::PC855),
    ("PC861", PageCode::PC861),
    ("PC862", PageCode::PC862),
    ("PC864", PageCode::PC864),
    ("PC869", PageCode::PC869),
    ("ISO8859_2", PageCode::ISO8859_2),
    ("ISO8859_15", PageCode::ISO8859_15),
    ("PC1098", PageCode::PC1098),
    ("PC1118", PageCode::PC1118),
    ("PC1119", PageCode::PC1119),
    ("PC1125", PageCode::PC1125),
    ("WPC1250", PageCode::WPC1250),
    ("WPC1251", PageCode::WPC1251),
    ("WPC1253", PageCode::WPC1253),
    ("WPC1254", PageCode::WPC1254),
    ("WPC1255", PageCode::WPC1255),
    ("WPC1256", PageCode::WPC1256),
    ("WPC1257", PageCode::WPC1257),
    ("WPC1258", PageCode::WPC1258),
    ("KZ1048", PageCode::KZ1048),
];

const LICENSE_NOTE: &str = include_str!("../LICENSE");

fn print_codepage_help() {
    println!("THERMALNOTE_CODEPAGE has to be set to one of the following values:");
    let (last, elements) = CODE_PAGES.split_last().unwrap();
    for (element, _) in elements {
        print!("{element}, ");
    }
    println!("{}", last.0);
}

fn main() -> Result<()> {
    if let Some(arg) = env::args().skip(1).next() {
        if arg == "-h" || arg == "-help" || arg == "--help" {
            println!(
                r#"
> ABOUT <

thermalnote - https://github.com/sysrqmagician/thermalnote
Utility for printing quick notes using a thermal printer.

> ENVIRONMENT VARIABLES <

THERMALNOTE_VENDOR   - Vendor ID of your thermal printer (hex without 0x prefix)
                       Example: 0ed6
THERMALNOTE_PRODUCT  - Product ID of your thermal printer (hex without 0x prefix)
                       Example: 06a6
THERMALNOTE_CODEPAGE - Character encoding for text
                       Example: PC437 or WPC1252
EDITOR               - Path to your preferred text editor
                       Example: /usr/bin/nano or /usr/bin/vim

> LICENSE <

{LICENSE_NOTE}

> CODE PAGES <"#
            );
            print_codepage_help();
        }
    }

    let device_id = u16::from_str_radix(
        &env::var("THERMALNOTE_VENDOR").context("reading THERMALNOTE_VENDOR")?,
        16,
    )
    .context("THERMALNOTE_VENDOR must be a hexadecimal number without a prefix")?;
    let product_id = u16::from_str_radix(
        &env::var("THERMALNOTE_PRODUCT").context("reading THERMALNOTE_PRODUCT")?,
        16,
    )
    .context("THERMALNOTE_PRODUCT must be a hexadecimal number without a prefix")?;

    let codepage = match env::var("THERMALNOTE_CODEPAGE") {
        Ok(val) => match CODE_PAGES.iter().find(|x| x.0 == val.to_uppercase()) {
            Some(x) => x,
            None => {
                print_codepage_help();
                return Ok(());
            }
        },
        Err(_) => {
            print_codepage_help();
            return Ok(());
        }
    };

    let mut input_path = std::env::temp_dir();
    input_path.push(format!(
        "thermalnote_{}",
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis()
    ));

    let mut cmd = Command::new(env::var("EDITOR").context("reading EDITOR")?)
        .arg(input_path.as_os_str())
        .spawn()
        .context(
            "Starting editor. Note: EDITOR must be *only* a path. There cannot be any arguments.",
        )?;
    cmd.wait()?;

    let text = std::fs::read_to_string(&input_path).context("reading input file")?;
    let driver = UsbDriver::open(device_id, product_id, None).context("Opening USB device")?;
    Printer::new(driver, Protocol::default(), None)
        .page_code(codepage.1)?
        .init()?
        .write(&text)?
        .feeds(2)?
        .print_cut()?;

    std::fs::remove_file(input_path).context("deleting input file")?;

    Ok(())
}
