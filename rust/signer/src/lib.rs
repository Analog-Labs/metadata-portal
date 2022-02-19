use std::process::Command;
use anyhow::{bail};

use definitions::crypto::SufficientCrypto;
use definitions::error::TransferContent;
use parity_scale_codec::Decode;

use generate_message::make_message::make_message;
use generate_message::parser::{Crypto, Goal, Make, Msg};
use qr_reader_pc::{CameraSettings, run_with_camera};
use transaction_parsing::check_signature::pass_crypto;
use app_config::{AppConfig};
use qr_lib::camera::read_qr_movie;
use qr_lib::path::{QrPath};
use qr_lib::read::{hex_to_bytes, read_qr_dir};

mod prompt;
    use crate::prompt::{select_file, want_to_continue};


pub fn full_run(config: AppConfig) -> anyhow::Result<()> {
    let mut files_to_sign: Vec<QrPath> = read_qr_dir(config.qr_dir)?
        .into_iter()
        .filter(|qr| !qr.file_name.is_signed)
        .collect();

    match files_to_sign.len() {
        0 => println!("✔ Nothing to sign"),
        _ => {
            while !files_to_sign.is_empty() {
                let i = select_file(&files_to_sign);
                run_for_file(&files_to_sign.swap_remove(i))?
            }
        },
    }
    Ok(())
}



fn run_for_file(qr_path: &QrPath) -> anyhow::Result<()> {
    open_in_browser(qr_path)?;

    if !want_to_continue() {
        println!("Skipping");
        return Ok(());
    }

    let signature = match run_with_camera(CameraSettings{index: Some(0)}) {
        Ok(line) => line,
        Err(e) => bail!("QR reading error. {}", e),
    };

    sign_qr(qr_path, &signature)?;
    println!("🎉 Signed!");
    Ok(())
}

fn sign_qr(unsigned_qr: &QrPath, signature: &str) -> anyhow::Result<QrPath> {
    let signature = hex_to_bytes(signature)?;
    let sufficient_crypto = <SufficientCrypto>::decode(&mut &signature[..])?;

    let mut signed_qr = unsigned_qr.clone();
    signed_qr.file_name.is_signed = true;

    let raw_read = read_qr_movie(&unsigned_qr.to_path_buf())?;
    let passed_crypto = pass_crypto(&raw_read, TransferContent::LoadMeta)
        .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;

    let make = Make {
        goal: Goal::Qr,
        crypto: Crypto::Sufficient(sufficient_crypto),
        msg: Msg::LoadMetadata(passed_crypto.message),
        name: Some(signed_qr.to_string())
    };
    println!("⚙ generating {}. It takes a while...", signed_qr);
    make_message(make).map_err(anyhow::Error::msg)?;
    Ok(signed_qr)
}



fn open_in_browser(file: &QrPath) -> anyhow::Result<()> {
    let cmd = format!("python -mwebbrowser file://{}", file);
    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()?;
    Ok(())
}