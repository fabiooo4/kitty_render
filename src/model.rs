use crate::{screen::Color, vector::vector3::Vector3};
use std::{fs::File, io::Read};

pub struct Model {
    pub points: Vec<Vector3<f64>>,
    pub face_colors: Vec<Color>,
}

impl Model {
    pub fn new(points: Vec<Vector3<f64>>, face_colors: Vec<Color>) -> Self {
        Model {
            points,
            face_colors,
        }
    }
}

pub fn load_obj(path: &str) -> std::io::Result<Vec<Vector3<f64>>> {
    let mut file = File::open(path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(parse_obj(&content))
}

fn parse_obj(content: &str) -> Vec<Vector3<f64>> {
    let mut vertices: Vec<Vector3<f64>> = Vec::new();
    let mut triangle_points: Vec<Vector3<f64>> = Vec::new();

    content.trim().lines().for_each(|line| {
        if line.trim().starts_with("v ") {
            let vec3: Vec<f64> = line.trim()[2..]
                .split(' ')
                .filter_map(|val| val.parse().ok())
                .collect();

            vertices.push(Vector3::new(vec3[0], vec3[1], vec3[2]));
        }

        if line.trim().starts_with("f ") {
            let face_groups: Vec<&str> = line.trim()[2..].split(' ').collect();

            for (i, &group) in face_groups.iter().enumerate() {
                let group_idx: Vec<usize> = group
                    .trim()
                    .split('/')
                    .filter_map(|val| val.parse().ok())
                    .collect();

                // The first entry of the group is the vertex index (starting from 1)
                if i >= 3 {
                    triangle_points.push(
                        *triangle_points
                            .iter()
                            .nth_back(2)
                            .expect("Failed to read face data"),
                    );

                    triangle_points.push(
                        *triangle_points
                            .iter()
                            .nth_back(1)
                            .expect("Failed to read face data"),
                    );
                }
                triangle_points.push(vertices[group_idx[0] - 1]);
            }
        }
    });

    triangle_points
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_obj() {
        let model = parse_obj(
            "
            v 1.000000 1.000000 -1.000000
            v 1.000000 -1.000000 -1.000000
            v 1.000000 1.000000 1.000000
            v 1.000000 -1.000000 1.000000
            v -1.000000 1.000000 -1.000000
            v -1.000000 -1.000000 -1.000000
            v -1.000000 1.000000 1.000000
            v -1.000000 -1.000000 1.000000
            f 1/1/1 5/2/1 7/3/1 3/4/1
            f 4/5/2 3/4/2 7/6/2 8/7/2
            f 8/8/3 7/9/3 5/10/3 6/11/3
            f 6/12/4 2/13/4 4/5/4 8/14/4
            f 2/13/5 1/1/5 3/4/5 4/5/5
            f 6/11/6 5/10/6 1/1/6 2/13/6
            ",
        );

        let expected_vertices = [
            Vector3::new(1.000000, 1.000000, -1.000000),
            Vector3::new(1.000000, -1.000000, -1.000000),
            Vector3::new(1.000000, 1.000000, 1.000000),
            Vector3::new(1.000000, -1.000000, 1.000000),
            Vector3::new(-1.000000, 1.000000, -1.000000),
            Vector3::new(-1.000000, -1.000000, -1.000000),
            Vector3::new(-1.000000, 1.000000, 1.000000),
            Vector3::new(-1.000000, -1.000000, 1.000000),
        ];

        let expected_vertices = vec![
            expected_vertices[0],
            expected_vertices[4],
            expected_vertices[6],
            expected_vertices[0],
            expected_vertices[6],
            expected_vertices[2],
            expected_vertices[3],
            expected_vertices[2],
            expected_vertices[6],
            expected_vertices[3],
            expected_vertices[6],
            expected_vertices[7],
            expected_vertices[7],
            expected_vertices[6],
            expected_vertices[4],
            expected_vertices[7],
            expected_vertices[4],
            expected_vertices[5],
            expected_vertices[5],
            expected_vertices[1],
            expected_vertices[3],
            expected_vertices[5],
            expected_vertices[3],
            expected_vertices[7],
            expected_vertices[1],
            expected_vertices[0],
            expected_vertices[2],
            expected_vertices[1],
            expected_vertices[2],
            expected_vertices[3],
            expected_vertices[5],
            expected_vertices[4],
            expected_vertices[0],
            expected_vertices[5],
            expected_vertices[0],
            expected_vertices[1],
        ];

        assert_eq!(expected_vertices, model)
    }

    #[test]
    fn test_model() {
        let model = parse_obj(
            "
            o Square
            v -1.000000 -1.000000 1.000000
            v  1.000000 -1.000000 1.000000
            v  1.000000  1.000000 1.000000
            v -1.000000  1.000000 1.000000
            #
            f 1/0/0 2/0/0 3/0/0 4/0/0
            ",
        );

        assert_eq!(
            model,
            vec![
                Vector3::new(-1., -1., 1.),
                Vector3::new(1., -1., 1.),
                Vector3::new(1., 1., 1.),
                Vector3::new(-1., -1., 1.),
                Vector3::new(1., 1., 1.),
                Vector3::new(-1., 1., 1.)
            ]
        )
    }
}
