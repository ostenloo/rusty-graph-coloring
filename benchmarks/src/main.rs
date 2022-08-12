use std::{io::*,env,fs,cmp};
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
                "random" => {
                    if args.len() != 6 {
                        panic!("Wrong number of arguments found."); 
                    }
                    let dist = &args[4]; 
                    let density = &args[5]; 
                    match &dist[..] {
                        "uniform" => {
                            generateRandom(directory, DistKind::uniform, density);
                        },
                        "skewed" => {
                            generateRandom(directory, DistKind::skewed, density);
                        },
                        "cosine" => {
                            generateRandom(directory, DistKind::cosine, density);
                        },
                        _ => panic!("Not a distribution type."), 
                    }
                }
                _ => panic!("Not a graph type."),
            }
        },
        "order" => {
            if args.len() != 4{
                panic!("Wrong number of arguments found."); 
            }
            let order = &args[3]; 
            vertexOrdering(directory, order); 
        },
        _ => panic!("Not a valid argument."), 
    }

}

//TO DO -- think about adding an OrderKind? Or just calling the function... 

//move the args.len() outside 

//runtime complexity for various sizes 
pub fn generateGraph(dir : &str, graph : GraphKind)
{
    let sizes : Vec<u32> = vec![100,200,300,400,500,600,700,800,900,1000]; 
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
pub fn generateRandom(dir : &str, dist : DistKind, density : &str)  
{
    let sizes : Vec<u32> = vec![100,200,300,400,500,600,700,800,900,1000]; 
    let mut runtime : Vec<u128> = Vec::new(); 
    fs::create_dir("../tmp/".to_owned() + dir).expect("create failed");
    for v in sizes.iter(){
        let input : Input = Input{
            vertices: *v,
            edges: match &density[..] {
                "sparse" => cmp::max(1,((*v)*(*v-1))/6),
                "dense" => cmp::max(1,((*v)*(*v-1))/3), 
                _ => panic!("Not a density."),
            },
            graph: GraphKind::random(dist.clone()),   
            dist: Some(dist.clone()), 
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

// //
pub fn vertexOrdering(dir : &str, order : &str ) 
{
    let sizes : Vec<u32> = vec![100,200,300,400,500,600,700,800,900,1000]; 
    let mut runtime : Vec<u128> = Vec::new(); 
    let paths = fs::read_dir("../tmp/".to_owned() + dir).unwrap();
    for path in paths {
        let filepath : &str = &format!("{}", path.unwrap().path().display()); 
        if &filepath[dir.len() + 8..dir.len() + 12] == "file"{
            let graph : Graph = Graph::new_from_file(filepath); 
            let start = Instant::now(); 
            let ordering : Ordering = match order{
                "SLVO" => Ordering::SLVO(graph),
                "SODL" => Ordering::SODL(graph),
                "URO" => Ordering::URO(graph),
                "BFSR" => Ordering::BFSR(graph),
                "BFSS" => Ordering::BFSS(graph),
                "BFSL" => Ordering::BFSL(graph),
                _ => panic!("Not an ordering."), 
            };
            let end = start.elapsed();
            println!("{},{}", &filepath[dir.len()+12..], end.as_nanos()); 
            runtime.push(end.as_nanos()); 
        }
    }
}