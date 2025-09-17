# kaizensim

Tool for [Kaizen: A Factory Story](https://coincidence.games/kaizen/) solutions

## CLI

### score

On success:

```yaml
{
    // ID of the level
    "level": 6, 
    // Time of the solution
    "time": 4, 
    // Cost of the solution
    "cost": 38,
    // Area of the solution
    "area": 130,
    // Was the solution manipulated outside of the game?
    "manipulated": false
}
```

On error:

```yaml
{
    // Human-readable error message
    "error": "Solution incomplete"
}
```

## FFI

### score

```rust
score_create(data: *const u8, len: usize) -> *const ScoreResult

score_destroy(score: *const ScoreResult)

struct ScoreResult
{
    // Pointer to NUL-terminated error string, null pointer if no error
    error: usize,
    error_len: usize,
    level: i32,
    time: i32,
    cost: i32,
    area: i32,
    manipulated: bool,
}
```
