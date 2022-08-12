# rusty-graph-coloring

# Setup

I've included the executables for the program so it's not required to install Rust. The commands for running without having Rust installed will be listed below. 

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

    ./exec/graph-coloring

A CLI will appear where you can control the settings for the Graphs you create. 

# Tests 

To run the unit tests (inside graph-coloring directory)
    
    cargo run tests 

These are a few tests to make sure the program works as expected. 

# Benchmarks

Benchmarks are controlled through command line arguments. The commands are 

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

For example, a sample command sequence might be (inside benchmarks directory)

    cargo run graph dir1 complete
    
    cargo run order SLVO dir1 
    
without Rust installed: 
    
    ./exec/benchmarks graph dir1 complete
    
    ./exec/benchmarks order SLVO dir1
    
    



