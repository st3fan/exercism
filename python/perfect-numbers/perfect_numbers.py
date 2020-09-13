def _factors(n):
    for factor in range(1, n // 2 + 1):
        if n % factor == 0:
            yield factor


def _aliquot_sum(n):
    return sum(_factors(n))


def classify(number):
    if number <= 0:
        raise ValueError("number cannot be zero negative")
    aliquot_sum = _aliquot_sum(number)
    if aliquot_sum == number:
        return "perfect"
    elif aliquot_sum < number:
        return "deficient"
    else:
        return "abundant"


