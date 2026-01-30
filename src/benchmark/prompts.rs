//! Test prompts for benchmarking.

/// A standardized test prompt with expected token counts
#[derive(Debug, Clone)]
pub struct TestPrompt {
    pub name: &'static str,
    pub text: &'static str,
    pub expected_input_tokens: u32,
    pub expected_output_tokens: u32,
}

/// Short prompt (~50 output tokens) - minimal cost
pub const SHORT_PROMPT: TestPrompt = TestPrompt {
    name: "short",
    text: "Explain what a binary search tree is in exactly three sentences.",
    expected_input_tokens: 15,
    expected_output_tokens: 50,
};

/// Medium prompt (~200 output tokens) - typical interaction
pub const MEDIUM_PROMPT: TestPrompt = TestPrompt {
    name: "medium",
    text: r#"Write a Python function that implements merge sort. Include:
1. The main merge_sort function
2. A helper merge function
3. Brief comments explaining each step
4. An example of calling the function with a sample list"#,
    expected_input_tokens: 50,
    expected_output_tokens: 200,
};

/// Long prompt (~500 output tokens) - extended response
pub const LONG_PROMPT: TestPrompt = TestPrompt {
    name: "long",
    text: r#"You are a technical writer. Write a comprehensive guide about REST API design best practices. The guide should cover:

1. Resource naming conventions
2. HTTP method usage (GET, POST, PUT, PATCH, DELETE)
3. Status code selection
4. Error response formatting
5. Pagination strategies
6. Versioning approaches
7. Authentication considerations

For each topic, provide a brief explanation and a concrete example. The guide should be suitable for intermediate developers who understand HTTP but are new to API design."#,
    expected_input_tokens: 100,
    expected_output_tokens: 500,
};

impl TestPrompt {
    /// Estimate cost for this prompt with given pricing (per million tokens)
    pub fn estimate_cost(&self, input_price: f64, output_price: f64) -> f64 {
        let input_cost = (self.expected_input_tokens as f64 / 1_000_000.0) * input_price;
        let output_cost = (self.expected_output_tokens as f64 / 1_000_000.0) * output_price;
        input_cost + output_cost
    }
}
