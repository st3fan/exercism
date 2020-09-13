def is_isogram(string):
    letters = [c.lower() for c in string if c.isalpha()]
    return len(letters) == len(set(letters))
