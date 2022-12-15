from .stringmetrics import levenshtein as levenshtein

# from .stringmetrics import levenshtein_advanced as _levenshtein_advanced

# Easier to do defaults here than in rust
# def levenshtein_advanced(
#     a: Iterable,
#     b: Iterable,
#     limit: Optional[int] = None,
#     insertion_weight: int = 1,
#     deletion_weight: int = 1,
#     substitution_weight: int = 1,
# ) -> int:
#     return _levenshtein_advanced(
#         a, b, limit, insertion_weight, deletion_weight, substitution_weight
#     )
