from functools import reduce


def append(list1, list2):
    return list1+list2


def concat(lists):
    result = []
    for list in lists:
        result += list
    return result


def filter(function, list):
    return [v for v in list if function(v)]


def length(list):
    n = 0
    for _ in list:
        n += 1
    return n


def map(function, list):
    return [function(v) for v in list]


def foldl(function, list, initial):
    result = initial
    for v in list:
        result = function(result, v)
    return result


def foldr(function, list, initial):
    result = initial
    for v in reversed(list):
        result = function(v, result)
    return result


def reverse(list):
    result = []
    for i in list:
        result = [i] + result
    return result

