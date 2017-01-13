"""
    Simple function to return the RNA compliment of a provided DNA string.
"""


def to_rna(dna=''):
    if dna is '' or not isinstance(dna, str):
        return ''

    rna = ''
    for character in dna.upper():
        if character == 'G':
            rna += 'C'
        elif character == 'C':
            rna += 'G'
        elif character == 'T':
            rna += 'A'
        elif character == 'A':
            rna += 'U'
        else:
            return ''

    return rna
