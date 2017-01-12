"""
    Simple function to calculate whether it is a leap year or not.
"""


def is_leap_year(year=None):
    """
        Simple function to calculate whether it is a leap year or not.

        A leap year in the Gregorian calendar occurs:
         - on every year that is evenly divisible by 4
          - except every year that is evenly divisible by 100
          - unless the year is also evenly divisible by 400
    """
    if year is None or not isinstance(year, int):
        return 0

    leap_year = False
    if year % 4 == 0:
        leap_year = True
        if year % 100 == 0:
            leap_year = False
            if year % 400 == 0:
                leap_year = True

    return leap_year
