use benchup_core::{DVec3, Model};

fn cm(x: f64, y: f64, z: f64) -> DVec3 {
    DVec3::new(x, y, z) / 100.0
}

fn name(position: DVec3) -> &'static str {
    let p = position * 100.0;
    match (p.x.round() as i64, p.y.round() as i64) {
        (0, 0) => "A",
        (60, 0) => "B",
        (60, 40) => "C",
        (0, 40) => "D",
        (30, 0) => "M",
        (30, 40) => "N",
        _ => "?",
    }
}

fn row(label: &str, model: &Model) {
    println!(
        "{:<34}{:>9}{:>7}{:>7}",
        label,
        model.vertices.len(),
        model.edges.len(),
        model.faces.len()
    );
}

fn main() {
    let (a, b, c, d) = (cm(0., 0., 0.), cm(60., 0., 0.), cm(60., 40., 0.), cm(0., 40., 0.));
    let lines = [(a, b, "A-B"), (b, c, "B-C"), (c, d, "C-D"), (d, a, "D-A")];

    let mut model = Model::default();
    println!("{:<34}{:>9}{:>7}{:>7}", "you draw", "vertices", "edges", "faces");
    for (i, (from, to, name)) in lines.into_iter().enumerate() {
        model.add_edge(from, to).unwrap();
        row(&format!("line {}  {name}", i + 1), &model);
    }

    // M is the middle of A-B, N the middle of C-D
    model.add_edge(cm(30., 0., 0.), cm(30., 40., 0.)).unwrap();
    row("line 5  M-N, across the face", &model);

    println!();
    for face in model.faces.values() {
        let ring: Vec<&str> = face.vertices.iter().map(|id| name(model.vertices[id].position)).collect();
        println!("face {}: {:<14} {:>5.0} cm2", face.id, ring.join(" "), model.face_area(face) * 1e4);
    }
}
