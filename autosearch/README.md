# Rust Autoresearch Proof of Concept

This is a complete, simple Proof of Concept (POC) for the autoresearch loop in the `rust-autoresearch-poc` directory.

Here's a breakdown of how this POC works, mapped directly to the concepts in the `AUTORESEARCH_COMPLETE_GUIDE.md`:

1.  **The Target (`src/target.rs`)**: This is the file you want to optimize. It starts with a naive, highly inefficient recursive implementation of a Fibonacci sequence generator.
2.  **The Evaluation Command (`eval.sh`)**: This script builds the Rust project in release mode, runs it, and extracts the metric. It also implements a huge penalty if the compilation fails.
3.  **The Metric (`src/main.rs`)**: The `main.rs` file acts as both a correctness validator (using `assert_eq!`) and a benchmark runner. It outputs `execution_time_ms`, which our `eval.sh` will grab. The baseline time is around ~750ms.
4.  **The Brain (`program.md`)**: This file contains the strict instructions for the AI agent, explaining the setup and defining the infinite "experiment loop" (try an idea -> measure -> keep if faster -> revert if slower -> repeat).

## How to start the Autoresearch loop

To kick off the autonomous optimization process using Gemini CLI, you just need to tell it to read `program.md` and start. You can do this by running a new command in your terminal like this:

```bash
gemini-cli "Read rust-autoresearch-poc/program.md and kick off a new experiment"
```

Once started, Gemini will autonomously edit the Rust file (trying things like iteration instead of recursion, or matrix exponentiation), test the changes by compiling and measuring the run time, and commit the improvements, looping indefinitely until you manually stop it!
