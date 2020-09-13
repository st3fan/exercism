

def slices(series, length):
    if length > len(series) or length <= 0:
        raise ValueError("invalid length")
    return [series[i:i+length] for i in range(0, len(series) - length + 1)]
