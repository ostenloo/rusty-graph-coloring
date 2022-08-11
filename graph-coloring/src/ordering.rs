use crate::graph::*; 

use std::cmp; 
use std::collections::{LinkedList,HashSet,VecDeque};
use rand::{Rng,thread_rng,seq::SliceRandom};

pub struct Ordering{ 
    pub order: Vec<u32>, 
    pub adjacencySetList: Vec<HashSet<u32>>, 
    pub vertices : usize, 
    pub edges : usize,
}

// remove from ll 
// O(n) time basically 
pub fn remove_from_ll(element: u32, ll: &LinkedList<u32>) -> LinkedList<u32> { 
    let mut v : Vec<u32> = Vec::new(); 
    for node in ll.iter() {
        if *node != element {
        v.push(*node); 
        }
    }
    let mut res : LinkedList<u32> = v.iter().copied().collect(); 
    res 
}

pub fn create_adj_set(AL : &Vec<LinkedList<u32>>) -> Vec<HashSet<u32>> {
    let mut adjacencySetList = vec![HashSet::<u32>::new(); AL.len()]; 

    for (ii,i) in AL.iter().enumerate()
    {
        for j in i.iter(){
            adjacencySetList[ii].insert(*j); 
        }
    }

    adjacencySetList
}

impl Ordering{
//Smallest last vertex ordering
pub fn SLVO(graph : Graph) -> Ordering{
    // graph.display(); 
    let mut v = graph.vertices; 
    let mut e = graph.edges; 
    let mut IDL = graph.inDegreeList;  
    let mut AL = graph.adjacencyList; 
    let mut order : Vec<u32> = Vec::new(); 

    let adjacencySetList : Vec<HashSet<u32>> = create_adj_set(&AL); 

    // //getting all the vertices with an initial in-degree of 0
    for i in IDL[0].iter(){
        order.push(*i);
        v -= 1;  
    }

    let mut flag = false; 
    loop {
        let mut IDL_ = vec![LinkedList::<u32>::new(); graph.vertices]; 
        let mut AL_  = AL.clone();  

        if v == 0{
            break; 
        }
        //looping through the in degree list  
        for i in IDL[1..].iter(){
            //looping through the vertices at each in degree 
            for j in i.iter()
            {
                order.push(*j); 
                for k in AL[*j as usize].iter()
                {
                    AL_[*k as usize] = remove_from_ll(*j, &AL_[*k as usize]); 
                    if AL_[*k as usize].len() == 0{
                        v -= 1; 
                        order.push(*k); 
                    }
                }
                AL_[*j as usize] = LinkedList::<u32>::new(); 
                IDL_ = vec![LinkedList::<u32>::new(); graph.vertices]; 
                for (ii, i) in AL_.iter().enumerate(){
                    IDL_[i.len()].push_back(ii.try_into().unwrap()); 
                }
                v -= 1;

                println!("Removing vertex {}\n", *j); 

                let g = Graph{ 
                    adjacencyList : AL_.clone(),
                    inDegreeList: IDL_.clone(), 
                    vertices : v, 
                    edges : 1, 
                }; 
                g.display();
                break;  
            }
            break;
        }
        IDL = IDL_; 
        AL = AL_; 
    }
    order.reverse(); 
    Ordering
    {
        order: order,
        adjacencySetList: adjacencySetList,
        vertices: graph.vertices, 
        edges: graph.edges, 
    }
}

//Smallest original degree last ordering
pub fn SODL(graph : Graph) -> Ordering {
    let mut order : Vec<u32> = Vec::new(); 
    let adjacencySetList : Vec<HashSet<u32>> = create_adj_set(&graph.adjacencyList); 

    for i in graph.inDegreeList.iter(){
        for j in i.iter(){
            order.push(*j); 
        }
    }
    order.reverse(); 
    Ordering
    {
        order: order,
        adjacencySetList: adjacencySetList,
        vertices: graph.vertices, 
        edges: graph.edges, 
    }
}

//Uniform random ordering 
pub fn URO(graph : Graph) -> Ordering{
    let mut rng = thread_rng();
    let mut order : Vec<u32> = (0..graph.vertices as u32).collect();
    order.shuffle(&mut rng);
    
    let adjacencySetList : Vec<HashSet<u32>> = create_adj_set(&graph.adjacencyList); 
    
    Ordering
    {
        order: order,
        adjacencySetList: adjacencySetList,
        vertices: graph.vertices, 
        edges: graph.edges, 
    }
}

//bfs starting from a random vertex 
pub fn BFSR(graph : Graph) -> Ordering{
    let mut rng = thread_rng(); 
    let mut order : Vec<u32> =  Vec::new(); 
    let mut IDL = graph.inDegreeList;  
    let mut AL = graph.adjacencyList; 
    let mut remaining : Vec<u32> = (0..graph.vertices as u32).collect(); 
    let mut visited : HashSet<u32> = HashSet::new();
    let mut queue : VecDeque<u32> = VecDeque::new(); 
    let adjacencySetList : Vec<HashSet<u32>> = create_adj_set(&AL); 
    //must loop until all vertices have been visited, 
    //because there can be any number of islands 
    loop {
        if remaining.len() == 0 {
            break; 
        }
        let mut start : u32 = rng.gen_range(0..remaining.len() as u32); 
        println!("{}", start); 
        queue.push_back(start); 
        visited.insert(start); 
        remaining.retain(|x| *x != start);
        for i in AL[start as usize].iter() 
        {
            if !visited.contains(i) {
                queue.push_back(*i); 
                visited.insert(*i);
                remaining.retain(|x| *x != *i);
            }
        }
        order.push(start); 
        queue.pop_front(); 
        loop{
            if queue.len() == 0{
                break; 
            }
            let node = queue[0]; 
            for i in AL[node as usize].iter() 
            {
                if !visited.contains(i) {
                    queue.push_back(*i); 
                    visited.insert(*i); 
                    remaining.retain(|x| *x != *i);
                }
            }
            order.push(node); 
            queue.pop_front(); 
        }
    }

    Ordering
    {
        order: order,
        adjacencySetList: adjacencySetList,
        vertices: graph.vertices, 
        edges: graph.edges, 
    }
}

//bfs starting from smallest remaining degree 
pub fn BFSS(graph : Graph) -> Ordering{
    let mut rng = thread_rng(); 
    let mut order : Vec<u32> =  Vec::new(); 
    let mut IDL = graph.inDegreeList;  
    let mut AL = graph.adjacencyList;
    let mut remaining : Vec<u32> = (0..graph.vertices as u32).collect(); 
    let mut visited : HashSet<u32> = HashSet::new();
    let mut queue : VecDeque<u32> = VecDeque::new(); 
    let adjacencySetList : Vec<HashSet<u32>> = create_adj_set(&AL); 
    //must loop until all vertices have been visited, 
    //because there can be any number of islands 
    loop {
        if remaining.len() == 0 {
            break; 
        }
        let mut start : u32 = u32::MAX;
        for i in IDL.iter() {
            for j in i.iter() {
                if !visited.contains(j)
                {
                    start = *j; 
                    break; 
                }
            }
        }
        queue.push_back(start); 
        visited.insert(start); 
        remaining.retain(|x| *x != start);
        for i in AL[start as usize].iter() 
        {
            if !visited.contains(i) {
                queue.push_back(*i); 
                visited.insert(*i);
                remaining.retain(|x| *x != *i);
            }
        }
        order.push(start); 
        queue.pop_front(); 
        loop{
            if queue.len() == 0{
                break; 
            }
            let node = queue[0]; 
            for i in AL[node as usize].iter() 
            {
                if !visited.contains(i) {
                    queue.push_back(*i); 
                    visited.insert(*i); 
                    remaining.retain(|x| *x != *i);
                }
            }
            order.push(node); 
            queue.pop_front(); 
        }
    }

    Ordering
    {
        order: order,
        adjacencySetList: adjacencySetList,
        vertices: graph.vertices, 
        edges: graph.edges, 
    }
}

//bfs starting from largest remaining degree 
pub fn BFSL(graph : Graph) -> Ordering{
    let mut rng = thread_rng(); 
    let mut order : Vec<u32> =  Vec::new(); 
    let mut IDL = graph.inDegreeList;  
    let mut AL = graph.adjacencyList; 
    let mut remaining : Vec<u32> = (0..graph.vertices as u32).collect(); 
    let mut visited : HashSet<u32> = HashSet::new();
    let mut queue : VecDeque<u32> = VecDeque::new(); 
    let adjacencySetList : Vec<HashSet<u32>> = create_adj_set(&AL); 
    //must loop until all vertices have been visited, 
    //because there can be any number of islands 
    loop {
        if remaining.len() == 0 {
            break; 
        }
        let mut start : u32 = u32::MAX;
        for i in IDL.iter().rev() {
            for j in i.iter() {
                if !visited.contains(j)
                {
                    start = *j; 
                    break; 
                }
            }
        }
        queue.push_back(start); 
        visited.insert(start); 
        remaining.retain(|x| *x != start);
        for i in AL[start as usize].iter() 
        {
            if !visited.contains(i) {
                queue.push_back(*i); 
                visited.insert(*i);
                remaining.retain(|x| *x != *i);
            }
        }
        order.push(start); 
        queue.pop_front(); 
        loop{
            if queue.len() == 0{
                break; 
            }
            let node = queue[0]; 
            for i in AL[node as usize].iter() 
            {
                if !visited.contains(i) {
                    queue.push_back(*i); 
                    visited.insert(*i); 
                    remaining.retain(|x| *x != *i);
                }
            }
            order.push(node); 
            queue.pop_front(); 
        }
    }

    Ordering
    {
        order: order,
        adjacencySetList: adjacencySetList,
        vertices: graph.vertices, 
        edges: graph.edges, 
    }
}

//coloring 
//adjacencySetList-- list of sets of all vertices adjacent to a vertice for each vertice
//adjacencyColorSetList-- list of sets of all colors adjacent to a vertice for each vertice
pub fn coloring(&self) {
    let mut adjacencyColorSetList : Vec<HashSet<u32>> = vec![HashSet::<u32>::new() ; self.order.len()]; 

    let mut requiredColors : u32 = 0; 

    println!("Coloring: \n"); 

    for i in self.order.iter(){
        let mut smallestAvailableColor : u32 = 0; 
        loop {
            if !adjacencyColorSetList[*i as usize].contains(&smallestAvailableColor)
            {
                break; 
            }
            else 
            {
                smallestAvailableColor += 1; 
            }
        }

        for j in self.adjacencySetList[*i as usize].iter(){
            adjacencyColorSetList[*j as usize].insert(smallestAvailableColor); 
        }
        requiredColors = cmp::max(smallestAvailableColor, requiredColors); 
        println!("Vertice: {}, Color: {}",*i,smallestAvailableColor); 
    }
    println!("\nNeeded {} colors to color a graph of {} Vertices and {} Edges.",requiredColors + 1, self.vertices, self.edges); 

    // for i in self.adjacencySetList.iter() {
    //     for j in i.iter() {
    //         print!("{}", *j); 
    //     }
    //     println!();
    // }

    // for i in adjacencyColorSetList.iter() {
    //     for j in i.iter() {
    //         print!("{}", *j); 
    //     }
    //     println!();
    // }
}

pub fn displayOrder(&self) {
    print!("Order: \n\n{}", self.order[0]); 
    for i in self.order[1..].iter() {
        print!("->{}", *i); 
    }
    println!("\n"); 
}
}