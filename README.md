# kaizensim

Tool for [Kaizen: A Factory Story](https://store.steampowered.com/app/2275490/Kaizen_A_Factory_Story/) solutions

## Output

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
    "error": "SolutionIncomplete"
}
```