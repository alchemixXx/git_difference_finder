use std::{ fs::File, io::{ Error, Write } };

pub fn save_to_file(data: &String, file_path: &String) -> Result<(), Error> {
    let file_result = File::create(file_path);

    let mut file = match file_result {
        Ok(file) => file,
        Err(err) => {
            return Err(err);
        }
    };

    // Write the data to the file
    let write_result = file.write_all(data.as_bytes());

    match write_result {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

// fn create_folder(folder_path: &String) -> Result<(), Error> {
//     let folder_creation_res = fs::create_dir_all(folder_path);
//     let logger = crate::logger::Logger::new();

//     match folder_creation_res {
//         Ok(_) => Ok(()),
//         Err(err) => {
//             if err.kind() == std::io::ErrorKind::AlreadyExists {
//                 logger.warn("Folder already exists");
//                 Ok(())
//             } else {
//                 Err(err)
//             }
//         }
//     }
// }
