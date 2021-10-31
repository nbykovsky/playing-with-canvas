use std::collections::{HashMap, HashSet};

pub mod g2d;
pub mod g3d;

pub trait Shape {
    fn approximate(&self) -> Vec<g3d::Triagnle3>;
}

pub struct SetOfTriangles {
    triangles: Vec<g3d::Triagnle3>,
}

impl SetOfTriangles {
    pub fn new(triangles: Vec<g3d::Triagnle3>) -> Self {
        SetOfTriangles { triangles }
    }
}

impl Shape for SetOfTriangles {
    fn approximate(&self) -> Vec<g3d::Triagnle3> {
        self.triangles.clone()
    }
}

pub struct SceneTmp {
    shapes: SetOfTriangles,
}

impl SceneTmp {
    pub fn new(shapes: SetOfTriangles) -> Self {
        Self { shapes }
    }

    fn get_all_triangles(&self) -> Vec<g3d::Triagnle3> {
        let result: Vec<g3d::Triagnle3> = self.shapes.triangles.clone();
        result
    }

    fn build_graph(&self, triangles_repo: &Vec<g3d::Triagnle3>) -> HashMap<usize, HashSet<usize>> {
        let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
        for i in 0..triangles_repo.len() {
            for j in 0..triangles_repo.len() {
                if triangles_repo[i].is_above(&triangles_repo[j]) {
                    let set = graph.entry(j).or_default();
                    (*set).insert(j);
                }
            }
        }
        graph
    }

    fn dfs(
        &self,
        graph: &HashMap<usize, HashSet<usize>>,
        mut order: &mut Vec<usize>,
        mut vis: &mut HashSet<usize>,
        node: usize,
    ) {
        if vis.contains(&node) {
            return;
        }
        vis.insert(node);
        for &next_node in graph.get(&node).unwrap_or(&HashSet::new()).iter() {
            self.dfs(graph, &mut order, &mut vis, next_node);
        }
        order.push(node);
    }

    fn get_ordered_projection(&self) -> Vec<g2d::Triangle2> {
        let triangles_repo = self.get_all_triangles();
        let mut graph = self.build_graph(&triangles_repo);
        let mut order: Vec<usize> = Vec::new();
        let mut vis: HashSet<usize> = HashSet::new();

        for node in 0..triangles_repo.len() {
            self.dfs(&graph, &mut order, &mut vis, node)
        }

        order
            .iter()
            .map(|&idx| triangles_repo.get(idx).unwrap().project())
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect()
    }

    pub fn render(&self) -> *const i32 {
        let mut buf: Vec<i32> = vec![0];
        let triangles = self.get_ordered_projection();

        buf[0] = triangles.len() as i32;

        for tri in triangles.iter() {
            buf.push(tri.p1.x);
            buf.push(tri.p1.y);
            buf.push(tri.p2.x);
            buf.push(tri.p2.y);
            buf.push(tri.p3.x);
            buf.push(tri.p3.y);
        }
        buf.as_ptr()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ordered_triangles() {
        let t1 = g3d::Triagnle3::new(
            g3d::Point3::new(0, 0, 0),
            g3d::Point3::new(-1, 1, 0),
            g3d::Point3::new(1, 1, 0),
        );
        let t2 = g3d::Triagnle3::new(
            g3d::Point3::new(0, 0, 1),
            g3d::Point3::new(-1, 1, 1),
            g3d::Point3::new(1, 1, 1),
        );
        let t3 = g3d::Triagnle3::new(
            g3d::Point3::new(0, 0, -1),
            g3d::Point3::new(-1, 1, -1),
            g3d::Point3::new(1, 1, -1),
        );
        let t4 = g3d::Triagnle3::new(
            g3d::Point3::new(0, 0, -10),
            g3d::Point3::new(-1, -1, -10),
            g3d::Point3::new(1, -1, -10),
        );
        let shapes = SetOfTriangles::new(vec![t1.clone(), t2.clone(), t3.clone(), t4.clone()]);
        let scene = SceneTmp::new(shapes);
        assert_eq!(
            scene.get_ordered_projection(),
            vec![
                t2.project().unwrap(),
                t1.project().unwrap(),
                t3.project().unwrap(),
                t4.project().unwrap()
            ]
        );
    }
}
