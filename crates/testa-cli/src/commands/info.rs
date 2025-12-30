pub fn info_command(extended: bool) {
    println!("TestA - Test Data Generator");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("Authors: {}", env!("CARGO_PKG_AUTHORS"));

    if extended {
        println!("\nSupported formats:\n  - CSV\n");
        println!("Features:");
        println!("  - Template-based data generation");
        println!("  - Type constraints and validation");
        println!("  - Template inheritance");
        println!("  - Enum support with weighted variants");
        println!("  - LSP support for editor integration\n");
        println!("Documentation: https://github.com/lazarnagulov/testa");
    }
}
