# wc-rs

> wc written in Rust – word, line, character, and byte count.

This is my own Rust implementation of the Unix command `wc` and serves as a solution to the Coding Challenge presented [here](https://codingchallenges.fyi/challenges/challenge-wc).

## How to build

Execute the following command to build the executable. The binary will be saved in `./target/release/wc-rs`.

```sh
make build
```

## How to run

Execute the binary with a file as an argument to obtain the desired result. The tool supports the following command line flags:

- `-c`: Outputs the number of bytes
- `-l`: Outputs the number of lines
- `-w`: Outputs the number of words
- `-m`: Outputs the number of characters

### Usage examples

```sh
# Outputs the number of bytes
./target/release/wc-rs -c test.txt
342190	test.txt

# Outputs the number of lines
./target/release/wc-rs -l test.txt
7145	test.txt

# Outputs the number of words
./target/release/wc-rs -w test.txt
58164	test.txt

# Outputs the number of characters
./target/release/wc-rs -m test.txt
339292	test.txt

# Outputs with -l, -w, and -c flags
./target/release/wc-rs test.txt
7145	58164	342190	test.txt
```

#### Read from Standard Input

You can also read from standard input by using a pipe:

```sh
cat test.txt | ./target/release/wc-rs -l
7145
```
