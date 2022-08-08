# rusty-graph-coloring

# Running without Installing Rust

Rust offers a way to run the program without compiling it, which is done through an executable. 



# Installing 

Follow instructions to [install Rust.](https://www.rust-lang.org/tools/install)  

Next run: 

    git clone https://github.com/ostenloo/rusty-graph-coloring.git 

    cd rusty-graph-coloring 

    mkdir tmp

# Running 

To run the main graph-coloring program, enter 

    cargo run -p graph-coloring

A CLI will appear where you can control the Graphs you create. 

# Tests 

To run the unit tests, enter 

    cargo run -p graph-coloring tests 

These are a few tests to make sure the program works as expected. 

# Benchmarks

Benchmarks are controlled through command line arguments. The commands are 

| Base Command            | Command | Directory | Arg[3]   | Arg[4]  | Arg[5]  |
|-------------------------|---------|-----------|----------|---------|---------|
| cargo run -p benchmarks | graph   | dirname   | complete |         |         |
| cargo run -p benchmarks | graph   | dirname   | cycle    |         |         |
| cargo run -p benchmarks | graph   | dirname   | random   | uniform | sparse  |
| cargo run -p benchmarks | graph   | dirname   | random   | uniform | dense   |
| cargo run -p benchmarks | graph   | dirname   | random   | skewed  | sparse  |
| cargo run -p benchmarks | graph   | dirname   | random   | skewed  | dense   |
| cargo run -p benchmarks | graph   | dirname   | random   | cosine  | sparse  |
| cargo run -p benchmarks | graph   | dirname   | random   | cosine  | dense   |
| cargo run -p benchmarks | order   | dirname   | SLVO     |         |         |
| cargo run -p benchmarks | order   | dirname   | SODL     |         |         |
| cargo run -p benchmarks | order   | dirname   | URO      |         |         |
| cargo run -p benchmarks | order   | dirname   | BFSR     |         |         |
| cargo run -p benchmarks | order   | dirname   | BFSS     |         |         |
| cargo run -p benchmarks | order   | dirname   | BFSL     |         |         |

    SLVO - Smallest Last Vertex Ordering 
    SODL - Smallest Original Degree Last 
    URO - Uniform Random Ordering 
    BFSR - Breadth First Search Random 
    BFSS - Breadth First Search Smallest 
    BFSL - Breadth First Search Largest 

You must run a `graph` command before running an `order` command. Running a `graph` command will create the graphs as files in a new folder inside the `/tmp` directory, specified by `dirname`. 

Then you must select an existing `dirname` in the `order` command to read the files from. This will benchmark both the order specified in `Arg[3]` and the coloring algorithm. 

For example, a sample command sequence might be 

    cargo run -p benchmarks graph dir1 complete 

    cargo run -p benchmarks order SLVO dir1 

or 

    cargo run -p benchmarks graph dir2 random uniform sparse

    cargo run -p benchmarks order SLVO dir2 

# Histograms 




