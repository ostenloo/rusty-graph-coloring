# rusty-graph-coloring

# Setup

I've included an executable to run the main program graph-coloring, so it's possible to run without Rust installed. 

Otherwise, follow instructions to [install Rust.](https://doc.rust-lang.org/book/ch01-01-installation.html)  

Next run: 

    git clone https://github.com/ostenloo/rusty-graph-coloring.git 

    cd rusty-graph-coloring 

    mkdir tmp

# Running 

To run the main graph-coloring program with Rust installed: 

    cd graph-coloring
    
    cargo run
    
without Rust installed: 

    cd graph-coloring

    ../exec/graph-coloring

A CLI will appear where you can control the settings for the Graphs you create. 

# Benchmarks

Running Benchmarks will require Rust to be installed (I was having issues and was too lazy to figure out a way to make it work, though it is possible). Benchmarks are controlled through command line arguments. The commands are 

| Base Command            | Command | Directory | Arg[3]   | Arg[4]  | Arg[5]  |
|-------------------------|---------|-----------|----------|---------|---------|
| cargo run               | graph   | dirname   | complete |         |         |
| cargo run               | graph   | dirname   | cycle    |         |         |
| cargo run               | graph   | dirname   | random   | uniform | sparse  |
| cargo run               | graph   | dirname   | random   | uniform | dense   |
| cargo run               | graph   | dirname   | random   | skewed  | sparse  |
| cargo run               | graph   | dirname   | random   | skewed  | dense   |
| cargo run               | graph   | dirname   | random   | cosine  | sparse  |
| cargo run               | graph   | dirname   | random   | cosine  | dense   |
| cargo run               | order   | dirname   | SLVO     |         |         |
| cargo run               | order   | dirname   | SODL     |         |         |
| cargo run               | order   | dirname   | URO      |         |         |
| cargo run               | order   | dirname   | BFSR     |         |         |
| cargo run               | order   | dirname   | BFSS     |         |         |
| cargo run               | order   | dirname   | BFSL     |         |         |

    SLVO - Smallest Last Vertex Ordering 
    SODL - Smallest Original Degree Last 
    URO - Uniform Random Ordering 
    BFSR - Breadth First Search Random 
    BFSS - Breadth First Search Smallest 
    BFSL - Breadth First Search Largest 

You must run a `graph` command before running an `order` command. Running a `graph` command will create the graphs as files in a new folder inside the `/tmp` directory, specified by `dirname`. 

Then you must select an existing `dirname` in the `order` command to read the files from. This will benchmark both the order specified in `Arg[3]` and the coloring algorithm. 

For example, a sample command sequence might be

    cd benchmarks

    cargo run graph dir1 complete
    
    cargo run order SLVO dir1 

## Generated Benchmarks 

These numbers were generated using Benchmarks. 

https://docs.google.com/spreadsheets/d/e/2PACX-1vRmm8M1quJzVrzNhRHXlO_S1JYL7cazmLZeD0PBCWBaFPlJbFOlyYV4-Rgk7WLwhF2JGrBKnF6IpOaN/pubhtml
