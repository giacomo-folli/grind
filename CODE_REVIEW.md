## To implement
- [ ] Time management can be improved

---

### Time management can be improved
Timestamps are RFC2822 `String`s. -> Less robustness at compile-time and recurring parsing during output.

Improvement:
- Consider `DateTime<Utc>` in the model.
- Use RFC3339 and explicitly handle future dates in the relative format.
