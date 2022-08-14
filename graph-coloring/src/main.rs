use std::{io,fs}; 
use graph_coloring::{input::*,graph::*,ordering::*}; 

fn main() {
    // uncomment this out for the SLVO walkthrough 
    // let graph : Graph = Graph::new_from_file("../sample-SLVO-graph.txt"); 
    // graph.display(); 
    // let ordering : Ordering = Ordering::SLVO(graph); 
    // ordering.displayOrder(); 
    // ordering.coloring(); 

    loop 
    {
        let (mut terminate, mut vertices, mut edges, mut graph, mut distribution, mut order) = 
        (String::new(), String::new(), String::new(), String::new(), String::new(), String::new());

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
        let mut e = match g{
            1 => (v)*(v-1)/2, 
            2 => v, 
            _ => 0
        }; 
        let mut d : u32 = 0; 

        if g == 3{
            println!("Choose the number of Edges (1-2,000,000):");
            io::stdin().read_line(&mut edges).expect("failed to readline"); 
            e = edges.trim().parse::<u32>().unwrap();  

            println!("Choose the Random distribution, 1 for Uniform, 2 for Skewed, 3 for Normal:");
            io::stdin().read_line(&mut distribution).expect("failed to readline"); 
            d = distribution.trim().parse::<u32>().unwrap();  
        }

        let input : Input = Input::new(v,e,g,d); 

        let graph : Graph = Graph::new(input); 

        graph.display(); 

        graph.output("../tmp/file1.txt");

        println!("Choose the type of Ordering: 
        \n1 - Smallest Last Vertex Ordering 
        \n2 - Smallest Original Degree Last 
        \n3 - Uniform Random Ordering 
        \n4 - Breadth First Search from a Random Vertice 
        \n5 - Breadth First Search from the Smallest Vertice 
        \n6 - Breadth First Search from the Largest Vertice");
        io::stdin().read_line(&mut order).expect("failed to readline");
        let mut o : u32 = order.trim().parse::<u32>().unwrap();  
        let ordering : Ordering = match o{
            1 => Ordering::SLVO(graph),
            2 => Ordering::SODL(graph), 
            3 => Ordering::URO(graph), 
            4 => Ordering::BFSR(graph), 
            5 => Ordering::BFSS(graph), 
            6 => Ordering::BFSL(graph),
            _ => panic!("Not an ordering."),  
        };

        ordering.displayOrder(); 

        ordering.coloring(); 
    }
}
