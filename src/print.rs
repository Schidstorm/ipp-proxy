use std::path::Path;

use printers;

pub fn print_file(file_path: impl AsRef<Path>) -> Result<(), String> {
    // https://www.bchemnet.com/suldr/index.html
    // driver for Samsung printers
    
    let default_printer = default_printer();
    if default_printer.is_none() {
        return Err("No default printer found".to_string());
    }

    let file_path = file_path.as_ref().to_str().unwrap_or("");
    let res = default_printer.unwrap().print_file(file_path, None);

    if res.is_err() {
        return Err(res.err().unwrap().to_string());
    }
    
    return Ok(());
}

fn default_printer() -> Option<printers::common::base::printer::Printer> {
    for printer in printers::get_printers() {
        if printer.is_default {
            return Some(printer);
        }
    }
    return None;
}