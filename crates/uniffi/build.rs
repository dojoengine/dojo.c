fn main() {
    uniffi::generate_scaffolding("src/dojo.udl").expect("Failed to generate UniFFI scaffolding");
}

