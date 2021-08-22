defmodule ListOps do
  # Please don't use any external modules (especially List or Enum) in your
  # implementation. The point of this exercise is to create these basic
  # functions yourself. You may use basic Kernel functions (like `Kernel.+/2`
  # for adding numbers), but please do not use Kernel functions for Lists like
  # `++`, `--`, `hd`, `tl`, `in`, and `length`.

  # count

  @spec count(list) :: non_neg_integer
  def count(l) do
    _count(l, 0)
  end

  defp _count([], a), do: a
  defp _count([_ | t], a), do: _count(t, a + 1)

  # reverse

  @spec reverse(list) :: list
  def reverse(l) do
    _reverse(l, [])
  end

  defp _reverse([], a), do: a
  defp _reverse([h | t], a), do: _reverse(t, [h | a])

  # map

  @spec map(list, (any -> any)) :: list
  def map(l, f) do
    _map(l, f, [])
  end

  defp _map([], _f, a), do: reverse(a)
  defp _map([h | t], f, a), do: _map(t, f, [f.(h) | a])

  # filter

  @spec filter(list, (any -> as_boolean(term))) :: list
  def filter(l, f) do
    _filter(l, f, [])
  end

  defp _filter([], _f, a), do: reverse(a)

  defp _filter([h | t], f, a) do
    if f.(h) do
      _filter(t, f, [h | a])
    else
      _filter(t, f, a)
    end
  end

  # reduce

  @type acc :: any
  @spec reduce(list, acc, (any, acc -> acc)) :: acc
  def reduce(l, acc, f) do
    _reduce(l, acc, f)
  end

  defp _reduce([], a, _), do: a
  defp _reduce([h | t], a, f), do: _reduce(t, f.(h, a), f)

  # append

  @spec append(list, list) :: list
  def append(a, b) do
    _append(reverse(a), b)
  end

  defp _append(a, []), do: reverse(a)
  defp _append([], b), do: b
  defp _append([h | t], b), do: _append(t, [h | b])

  # concat

  @spec concat([[any]]) :: [any]
  def concat(ll) do
    reduce(reverse(ll), [], &append/2)
  end
end
