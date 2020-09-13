# I decided to go with a textbook recursive implementation of a binary
# search function. Python has a default recursion limit which you are
# unlikely to hit. This is however not very space efficient as large
# chunks of the items are being copied.

def _find(items, value, index):
    if len(items) == 1:
        if items[0] != value:
            raise ValueError("value not found")
        return index
    mid = len(items) // 2
    if items[mid] > value:
        return _find(items[:mid], value, index)
    else:
        return _find(items[mid:], value, index + mid)

def find(search_list, value):
    if not len(search_list):
        raise ValueError("search_list is empty")
    return _find(search_list, value, 0)
