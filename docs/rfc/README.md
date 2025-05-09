# RFC (Request for Comments) Process

## Rule
**All code changes must start with an RFC.**

## Process
1. Create a new RFC file at `docs/rfc/YYYY-MM-<theme>.md`
2. Describe:
   - Why the change is needed
   - Which modules will be affected
   - Expected behavior changes
3. Submit the RFC as a PR for review
4. After RFC approval, implement the actual code changes

## Example
```
docs/rfc/2025-05-message-bus.md
```

## Benefits
- Clear documentation of design decisions
- Prevents hasty changes
- Ensures all stakeholders are informed
- Provides historical reference for future changes
