use std::io;
use std::fs;

use graph_coloring::input::*; 
use graph_coloring::graph::*; 
use graph_coloring::ordering::*;

fn main() {
    // let contents = fs::read_to_string("file.txt").expect("Something went wrong reading the file");
    // let graph : Graph = Graph::new_from_file("file.txt");
    // graph.display();  
    // let ordering : Ordering = Ordering::BFSS(graph); 
    // ordering.displayOrder(); 
    // ordering.coloring(); 
    // graph.output("file1.txt"); 
    loop 
    {
        let (mut terminate, mut vertices, mut edges, mut graph, mut distribution) = 
        (String::new(), String::new(), String::new(), String::new(), String::new());

        println!("Welcome to the Rusty Graph Coloring Analysis Project! Press any key to continue, or 'q' to quit.");
        io::stdin().read_line(&mut terminate).expect("failed to readline");
        terminate = terminate.trim().to_string(); 
        if terminate == "q"
        {
            println!("Thanks for using the Rusty Graph Coloring Analysis Project! Have a nice day!"); 
            break; 
        }
        
        println!("Choose the type of Graph, 1 for Complete, 2 for Cycle, or 3 for Random:");
        io::stdin().read_line(&mut graph).expect("failed to readline"); 
        //The use of .unwrap() is to "catch" the potential error and fail at this point.
        let mut g : u32 = graph.trim().parse::<u32>().unwrap(); 

        println!("Choose the number of Vertices (1-10,000):");
        io::stdin().read_line(&mut vertices).expect("failed to readline");
        let mut v : u32 = vertices.trim().parse::<u32>().unwrap();  
        let mut e : u32 = 0; 
        let mut d : u32 = 0; 

        if g == 3{
            println!("Choose the number of Edges (1-2,000,000):");
            io::stdin().read_line(&mut edges).expect("failed to readline"); 
            e = edges.trim().parse::<u32>().unwrap();  

            println!("Choose the Random distribution, 1 for Uniform, 2 for Skewed, 3 for Cosine:");
            io::stdin().read_line(&mut distribution).expect("failed to readline"); 
            d = distribution.trim().parse::<u32>().unwrap();  
        }

        let input : Input = Input::new(v,e,g,d); 

        let graph : Graph = Graph::new(input); 

        graph.display(); 

        graph.output("file1.txt");

        let ordering : Ordering = Ordering::SLVO(graph); 

    }

    // for i in 1..100{
    //     let input : Input = Input::new(*i*100,0,1,0); 
    //     let graph : Graph = Graph::new(input); 
    // }
    
}
