use benchup_core::{DVec3, Model};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
struct Stage {
    operation: String,
    vertices: usize,
    edges: usize,
    faces: usize,
    holes: usize,
    euler: i64,
    height_cm: f64,
    volume_l: Option<f64>,
    kernel_error: Option<String>,
}

fn cm(x: f64, y: f64, z: f64) -> DVec3 {
    DVec3::new(x, y, z) / 100.0
}

fn rectangle(model: &mut Model, x0: f64, y0: f64, x1: f64, y1: f64, z: f64) -> u64 {
    let points = [
        cm(x0, y0, z),
        cm(x1, y0, z),
        cm(x1, y1, z),
        cm(x0, y1, z),
    ];
    for i in 0..4 {
        model.add_edge(points[i], points[(i + 1) % 4]).unwrap();
    }
    *model.faces.keys().next_back().unwrap()
}

fn report(label: &str, model: &Model, face: u64) -> Stage {
    let vertices = model.vertices.len();
    let edges = model.edges.len();
    let faces = model.faces.len();
    let holes: usize = model.faces.values().map(|f| f.holes.len()).sum();
    let euler = vertices as i64 - edges as i64 + faces as i64 - holes as i64;
    let height_cm = model.faces[&face]
        .vertices
        .iter()
        .map(|id| model.vertices[id].position.z)
        .fold(f64::NEG_INFINITY, f64::max)
        * 100.0;
    let (volume_l, kernel_error) = match model.solid_volume(face) {
        Ok(volume) => (Some(volume * 1000.0), None),
        Err(error) => (None, Some(error.split(';').next().unwrap_or(error).trim().to_string())),
    };
    println!(
        "{:<32}{:>4}{:>4}{:>4}{:>4}{:>9}{:>9}{:>10}",
        label,
        vertices,
        edges,
        faces,
        holes,
        euler,
        format!("{height_cm:.0} cm"),
        volume_l.map(|v| format!("{v:.1} L")).unwrap_or_else(|| "–".to_string()),
    );
    if let Some(error) = &kernel_error {
        println!("{:32}  kernel: {error}", "");
    }
    Stage {
        operation: label.to_string(),
        vertices,
        edges,
        faces,
        holes,
        euler,
        height_cm,
        volume_l,
        kernel_error,
    }
}

fn results_path() -> PathBuf {
    if std::path::Path::new("/work").is_dir() {
        PathBuf::from("/work/results.json")
    } else {
        PathBuf::from("results.json")
    }
}

fn main() {
    let mut model = Model::default();
    let base = rectangle(&mut model, 0., 0., 60., 40., 0.); // 60 x 40 cm on the ground
    println!(
        "{:<32}{:>4}{:>4}{:>4}{:>4}{:>9}{:>9}{:>10}",
        "operation", "V", "E", "F", "H", "V-E+F-H", "height", "volume"
    );
    let mut stages = vec![report("rectangle 60 x 40 cm", &model, base)];

    let top = model.push_pull_along(base, DVec3::Z * 0.30).unwrap(); // up 30 cm
    stages.push(report("push/pull it up 30 cm", &model, top));

    // 1. Pull the top face 10 cm further
    let mut taller = model.clone();
    let face = taller.push_pull_along(top, DVec3::Z * 0.10).unwrap();
    stages.push(report("pull the top up 10 cm", &taller, face));

    // 2. The same pull in Ctrl "new face" mode
    let mut layered = model.clone();
    let cap = layered.push_pull_along_with_new_faces(top, DVec3::Z * 0.10).unwrap();
    stages.push(report("same pull, Ctrl new-face mode", &layered, cap));

    // 3. A smaller rectangle drawn on the top face, pushed 5 cm down
    let mut pocket = model.clone();
    let inner = rectangle(&mut pocket, 10., 10., 50., 30., 30.);
    stages.push(report("rectangle on the top face", &pocket, inner));
    pocket.push_pull_along(inner, DVec3::Z * -0.05).unwrap();
    stages.push(report("push it down 5 cm", &pocket, top));

    let path = results_path();
    std::fs::write(&path, serde_json::to_string_pretty(&stages).unwrap()).unwrap();
    println!("wrote {} stages to {}", stages.len(), path.display());
}
