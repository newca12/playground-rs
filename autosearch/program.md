# Autoresearch: Rust Fibonacci Optimization

## Setup
- **Target file**: `src/target.rs`
- **Eval command**: `bash eval.sh`
- **Metric**: `execution_time_ns` (lower is better)
- **Constraint**: Only modify `src/target.rs`. Do not modify `src/main.rs` or `eval.sh`. The function signature must remain `pub fn fibonacci(n: u64) -> u64`. Correctness must be maintained (main.rs checks this automatically).

## The experiment loop
LOOP FOREVER:
1. Look at current git state.
2. Modify `src/target.rs` with an experimental idea to make it faster (e.g. memoization, iterative approach, matrix exponentiation).
3. `git add src/target.rs`
4. `git commit -m "experiment: [description]"`
5. Run: `bash eval.sh`
6. Read metric (`execution_time_ns`).
7. If improved (lower time) → keep the commit
8. If worse or crashed → `git reset --hard HEAD~1`
9. Log result to `results.tsv` (columns: commit_hash, execution_time_ns, description)
10. Repeat. Never stop.