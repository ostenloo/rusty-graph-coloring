use std::env;
use graph_coloring::{input::*,ordering::*}; 

pub enum Test{
    generate_graph(), 
    generate_random(),
    vertex_ordering(), 
    coloring_algorithm(),
}

fn main() {
    let args : Vec<String> = env::args().collect(); 

    let test = &args[1]; 

    match test { 
        "graph" => {
            if args.len() < 3{
                panic!("Wrong number of arguments found."); 
            }
            let graph = &args[2];
            match graph {
                "complete" || "cycle" => {
                    if args.len() != 3 {
                        panic!("Wrong number of arguments found."); 
                    }
                    generate_graph(graph); 
                },
                "random" => {
                    if args.len() != 5 {
                        panic!("Wrong number of arguments found."); 
                    }
                    let dist = &args[3]; 
                    let density = &args[4]; 
                    generate_random(graph, dist, density);
                },
                _ => panic!("Not a graph type."); 
            }
        },
        "order" => {
            if args.len() != 3{
                panic!("Wrong number of arguments found."); 
            }
            let order = &args[2]; 
            match order { 
                "SLVO" || "SODL" || "URO" || "BFSR" || "BFSS" || "BFSR" => {
                    if args.len() != 3 {
                        panic!("Wrong number of arguments found."); 
                    }
                    vertex_ordering(order); 
                }
                _ => panic!("Not a vertex ordering."); 
            }
        },
        "color" => {
            if args.len() < 3{
                panic!("Wrong number of arguments found."); 
            }
            coloring_algorithm 
            
        }
        _ => panic!("Not a valid argument."); 
    }

}

//runtime complexity for 
pub fn generate_graph() 
{

}

//runtime complexity for different sizes of random graphs with various densities 
pub fn generate_random() 
{

}

//
pub fn vertex_ordering() 
{

}

pub fn coloring_algorithm() 
{

}