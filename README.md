# Spork Checker

A very simple Levenshtein-based spell checker that finds approximate matches for a given word by comparing to a word list.

---

## Features
- **Levenshtein Distance Calculation**: Find similar words based on edit distance.
- **Customizable Results**: Set the number of matches and verbosity.

---

## Installation

### Prerequisites
- Rust installed on your system. You can install it using [rustup](https://rustup.rs/).

### Build and Run
1. Clone this repository:
   ```bash
   git clone 
   cd
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Run the application:
   ```bash
   ./target/release/spork-checker
   ```

## Usage
```bash
spork-checker [OPTIONS] [TARGET]
```
### Arguments
[TARGET]: The target word to check against the word list.

### Options
-v, --verbose: Show detailed output, including the Levenshtein distance for each match.
-n, --number <NUMBER>: Specify the number of matches to display (default: 5).

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
The application reads a word list from en.txt. Ensure this file exists in the same directory as the binary or specify the path in the read_word_list function.

## Equation:

<img src="https://latex.codecogs.com/svg.image?\inline&space;\bg{white}lev_{a,b}(i,j)=\left\{\begin{matrix}min\end{matrix}\right.\left\{\begin{matrix}lev_{a,b}(i-1,j)&plus;1\\lev_{a,b}(i,j-1)&plus;1\\lev_{a,b}(i-1,j-1)&plus;\delta(a[i-1],b[j-1])\end{matrix}\right." title="lev_{a,b}(i,j)=\left\{\begin{matrix}min\end{matrix}\right.\left\{\begin{matrix}lev_{a,b}(i-1,j)+1\\lev_{a,b}(i,j-1)+1\\lev_{a,b}(i-1,j-1)+\delta(a[i-1],b[j-1])\end{matrix}\right." />

<img src="https://latex.codecogs.com/svg.image?\inline&space;\bg{white}\begin{matrix}where&\delta(a[i-1],b[j-1])=0\\if&a[i-1]=b[j-1]\\otherwise&\delta=1\\\end{matrix}" title="\begin{matrix}where&\delta(a[i-1],b[j-1])=0\\if&a[i-1]=b[j-1]\\otherwise&\delta=1\\\end{matrix}" />

```
(i-1, j) deletion
(i, j-1) insertion
(i-1, j-1) substitution
```

### Matrix Example:
![Source: Datumorphism](https://github.com/user-attachments/assets/86affff9-95cc-4086-8add-33841a474a82)



#### Author
Sporky Develops
