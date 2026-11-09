#![no_main]
#![no_std]

mod file;
mod memmap;

use uefi::{
    Result,
    prelude::*,
    println,
    proto::console::text::{Input, Key, ScanCode},
};

use crate::{file::FileWrapper, memmap::disp_memmap};

fn wait_esc(input: &mut Input) -> Result {
    let events = [input.wait_for_key_event().unwrap()];
    loop {
        boot::wait_for_event(&events).discard_errdata()?;
        if let Some(Key::Special(ScanCode::ESCAPE)) = input.read_key()? {
            break;
        }
    }

    Ok(())
}

fn run() -> Result {
    println!("Welcome to LisbOS bootloader");
    println!();

    disp_memmap();
    let mut file_buffer = [0; 1024];

    let file_name = "test.txt";
    let mut file_wrapper = FileWrapper::new(file_name)?;

    let file_len = file_wrapper.read(&mut file_buffer)?;

    println!(
        "{}: {}",
        file_name,
        str::from_utf8(&file_buffer[..file_len]).unwrap()
    );

    system::with_stdin(wait_esc)?;

    Ok(())
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    match run() {
        Ok(_) => Status::SUCCESS,
        Err(x) => {
            println!("{}", x);
            x.status()
        }
    }
}
