// Utility to generate config/ll1_table.txt
// This creates a human-readable export of the LL(1) parsing table

use interprete_topologias::parser_ll1::LL1Table;
use std::fs;
use std::io::Write;

fn main() {
    println!("Generating LL(1) parsing table...");

    // Create the LL(1) table
    let table = LL1Table::new();

    // Export to human-readable format
    let output = table.export_table();

    // Write to config/ll1_table.txt
    let path = "config/ll1_table.txt";
    match fs::write(path, &output) {
        Ok(_) => {
            println!("✅ Successfully generated {}", path);
            println!("   File size: {} bytes", output.len());
            println!("   Lines: {}", output.lines().count());
        }
        Err(e) => {
            eprintln!("❌ Error writing file: {}", e);
            std::process::exit(1);
        }
    }
}
