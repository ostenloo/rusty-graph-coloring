//use custom Input type to do error-handling on the I/O 

pub enum InputKind<'a>{ 
    input (Input), 
    file (&'a str), 
}

pub struct Input{
    pub vertices: u32, 
    pub edges: u32,
    pub graph: GraphKind, 
    pub dist: Option<DistKind> 
}

#[derive(Clone)]
pub enum GraphKind{
    complete,
    cycle, 
    random (DistKind), 
}

#[derive(Clone)]
pub enum DistKind{
    uniform, 
    skewed, 
    sine, 
}

impl Input{
    pub fn new(vertices: u32, edges: u32, graph: u32, dist: u32) -> Input {
        if vertices > 10000 {
            panic!("Vertices must be between 0 and 10,000, got {}", vertices); 
        }
        let max_edges = vertices*(vertices-1)/2; 
        if edges > max_edges{
            panic!("Edges must be between 0 and {}, got {}", max_edges, edges); 
        }
        if graph > 3 {
            panic!("Choose 1 for Complete, 2 for Cycle, and 3 for Random"); 
        }  
        let g = match graph { 
            1 => GraphKind::complete,
            2 => GraphKind::cycle, 
            3 => {
                    GraphKind::random(
                        match dist {
                            1 => DistKind::uniform, 
                            2 => DistKind::skewed, 
                            3 => DistKind::sine, 
                            _ => panic!("Choose 1 for Uniform, 2 for Skewed, and 3 for sine"), 
                        }
                    ) 
                },   
            _ => panic!("Choose 1 for Complete, 2 for Cycle, and 3 for Random"),
        };

        let d = match g {
            GraphKind::complete => None, 
            GraphKind::cycle => None,
            GraphKind::random(DistKind::uniform) => Some(DistKind::uniform), 
            GraphKind::random(DistKind::skewed) => Some(DistKind::skewed), 
            GraphKind::random(DistKind::sine) => Some(DistKind::sine), 
            _ => None, 
        };

        Input {
            vertices: vertices, 
            edges: edges, 
            graph: g,  
            dist: d,  
        }
    }

    pub fn max_edges(&self) -> u32 {
        self.vertices*(self.vertices - 1)/2 
    }
}