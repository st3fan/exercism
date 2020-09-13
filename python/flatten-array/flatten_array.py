# A simple recursive version to walk over nested
# lists. The `yield from` syntax was introduced
# in Python 3.3 and documented in PEP380.

# This implementation assumes that only lists are
# nested. It would be better to test for a real
# Iterable to support more data types.

def _flatten(iterable):
    for value in iterable:
        if isinstance(value, list):
            yield from _flatten(value)
        elif value is not None:
            yield value

def flatten(iterable):
    return list(_flatten(iterable))
