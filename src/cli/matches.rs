use colored::*;

use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use emet::Emet;
use emet::core::EmetError;

use crate::{cli::commands::Commands, core::Files};


pub fn matches(command: &Commands, emet: &mut Emet, files: &mut Files) -> Result<(), EmetError> {

    match command {

        Commands::Up { private_key, show, change } => {
        
            if let Some(nk) = change {

                if let Ok(key_created) = files.verify_if_has_private_key() {

                    if key_created {

                        emet.private_key = nk.clone();
                        println!("{}", "Don't forget this new key".red());
                        println!("{}", "Private Key Changed!".yellow().bold());

                        files.write(emet.private_key.clone(), &files.emet_path).map_err(|e| {
                            EmetError::IoError(Error::new(ErrorKind::Other, e))
                        })?;


                    } else {

                        println!("");
                        println!("{}", "You don't have a private key already".dimmed());
                        println!("To create use: {}", "emet up <YOUR_PRIVATE_KEY>".yellow().bold());
                        println!("");

                    }

                }

            }

            if let Some(pk) = private_key {

                if let Ok(key_created) = files.verify_if_has_private_key() {
                    
                    if !key_created {

                        emet.private_key = pk.clone();
                        println!("{}", "Don't forget that".red());
                        println!("-- {}", "Private Key Created!".yellow().bold());

                        files.write(emet.private_key.clone(), &files.emet_path).map_err(|e| {
                            EmetError::IoError(Error::new(ErrorKind::Other, e))
                        })?;
                        
                        //

                        files.edit_emet_cfg_prvkey(true).map_err(|e| EmetError::IoError(Error::new(ErrorKind::Other, e)))?;
                    
                    } else {
                        
                        println!("");
                        println!("{}", "Your digital signatures are maked with this key.".dimmed());
                        println!("If you want change (not recommended) use: {}", "emet up --change".yellow().bold());
                        println!("");
                        println!("{}", "All your files signed will be lost".red());
                        println!("");
                    
                    }

                }
                
            
            } else {
                if *show {
                    
                    let content = files.read(&files.emet_path).map_err(|e| {
                        EmetError::IoError(Error::new(ErrorKind::Other, e))
                    })?;

                    if !content.is_empty() {
                        println!("{}", content.yellow().bold());
                    } else {
                        println!("{}", "You didn't create a private key already.".dimmed());
                        println!("Use: {}", "emet up <YOUR_PRIVATE_KEY>".yellow().bold());
                    }

                }
            }

            Ok(())

        },


        Commands::Seal { path } => {
        
            if let Some(p) = path {

                if let Ok(key_created) = files.verify_if_has_private_key() {

                    if key_created {

                        if p.extension().unwrap() == "emet" {
                            println!("{} is already .emet file.", p.display().to_string().yellow());
                            println!("{}", "-- You can't sign a .emet file again.".dimmed());
                            return Err(EmetError::AlreadySigned)
                        }

                        let path_str = p.to_str().ok_or_else(|| EmetError::IoError(Error::new(ErrorKind::Other, "UTF-8 Error")))?;
                        let sealed = emet.seal(path_str).map_err(|e| e)?;

                        let path_to_save = PathBuf::from(path_str);

                        let pts_str = path_to_save.to_str().ok_or_else(|| EmetError::IoError(Error::new(ErrorKind::Other, "UTF-8 Error")))?;

                        emet.save_seal(pts_str, &sealed).map_err(|e| EmetError::IoError(Error::new(ErrorKind::Other, format!("UTF-8 Error -- {}", e))))?;

                        println!("{} was signed!", p.display().to_string().yellow());
                        println!(" -- {}{} created!", p.display().to_string().yellow(), ".emet".yellow());

                    } else {
                        
                        println!("{}", "You didn't set your private key already.".yellow());
                        println!("Use: {}", "emet up <YOUR_KEY>".yellow().bold());

                    }
                
                }

            } else {

                println!("{}", "Path not provided".yellow())

            }

            Ok(())

        }


        Commands::Check { path, emet_path } => {
            
            if let Some(p) = path {
                
                if let Some(ep) = emet_path {

                    if let Ok(key_created) = files.verify_if_has_private_key() {

                        if key_created {

                            let path_str = p.to_str().ok_or_else(|| EmetError::IoError(Error::new(ErrorKind::Other, "UTF-8 Error")))?;
                            let emet_path_str = ep.to_str().ok_or_else(|| EmetError::IoError(Error::new(ErrorKind::Other, "UTF-8 Error")))?;

                            match emet.check(path_str, emet_path_str) {
                                Ok(_) => {
                                    println!("{} {} is {}!", "[emet]".dimmed(), path_str.yellow(), "authentic".yellow().bold())
                                }
                                Err(e) => {
                                    println!("{} {} -- {}", "[emet alert]".dimmed(), emet_path_str.yellow(), e.to_string().red());
                                } 
                            }
                            

                        } else {

                            println!("{}", "You didn't set your private key already.".yellow());
                            println!("Use: {}", "emet up <YOUR_KEY>".yellow());

                        }

                    } else {

                        println!("{}", "You didn't set your private key already.".yellow());
                        println!("Use: {}", "emet up <YOUR_KEY>".yellow());

                    }

                } else {
                    println!("{}", ".emet Path not provided".yellow())
                }

            } else {
                println!("{}", "Path not provided".yellow())
            }

            Ok(())
        
        }

    }

}