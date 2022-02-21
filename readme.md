# Data Mining Homework 2 -- Apriori Pattern Program

## Installation / Running

This program is written in Rust. Rust is a compiled languages -- if you want to avoid installing build tools, I've included compiled binaries for linux and windows. There are therefore 3 options for running this:

### Simply Running Executable

To run this executable on linux: `./mining_hw2_linux <data_file> <threshold> <output_file>`.

Windows executable has not been tested, but presumably you'd have to do something similar to above but with command prompt

### Running on Emory CS Lab

The linux executable has also been tested to work on the Emory CS Lab system. It can be uploaded and tested there via the following set of commands. This series has also been turned into a small bash script, which can be run with a single command using: `./test_on_lab.sh <username> <data_file> <threshold> <output_file>`. The shell instructions to do this manually:

``` bash
scp ./mining_hw2_linux <data_file> <username>@lab0z.mathcs.emory.edu:~
ssh <username>@lab0z.mathcs.emory.edu
./mining_hw2_linux $(basename <data_file>) <threshold> <output_file>
exit
scp <username>@lab0z.mathcs.emory.edu:~/<output_file> ./<output_file>
```

### Building Yourself

On linux, this is somewhat easy to do. In reference to official docs:

``` bash
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
cargo build --release
mv ./target/release/mining_hw2 ./mining_hw2
```

### Too Much

If you think this is unfeasable, please send me an email and I can easily arrange time to either compile this specifically for your system, create a shared linux box in the cloud to run this, or install rust and cargo tools on your computer: mfigurs@emory.edu

## Security Aspects of Running Pre-Compiled Binaries

I assume responsibility for any intentional malware or security breach found in these files (as there is none) to the full degree required by the Emory Honor Code. I can attest that these are indeed the binaries resulting from the compilation of the code submitted, as dictated by the Emory Honor Code
