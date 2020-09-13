# This can be done with list/dict comprehensions
# but I find two loops like this more readable.

def transform(legacy_data):
    scores = {}
    for score, letters in legacy_data.items():
        for letter in letters:
            scores[letter.lower()] = score
    return scores
