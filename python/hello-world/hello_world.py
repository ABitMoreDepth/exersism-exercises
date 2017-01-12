"""
    Skeleton file for the Python "Hello World" exercise.
"""


def hello(name=''):
    """
        Simple function to return a name, or if not set return 'Hello, World!'
    """

    if name == '' or name is None:
        return u'Hello, World!'
    else:
        return u'Hello, {}!'.format(name)
