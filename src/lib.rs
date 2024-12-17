/// Computes the Levenshtein distance between two strings.
///
/// # Arguments
/// * `a` - The first string.
/// * `b` - The second string.
///
/// # Returns
/// * The Levenshtein distance between `a` and `b`.
///
/// # Example
/// ```
/// let distance = levenshtein_distance("sitting", "kitten");
/// assert_eq!(distance, 3);
/// ```
/// lev_{a, b}(i, j) = { min { 
///
///lev_{a, b}(i - 1, j) + 1
///lev_{a, b}(i, j - 1) + 1
///lev_{a, b}(i - 1, j -1) + 𝛿(a[i - 1], b[j - 1])
///
///    where 𝛿(a[i - 1], b[j - 1]) = 0 -> if a[i - 1] = b[j - 1]
///    otherwise 𝛿 = 1
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let m = a.len();
    let n = b.len();
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    //2d matrix with 0 value and +1 offset
    let mut matrix = vec![vec![0; n+1]; m+1];

    // Base Cases
    for i in 0..=m {
        matrix[i][0] = i;
    }
    
    for j in 0..=n {
        matrix[0][j] = j;
    }

    // Fill matrix
    for i in 1..=m {
        for j in 1..=n{
            
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            }
            else {
                1
            };

            matrix[i][j] = * [
                matrix[i - 1][j] + 1, // deletion
                matrix[i][j - 1] + 1, // insertion
                matrix[i - 1][j - 1] + cost, // substitution
            ]
            .iter()
            .min()
            .unwrap()


        }
    }

    //check bottom right for final score/distance
    matrix[m][n]
}

///takes file name and parses lines
pub fn read_word_list(file_name: &str) -> Vec<String> {
    let mut list = Vec::new();
    if let Ok(lines) = std::fs::read_to_string(file_name) {
        for line in lines.lines() {
            list.push(line.to_string());
        }
    }
    list
}