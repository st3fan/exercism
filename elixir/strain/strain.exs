defmodule Strain do
  @doc """
  Given a `list` of items and a function `fun`, return the list of items where
  `fun` returns true.

  Do not use `Enum.filter`.
  """
  @spec keep(list :: list(any), fun :: (any -> boolean)) :: list(any)
  def keep(list, fun) do
    _keep([], list, fun)
  end

  defp _keep(results, [], _) do
    results
  end

  defp _keep(results, [head|tail], fun) do
    if fun.(head) do
      _keep(results ++ [head], tail, fun)
    else
      _keep(results, tail, fun)
    end
  end

  @doc """
  Given a `list` of items and a function `fun`, return the list of items where
  `fun` returns false.

  Do not use `Enum.reject`.
  """
  @spec discard(list :: list(any), fun :: (any -> boolean)) :: list(any)
  def discard(list, fun) do
    keep(list, fn v -> not fun.(v) end)
  end
end
