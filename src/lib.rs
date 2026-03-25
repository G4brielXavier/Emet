pub mod core;


use tequel_rs::hash::TequelHash;

use serde::{ Deserialize, Serialize };
use chrono::{Utc};

use std::{fs};
use std::path::PathBuf;
use std::io::{Error, ErrorKind};
use std::{fs::File, io::Write};
use memmap2::Mmap;


use crate::core::EmetError;


/// Struct that represents the file sealed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmetSeal {

    /// Original Hash before be combined with timestamp and private key
    pub original_hash: String,

    /// Time stamp
    pub timestamp: String,

    /// Tequel's version used 
    pub tequel_version: String,

    /// Digital signature
    pub digital_signature: String

}



pub struct Emet {
    tequel: TequelHash,
    pub private_key: String
}


impl Emet {

    /// 'Up' executes the `TequelHash` and storage the private key
    pub fn up(private_key: String) -> Self {

        let mut teqhash = TequelHash::new();
        let secret = teqhash.tqlhash(private_key.as_bytes());

        Self {
            tequel: teqhash,
            private_key: secret
        }
        
    }


    /// Receive a file content (`&str`), turn to hash with `TequelHash` and combine with private key to create a Hash (signature). It returns a `EmetSeal`
    pub fn seal(&mut self, path: &str) -> Result<EmetSeal, EmetError> {

        let path = PathBuf::from(path);

        if !path.exists() {
            return Err(EmetError::IoError(Error::new(
                ErrorKind::Other,
                "Could not find original path to seal"
            )))   
        }


        let file = fs::File::open(&path).map_err(EmetError::IoError)?;
        
        let mmap = unsafe { Mmap::map(&file).map_err(EmetError::IoError)? };

        let file_hash = self.tequel.tqlhash(&mmap);
        let timestamp = self.get_timestamp();
        
        let mix = &format!("{}{}{}", file_hash, &timestamp, self.private_key);
        let signature = self.tequel.tqlhash(mix.as_bytes());

        Ok(EmetSeal { 
            original_hash: file_hash, 
            timestamp: timestamp, 
            tequel_version: "+0.7.0".to_string(), 
            digital_signature: signature
        })

    }


    /// Receive a original path and `.emet` file path to check if the hash match
    pub fn check(&mut self, path: &str, emet_path: &str) -> Result<(), EmetError> {
        
        let original_path = PathBuf::from(path);
        let emet_path = PathBuf::from(emet_path);

        if !original_path.exists() {
            return Err(EmetError::IoError(Error::new(
                ErrorKind::Other,
                "Could not find original path"
            )))   
        }

        if !emet_path.exists() {
            return Err(EmetError::IoError(Error::new(
                ErrorKind::Other,
                "Could not find .emet path"
            )))   
        }

        // Reading the .emet file to get EmetSeal Struct
        let file_original = fs::File::open(&original_path).map_err(EmetError::IoError)?;
        let mmap_orig = unsafe { Mmap::map(&file_original).map_err(EmetError::IoError)? };
    
        let content_emet = fs::read_to_string(emet_path).map_err(|e| {
            EmetError::IoError(e)
        })?;
        
        let emet_seal_stamped = serde_json::from_str::<EmetSeal>(&content_emet).map_err(|e| {
            EmetError::IoError(e.into())
        })?;
        

        // Integrity

        let orig_file_hash = self.tequel.tqlhash(&mmap_orig);

        if orig_file_hash != emet_seal_stamped.original_hash {
            return Err(EmetError::TruthViolated)
        }

        // Signature Autenticity

        let signature_emet = emet_seal_stamped.digital_signature;
        let timestamp_emet = emet_seal_stamped.timestamp;

        let mix = format!("{}{}{}", orig_file_hash, timestamp_emet, self.private_key);
        let signature_test = self.tequel.tqlhash(&mix.as_bytes());

        if signature_test != signature_emet {
            return Err(EmetError::TruthViolated)
        }

        Ok(())

    }



    /// Receive a path and `EmetSeal` to save the `EmetSeal` in file `.emet`
    pub fn save_seal(&self, path: &str, seal: &EmetSeal) -> Result<(), Box<dyn std::error::Error>> {

        let json = serde_json::to_string_pretty(seal).map_err(|e| {
            e
        })?;

        let seal_path = format!("{}.emet", path);

        let mut file = File::create(seal_path)?;
        
        file.write_all(json.as_bytes()).map_err(|e| {
            e
        })?;

        Ok(())

    }






    
    fn get_timestamp(&self) -> String {
        let now = Utc::now();
        now.to_rfc3339()
    }

}