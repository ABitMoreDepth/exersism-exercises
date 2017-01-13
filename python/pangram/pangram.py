"""
    Function to return True if the input phrase is a pangram.

    See https://en.wikipedia.org/wiki/Pangram for more info.
"""
import string


def is_pangram(sample=None):
    if sample is None or not isinstance(sample, str):
        return False
    letters = []

    for character in sample.lower():
        if character in string.ascii_lowercase and character not in letters:
            letters.append(character)

    return ''.join(sorted(letters)) == string.ascii_lowercase
