from typing import Optional

def levenshtein(a: str, b: str, limit: Optional[int] = None) -> int:
    """Compute the Levenshtein distance between two strings.

    If a limit is specified, the algorithm will stop calculating once that
    difference is reached. It is a good idea to specify this when performance is
    required, since "very different" strings can be quickly discarded.

    :param a: first string to compare
    :param b: second string to compare
    :param limit: Optional maximum difference to allow, defaults to None

    .. code-block:: pycon
        >>> from stringmetrics import levenshtein
        >>> levenshtein("a slow cat", "a fast bat")
        5
        >>> levenshtein("a slow cat", "a fast bat", limit=3)
        3
    """
    ...
