use crate::input::*; 

use std::collections::LinkedList;
use std::io::*; 
use rand::prelude::*;
use std::fs;

//Vector of Nodes to represent a Graph 
pub struct Graph{
    pub adjacencyList : Vec<LinkedList<u32>>, 
    pub inDegreeList : Vec<LinkedList<u32>>, 
    pub vertices : usize, 
    pub edges : usize,
}

impl Graph{
    pub fn new(input : Input) -> Graph 
    {
        match input.graph {
        GraphKind::complete => Graph::complete_graph(input),
        GraphKind::cycle => Graph::cycle_graph(input),
        GraphKind::random(_) => Graph::random_graph(input), 
        }
    }

    //maybe think about refactoring 
    //.split creates string slices &str which 
    pub fn new_from_file(filename : &str) -> Graph 
    {
        let content: Vec<String> = fs::read_to_string("tmp/".to_owned() + filename)
        .expect("Something went wrong reading the file")
        .split("\n")
        .map(|s| s.to_string()) 
        .collect(); 

        let v = content[0].parse::<usize>().unwrap(); 
        let e = (content.len() - content[1].parse::<usize>().unwrap() - 1)/2; 
        let mut adj_list = vec![LinkedList::<u32>::new(); v]; 
        let mut in_degree_list = vec![LinkedList::<u32>::new(); v]; 

        for i in 1..v+1{
            let start = content[i].parse::<usize>().unwrap(); 
            let end = if i == v {content.len() - 1} else {content[i+1].parse::<usize>().unwrap()}; 
            for j in start..end
            {
                let node = content[j].parse::<u32>().unwrap(); 
                adj_list[i-1].push_back(node); 
            }
        }

        for (ii, i) in adj_list.iter().enumerate(){
            in_degree_list[i.len()].push_back(ii.try_into().unwrap()); 
        }

        Graph{
            adjacencyList : adj_list, 
            inDegreeList : in_degree_list, 
            vertices: v, 
            edges: e,
        }
    }

    pub fn complete_graph(input : Input) -> Graph 
    {
        let v = input.vertices.try_into().unwrap(); 
        let e = input.edges.try_into().unwrap();  
        let mut adj_list = vec![LinkedList::<u32>::new(); v]; 
        let mut in_degree_list = vec![LinkedList::<u32>::new(); v]; 

        for (i, ll) in adj_list.iter_mut().enumerate() {
            for j in 0..v {
                if i != j {
                ll.push_back(j.try_into().unwrap()); 
                }
            }
        }

        //set the in-degree list 
        for i in 0..v{ 
            in_degree_list[v - 1].push_back(i.try_into().unwrap()); 
        }

        Graph{
            adjacencyList : adj_list, 
            inDegreeList: in_degree_list, 
            vertices: v, 
            edges: e, 
        }
    }

    pub fn cycle_graph(input : Input) -> Graph 
    {
        let v = input.vertices.try_into().unwrap();
        let e = input.edges.try_into().unwrap();  
        let mut adj_list = vec![LinkedList::<u32>::new(); v]; 
        let mut in_degree_list = vec![LinkedList::<u32>::new(); v]; 

        for (i, ll) in adj_list.iter_mut().enumerate() {
            let i1 : i32 = i as i32; 
            let v1 : i32 = v as i32; 
 
            ll.push_back((i1-1).rem_euclid(v1).try_into().unwrap()); 
            ll.push_back((i1+1).rem_euclid(v1).try_into().unwrap()); 
        }

        //set the in-degree list 
        for i in 0..v{ 
            in_degree_list[2].push_back(i.try_into().unwrap()); 
        }

        Graph{
            adjacencyList : adj_list, 
            inDegreeList: in_degree_list,
            vertices: v,
            edges: e,   
        }
    }

    pub fn random_graph(input : Input) -> Graph 
    {
        let v = input.vertices.try_into().unwrap();
        let e = input.edges.try_into().unwrap();  
        let mut adj_list = vec![LinkedList::<u32>::new(); v]; 
        let mut in_degree_list = vec![LinkedList::<u32>::new(); v]; 
        //random_distr is a Vec<u32> of size m_e to get where the edges are
        //will still need to parse it a bit to fit our adjacency list 
        //here each edge will map to 2 nodes in the adjacency list, 
        //one for each vertice in the edge 
        let random_distr = Graph::random_distr(input);
        let mut random_distr_matrix = vec![vec![0; v as usize]; v as usize]; 

        let mut itr = 0; 
        for i in 0..v{
            for j in i+1..v{
                if random_distr[itr] == 1
                {
                    random_distr_matrix[i][j] = 1; 
                    random_distr_matrix[j][i] = 1; 
                }
                itr += 1; 
            }
        }

        for i in 0..v{
            for j in 0..v{
                if i != j && random_distr_matrix[i][j] == 1{
                    adj_list[i].push_back(j.try_into().unwrap()); 
                }
            }
        }

        for (ii, i) in adj_list.iter().enumerate()
        {
            in_degree_list[i.len()].push_back(ii.try_into().unwrap()); 
        }

        Graph{
            adjacencyList : adj_list, 
            inDegreeList: in_degree_list, 
            vertices : v, 
            edges : e, 
        }
    }

    /* 
    **Function to generate the random distribution 
    **For uniform, skewed, and cosine 
    **If x = each potential edge and 
    **f(x) = the probability of a potential edge of having an edge 
    **Then these are the distributions generated by those probabilities 
    */ 
    pub fn random_distr(input : Input) -> Vec<u32> 
    {
        let m_e = input.max_edges(); 
        let mut e = input.edges; 
        let v = input.vertices; 
        //m is the slope of the function representing the distribution 
        //in the form y = mx+b 
        let m = -(0.5*(m_e as f64) - e as f64)*2.0 / ((m_e as f64) * (m_e as f64));
        let mut rng = rand::thread_rng();
        //[0,1,2,3,...,m_e - 1] 
        //array with numbers indicating each of the edges 
        //keeping a mutable array of "references" to the edges, 
        //so that we may remove them once they are chosen 
        let mut edge_nums : Vec<u32> = (0..m_e).collect();
        //array with 0s and 1s 
        //each element is a potential edge 
        //0 indicates there is no edge 
        //1 indicates there is an edge 
        //vec![value; size]; 
        let mut edge_vec : Vec<u32> = vec![0; m_e as usize]; 
        //count down to get the required number of edges 
        while e > 0{
            //choose the index of the edge_number 
            let i = (0..edge_nums.len()).choose(&mut thread_rng()).unwrap(); 
            //
            if edge_vec[edge_nums[i] as usize] != 1 {
                let y: f64 = rng.gen();
                let cond = match input.dist {
                    Some(DistKind::uniform)=> 0.5, 
                    Some(DistKind::skewed) => 0.5 + m*(i as f64), 
                    Some(DistKind::cosine) => (1.0 + (i as f64).sin())*(e as f64)/(m_e as f64), 
                    None => 0.0, 
                }; 
                if y < cond 
                {
                    edge_vec[edge_nums[i] as usize] = 1;
                    e -=1;  
                }
            }
            //After you chose one potential edge, remove it until all edges have been chosen  
            Some(edge_nums.swap_remove(i)); 
            //Now all potential edges have been chosen, but if you are here, 
            //you haven't filled out all the actual edges yet 
            //now each of the edges are fair game to be chosen 
            //but you will skip if there is already an edge in that slot 
            if edge_nums.is_empty() {
                edge_nums = (0..m_e).collect(); 
            }
        }

        edge_vec 
    }

    pub fn display(&self) {
        fn display_helper(arr : &Vec<LinkedList<u32>>){
            //(index, value)
            println!("");
            for (ii, i) in arr.iter().enumerate()
            {
                print!("{}", ii); 
                for (ji, j) in i.iter().enumerate()
                {
                    if ji == 0
                    {
                        print!("->"); 
                    }
                    print!("{}", j); 
                    if ji != i.len() - 1
                    {
                        print!("->"); 
                    }
                }
                print!("\n"); 
            }
            println!(""); 
        }

        println!("Adjacency List:"); 
        display_helper(&self.adjacencyList); 
        println!("In-degree List:");
        display_helper(&self.inDegreeList);  
    }

    pub fn output(&self, filename : &str) {
        let mut file = std::fs::File::create("tmp/".to_owned() + filename).expect("create failed");
        let v = self.adjacencyList.len(); 
        write!(file, "{}\n", v);
        let mut sum = v + 1; 
        for i in self.adjacencyList.iter(){
            write!(file, "{}\n", sum); 
            sum += i.len(); 
        }
        for i in self.adjacencyList.iter(){
            for j in i.iter(){
                write!(file, "{}\n", j); 
            }
        }
    }

}

