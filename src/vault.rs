use dirs;
use std::{path::Path, process::{exit, Command}};
use prettytable::{Table, row};

use crate::{account::{read_account, get_account_location}, json::read_json};

// Finds where fmp's vault is
//
// USAGE
//
// let var: String = get_fmp_vault_location();
pub fn get_fmp_vault_location() -> String{
    // Gets users home directory
    let home_dir = dirs::home_dir().expect("Could not find home directory!");
    // Appends directory name to end of home directory
    let fmp_vault_location = home_dir.join(".fmpVault");

    return fmp_vault_location.display().to_string();
}

// Encrypts the .fmpVault file to .fmpVault.tar.gz.gpg
//
// USAGE
//
// encrypt_fmp_vault();
pub fn encrypt_fmp_vault() {
    // Gets locations
    let fmp_vault_location = get_fmp_vault_location();
    let fmp_vault_as_encrypted_tar = format!("{}.tar.gz.gpg", fmp_vault_location);
    let fmp_vault_as_tar = format!("{}.tar.gz", fmp_vault_location);
    
    println!("Encrypting fmp vault...\n");
    if Path::new(&fmp_vault_as_encrypted_tar).exists() {
        Command::new("rm")
            .arg(fmp_vault_as_encrypted_tar.as_str()).output().expect("Could not remove encrypted file");
    }
    // Turns .fmpVault into tarball
    Command::new("tar")
        .args(["-czf", fmp_vault_as_tar.as_str(), fmp_vault_location.as_str()]).output().expect("Failed to execute command");
    // Encrypts vault, handles incorect password
    Command::new("gpg")
        .args(["-c", "--no-symkey-cache", fmp_vault_as_tar.as_str()]).output().expect("Could not encrypt vault, please run fmp -E to encrypt");
    // If pasword is incorrect, tarball is removed
    while Path::new(&fmp_vault_as_encrypted_tar).exists() == false {
        Command::new("rm")
            .arg(fmp_vault_as_tar.as_str()).output().expect("Could not remove file");
        exit(1);
    }
    // Cleanup
    Command::new("rm")
        .args(["-r", fmp_vault_as_tar.as_str()]).output().expect("Could not remove file");
    Command::new("rm")
        .args(["-r", fmp_vault_location.as_str()]).output().expect("Could not remove file");
    println!("\nEncrypted!");
}

// Decrypts the .fmpVault.tar.gz.gpg file to .fmpVault
//
// USAGE
//
// decrypt_fmp_vault();
pub fn decrypt_fmp_vault(){
    let fmp_vault_location = get_fmp_vault_location();
    let fmp_vault_as_encrypted_tar = format!("{}.tar.gz.gpg", fmp_vault_location);
    let fmp_vault_as_tar = format!("{}.tar.gz", fmp_vault_location);
    println!("Decrypting fmp vault...\n");

    // Decrypts vault, handles incorrect password
    Command::new("gpg")
        .args(["-q", "--no-symkey-cache", fmp_vault_as_encrypted_tar.as_str()]).output().expect("Could not encrypt vault");
    // If file has not been decrypted
    if Path::new(&fmp_vault_as_tar).exists() == false{
        println!("Bad decrypt!");
        exit_vault(get_fmp_vault_location());
    }
    // Decrypts tarball
    Command::new("tar")
        .args(["-xf", fmp_vault_as_tar.as_str(), "-C", "/"]).output().expect("Failed to execute command");
    // Removes tarball
    Command::new("rm")
        .arg(fmp_vault_as_tar.as_str()).output().expect("Could not remove tarball vault");
    println!("Decrypted\n");
 
}

// Reads all json files and prints to screen
//
// USAGE
//
// print_vault_entries() 
pub fn print_vault_entries() {
    // Gets list of accounts
    let accounts_list: Vec<String> = read_account(get_account_location());
        // Loop for each entry in accounts_list
        if accounts_list.len() == 0 {
            println!("No accounts have been created! Use fmp -a to create an account.");
            return;
        }
        // Create the table
        let mut table = Table::new();
        table.add_row(row!["Account", "Username", "Password"]);
        for i in 0..accounts_list.len() {
            // Find corrosponding json file and read
            let account = accounts_list[i].clone();
            let json = read_json(get_fmp_vault_location(), account);
            // Output
            table.add_row(row![accounts_list[i], json.username, json.password]);
        }
        table.printstd();
}

// Removes the vault folder
//
// USAGE
//
// delete_vault(get_fmp_vault_location())
pub fn delete_vault(fmp_vault_location: String) {
    if Path::new(&fmp_vault_location).exists() {
        Command::new("rm")
            .args(["-r", fmp_vault_location.as_str()]).output().expect("Could not remove .fmpVault");
    }
}

// Exits the vault and program
//
// USAGE
//
// exit(get_fmp_vault_location())
pub fn exit_vault(fmp_vault_location: String) {
    delete_vault(fmp_vault_location);
    exit(1);
}

// Encrypts the vault any tidy's files up
//
// USAGE
//
// encrtpt_and_exit();
pub fn encrypt_and_exit() {
    encrypt_fmp_vault();
    delete_vault(get_fmp_vault_location());
    exit_vault(get_fmp_vault_location());
}