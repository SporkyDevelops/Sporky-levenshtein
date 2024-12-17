# Spork Checker
![Rust](https://img.shields.io/badge/Rust-v1.0-orange?logo=rust&style=flat-square)

A very simple Levenshtein-based spell checker that finds approximate matches for a given word by comparing to a word list. Implemented in rust.

## Features
- **Levenshtein Distance Calculation**: Find similar words based on edit distance.
- **Customizable Results**: Set the number of matches and verbosity.

## Installation

### Prerequisites
- Rust installed on your system. You can install it using [rustup](https://rustup.rs/).

### Build and Run
1. Clone this repository:
```bash
git clone https://github.com/SporkyDevelops/Sporky-levenshtein.git
cd Sporky-levenshtein
```
2. Build the project:
```bash
cargo build --release
```
3. Copy wordlist:
```bash
cp src/en.txt /target/release/
```
4. Run the application:
```bash
cd /target/release/
./spork-checker
```

## Usage
```bash
spork-checker [OPTIONS] [TARGET]
```
### Arguments
**`[TARGET]`**: The target word to check against the word list.

### Options
- **`-v`, `--verbose`**: Show detailed output, including the Levenshtein distance for each match.
- **`-n`, `--number [NUMBER]`**: Specify the number of matches to display (default: 5).

## Example

### Input:
```bash
spork-checker ruzt -n 1
```
### Output:
```
Possible Matches: 
- 'rust'
```

## Word List
The application reads a word list from en.txt. Ensure this file exists in the same directory as the binary or specify the path in the `read_word_list` function.

## Equation:

<img src="https://latex.codecogs.com/png.image?\inline&space;\dpi{190}\bg{white}lev_{a,b}(i,j)=\left\{\begin{matrix}min\left\{\begin{matrix}lev_{a,b}(i-1,j)&plus;1\\lev_{a,b}(i,j-1)&plus;1\\lev_{a,b}(i-1,j-1)&plus;\delta(a[i-1],b[j-1])\\\end{matrix}\right.\end{matrix}\right." title="lev_{a,b}(i,j)=\left\{\begin{matrix}min\left\{\begin{matrix}lev_{a,b}(i-1,j)&plus;1\\lev_{a,b}(i,j-1)&plus;1\\lev_{a,b}(i-1,j-1)&plus;\delta(a[i-1],b[j-1])\\\end{matrix}\right.\end{matrix}\right." />

<img src="https://latex.codecogs.com/png.image?\inline&space;\dpi{190}\bg{white}\begin{matrix}where&\delta(a[i-1],b[j-1])=0\\if&a[i-1]=b[j-1]\\otherwise&\delta=1\\\end{matrix}" title="\begin{matrix}where&\delta(a[i-1],b[j-1])=0\\if&a[i-1]=b[j-1]\\otherwise&\delta=1\\\end{matrix}" />

```
(i-1, j) deletion
(i, j-1) insertion
(i-1, j-1) substitution
```

### Matrix Example:
![Source: Datumorphism](https://github.com/user-attachments/assets/86affff9-95cc-4086-8add-33841a474a82)
<sub>Photo source: Datumorphism</sub>

#

>#### Author
> Sporky Develops
