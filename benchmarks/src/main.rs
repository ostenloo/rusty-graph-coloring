use std::{io::*,env,fs};
use graph_coloring::{graph::*,input::*,ordering::*}; 
use std::time::{Duration,Instant}; 

fn main() {
    let args : Vec<String> = env::args().collect(); 

    if args.len() < 3 {
        panic!("Wrong number of arguments found."); 
    }

    let command = &args[1]; 
    let directory = &args[2]; 

    match &command[..] { 
        "graph" => {
            if args.len() < 4{
                panic!("Wrong number of arguments found."); 
            }
            let graph = &args[3];
            match &graph[..] {
                "complete" => {
                    if args.len() != 4 {
                        panic!("Wrong number of arguments found."); 
                    }
                    generateGraph(directory, GraphKind::complete); 
                },
                "cycle" => {
                    if args.len() != 4 {
                        panic!("Wrong number of arguments found."); 
                    }
                    generateGraph(directory, GraphKind::cycle); 
                },
                // "random" => {
                //     if args.len() != 6 {
                //         panic!("Wrong number of arguments found."); 
                //     }
                //     let dist = &args[4]; 
                //     let density = &args[5]; 
                //     match dist[..] {
                //         "uniform" => {
                //             generateRandom(directory, GraphKind::random, DistKind::uniform, density);
                //         },
                //         "skewed" => {
                //             generateRandom(directory, GraphKind::random, DistKind::skewed, density);
                //         },
                //         "cosine" => {
                //             generateRandom(directory, GraphKind::random, DistKind::cosine, density);
                //         },
                //         _ => panic!("Not a distribution type."), 
                //     }
                    
                // },
                _ => panic!("Not a graph type."),
            }
        },
        // "order" => {
        //     if args.len() != 4{
        //         panic!("Wrong number of arguments found."); 
        //     }
        //     let order = &args[3]; 
        //     match order[..] { 
        //         "SLVO" | "SODL" | "URO" | "BFSR" | "BFSS" | "BFSL" => {
        //             if args.len() != 4 {
        //                 panic!("Wrong number of arguments found."); 
        //             }
        //             vertexOrdering(order); 
        //         }
        //         _ => panic!("Not a vertex ordering."),
        //     }
        // },
        _ => panic!("Not a valid argument."), 
    }

}

//runtime complexity for various sizes 
pub fn generateGraph(dir : &str, graph : GraphKind)
{
    let sizes : Vec<u32> = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 200];//, 300, 400, 500, 600, 700, 800, 900, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000]; 
    let mut runtime : Vec<u128> = Vec::new(); 
    fs::create_dir("../tmp/".to_owned() + dir).expect("create failed"); 
    for v in sizes.iter(){
        let input : Input = Input{
            vertices: *v,
            edges: match graph {
                GraphKind::complete => (*v)*(*v-1)/2, 
                GraphKind::cycle => *v, 
                _ => 0,
            },
            graph: graph.clone(),  
            dist: None, 
        }; 
        let start = Instant::now(); 
        let graph : Graph = Graph::new(input); 
        let end = start.elapsed(); 
        println!("{}", end.as_nanos()); 
        runtime.push(end.as_nanos()); 
        graph.output(&format!("../tmp/{}/file{}",dir,*v)[..]); 
    }
    let mut runtimeFile = std::fs::File::create("../tmp/".to_owned() + dir + "/runtimeFile").expect("create failed");
    for i in 0..sizes.len() {
        write!(runtimeFile, "{},{}\n", sizes[i], runtime[i]); 
    }
}

//runtime complexity for different sizes of random graphs with various densities 
// pub fn generateRandom() 
// {

// }

// //
// pub fn vertexOrdering() 
// {

// }