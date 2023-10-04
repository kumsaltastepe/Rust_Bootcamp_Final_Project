// Define the FilterCondition struct
struct FilterCondition {
    value: i32,
}

// Implement the is_match method on the FilterCondition struct
impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        *item == self.value
    }
}

// Define the custom_filter function
fn custom_filter(collection: Vec<i32>, condition: &FilterCondition) -> Vec<i32> {
    let mut result = Vec::new();
    for item in collection {
        if condition.is_match(&item) {
            result.push(item);
        }
    }
    result
}

fn main() {
    // Create a vector with some elements
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Initialize a FilterCondition object with the desired value
    let condition = FilterCondition { value: 5 };

    // Call the custom_filter function with the vector and the FilterCondition object
    let filtered_numbers = custom_filter(numbers, &condition);

    // Print the filtered result to the console
    println!("{:?}", filtered_numbers);
}
