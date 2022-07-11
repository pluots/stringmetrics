# Stringmetrics

Python bindings for the Rust stringmetrics library

```py
>>> from stringmetrics import levenshtein
>>> levenshtein("a slow cat", "a fast bat")
5
>>> levenshtein("a slow cat", "a fast bat", limit=3)
3
```
