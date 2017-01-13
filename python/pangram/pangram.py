"""
    Function to return True if the input phrase is a pangram.

    See https://en.wikipedia.org/wiki/Pangram for more info.
"""
import string


def is_pangram(sample=None):
    if sample is None or not isinstance(sample, str):
        return False

    sample = set(str(sample).lower())
    alphabet = set(string.ascii_lowercase)
    print(alphabet)

    return alphabet <= sample
