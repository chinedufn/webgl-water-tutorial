// use bincode;
// use blender_mesh;
// use landon;
// use std::fs::File;
// use std::io::Write;
// use std::process::Command;

// Generates the `meshes.bytes` and `armatures.bytes` files that get included into the wasm
// binary.
//
// The tutorial repository should come with these files so you only need this if you want to
// modify the `.blend` files and then regenerate the `.bytes` files.
//
// So.. basically..:
//  Comment out the contents of the main function if you get compile time errors from it.
//
// You'll need to install `landon` for this to work.
//
// ```
// cargo install -f landon@0.1.2
// landon blender install mesh-to-json
// landon blender install armature-to-json
// ```
fn main() {
    // TODO: This worked in Blender 2.7 but no longer in Blender 2.8. Later versions of landon
    // work with Blender 2.8 - so feel free to adjust this code to a later version of landon.

    // let blender_files = vec!["./terrain.blend".to_string(), "./bird.blend".to_string()];

    // // Only re-run this build script if we change our blender file
    // for blender_file in blender_files.iter() {
    //     println!("{}", format!("cargo:rerun-if-changed={}", blender_file));
    // }

    // // Checks if `blender` is in your $PATH
    // let found_blender_executable = Command::new("command")
    //     .args(&["-v", "blender"])
    //     .output()
    //     .unwrap()
    //     .stdout
    //     .len()
    //     > 0;

    // if !found_blender_executable {
    //     return;
    // }

    // let blender_stdout = landon::export_blender_data(&blender_files).unwrap();

    // let meshes_by_file = blender_mesh::parse_meshes_from_blender_stdout(&blender_stdout).unwrap();
    // let flattened_meshes = blender_mesh::flatten_exported_meshes(&meshes_by_file).unwrap();
    // let flattened_meshes = bincode::serialize(&flattened_meshes).unwrap();

    // let mut f = File::create("./meshes.bytes").unwrap();
    // f.write_all(&flattened_meshes[..]).unwrap();

    // let armatures_by_file =
    //     blender_armature::parse_armatures_from_blender_stdout(&blender_stdout).unwrap();

    // let flattened_armatures =
    //     blender_armature::flatten_exported_armatures(&armatures_by_file).unwrap();

    // let flattened_armatures = bincode::serialize(&flattened_armatures).unwrap();

    // let mut f = File::create("./armatures.bytes").unwrap();
    // f.write_all(&flattened_armatures[..]).unwrap();
}
