use memmap2::Mmap;
use dirs::data_local_dir;

use std::{fmt, path::PathBuf};
use std::fs;
use std::io::Error;
use std::io::ErrorKind;


pub struct Files {
    pub local_path: PathBuf,
    pub emet_path: PathBuf,
    pub emet_cfg_path: PathBuf,
}


#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct EmetCFG {
    pub has_private_key: bool
}


impl Files {


    pub fn new() -> Result<Self, EmetError> {

        let mut local_path = data_local_dir().ok_or_else(|| {
            EmetError::IoError(Error::new(
                ErrorKind::Other,
                "Could not find the system APPDATA directory"
            ))
        })?;

        local_path.push("emet");

        let pri_va_ph = local_path.join("privemetkey.key");
        let emt_cfg_ph = local_path.join("emet.config.em");
        
        Ok(Self {
            local_path: local_path,
            emet_path: pri_va_ph,
            emet_cfg_path: emt_cfg_ph
        })

    }


    pub fn setup(&mut self) -> Result<(), EmetError> {

        fs::create_dir_all(&self.local_path).map_err(|e| EmetError::IoError(e))?;
    
        if !self.emet_path.exists() {
            self.write("".to_string(), &self.emet_path.clone())?;
        }

        if !self.emet_cfg_path.exists() {
            let struct_ctnt = EmetCFG { has_private_key: false };

            let ctnt_str = serde_json::to_string_pretty(&struct_ctnt).map_err(|e| EmetError::IoError(e.into()))?;

            self.write(ctnt_str, &self.emet_cfg_path.clone())?;
        }

        Ok(())

    }


    pub fn write(&self, data: String, path: &PathBuf) -> Result<(), EmetError> {

        fs::write(&path, data).map_err(|e| {
            EmetError::IoError(e)
        })?;

        Ok(())

    }


    pub fn read(&self, path: &PathBuf) -> Result<String, EmetError> {

        if path.exists() {
            let file = fs::File::open(&path).map_err(EmetError::IoError)?;
            let mmap_orig = unsafe { Mmap::map(&file).map_err(EmetError::IoError)? };

            let content = String::from_utf8(mmap_orig.to_vec()).map_err(|e| {
                EmetError::IoError(
                    Error::new(
                        ErrorKind::Other,
                        e
                    )
                )
            })?;

            return Ok(content)
        }        

        return Ok("".to_string())

    }



    pub fn verify_if_has_private_key(&self) -> Result<bool, EmetError> {

        let content = self.read(&self.emet_cfg_path)?;

        let struc_ctnt = serde_json::from_str::<EmetCFG>(&content).map_err(|e| EmetError::IoError(e.into()))?;

        Ok(struc_ctnt.has_private_key)

    }

    pub fn edit_emet_cfg_prvkey(&self, value: bool) -> Result<(), EmetError> {

        let content = self.read(&self.emet_cfg_path)?;

        let mut struct_ctnt = serde_json::from_str::<EmetCFG>(&content).map_err(|e| EmetError::IoError(e.into()))?;

        struct_ctnt.has_private_key = value;

        let str_struct_ctnt = serde_json::to_string_pretty(&struct_ctnt).map_err(|e| EmetError::IoError(e.into()))?;

        self.write(str_struct_ctnt, &self.emet_cfg_path)?;

        Ok(())

    }


}








#[derive(Debug)]
pub enum EmetError {
    TruthViolated,
    ForgedStamp,
    IoError(std::io::Error)
}


impl fmt::Display for EmetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmetError::ForgedStamp => write!(f, "Forged Stamp"),
            EmetError::TruthViolated => write!(f, "Truth violated"),
            EmetError::IoError(e) => write!(f, "Io Error: {}", e),
        }
    }
}

impl std::error::Error for EmetError {}