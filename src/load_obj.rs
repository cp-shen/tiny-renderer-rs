pub fn test() {
    use std::path::Path;

    let mod_load_result = tobj::load_obj(
        &Path::new("resources/models/african_head/african_head.obj"));

    assert!(mod_load_result.is_ok());
    let (models, materials) = mod_load_result.unwrap();

    let mesh = &models[0].mesh;
    let i = mesh.indices[0] as usize;
    // pos = [x, y, z]
    let pos = [mesh.positions[i * 3], mesh.positions[i * 3 + 1],
        mesh.positions[i * 3 + 2]];

    if !mesh.normals.is_empty() {
        // normal = [x, y, z]
        let normal = [mesh.normals[i * 3], mesh.normals[i * 3 + 1],
            mesh.normals[i * 3 + 2]];
    }

    if !mesh.texcoords.is_empty() {
        // texcoord = [u, v];
        let texcoord = [mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]];
    }

    println!("test finish!");
}
