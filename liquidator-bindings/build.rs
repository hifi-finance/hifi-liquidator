use ethers::contract::Abigen;

// TODO: Figure out how to write the rerun-if-changed script properly
fn main() {
    // Only re-run the builder script if the contract changes
    println!("cargo:rerun-if-changed=./abis/*.json");

    // bindgen("BalanceSheet");
    // bindgen("Fintroller");
    // bindgen("FyToken");
    // bindgen("UniswapV2Pair");
}

#[allow(dead_code)]
fn bindgen(file_name: &str) {
    let bindings = Abigen::new(file_name, format!("./abis/{}.json", file_name))
        .expect("Could not instantiate Abigen")
        .generate()
        .expect("Could not generate bindings");

    bindings
        .write_to_file(format!("./src/{}.rs", file_name.to_lowercase()))
        .expect("Could not write bindings to file");
}
