def distance(first='', second=''):
    if not isinstance(first, str) or not isinstance(second, str):
        raise ValueError('Improper arguments')
    if len(first) != len(second):
        raise ValueError('Arguments are different lengths')

    hamming_distance = 0

    for position in range(0, len(first)):
        if first[position] != second[position]:
            hamming_distance += 1

    return hamming_distance
