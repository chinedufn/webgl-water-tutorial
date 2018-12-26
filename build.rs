use bincode;
use blender_mesh;
use landon;
use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    let blender_files = vec![
        "./terrain.blend".to_string(),
        "./bird.blend".to_string()
    ];

    // Only re-run this build script if we change our blender file
    for blender_file in blender_files.iter() {
        println!("{}", format!("cargo:rerun-if-changed={}", blender_file));
    }


    // Checks if `blender` is in your $PATH
    let found_blender_executable = Command::new("command")
        .args(&["-v", "blender"])
        .output()
        .unwrap()
        .stdout
        .len()
        > 0;

    if !found_blender_executable {
        return;
    }

    let blender_stdout = landon::export_blender_data(&blender_files).unwrap();

    let meshes_by_file = blender_mesh::parse_meshes_from_blender_stdout(&blender_stdout).unwrap();
    let flattened_meshes = blender_mesh::flatten_exported_meshes(&meshes_by_file).unwrap();
    let flattened_meshes = bincode::serialize(&flattened_meshes).unwrap();

    let mut f = File::create("./meshes.bytes").unwrap();
    f.write_all(&flattened_meshes[..]).unwrap();

    let armatures_by_file =
        blender_armature::parse_armatures_from_blender_stdout(&blender_stdout).unwrap();

    let flattened_armatures =
        blender_armature::flatten_exported_armatures(&armatures_by_file).unwrap();

    let flattened_armatures = bincode::serialize(&flattened_armatures).unwrap();

    let mut f = File::create("./armatures.bytes").unwrap();
    f.write_all(&flattened_armatures[..]).unwrap();
}
