defmodule Bob do
  def hey(input) do
    answer(String.trim(input), shouting?(input), String.ends_with?(input, "?"))
  end

  defp answer("", _, _) do
    "Fine. Be that way!"
  end

  defp answer(_, true, false) do
    "Whoa, chill out!"
  end

  defp answer(_, true, true) do
    "Calm down, I know what I'm doing!"
  end

  defp answer(_, _, true) do
    "Sure."
  end

  defp answer(_, _, _) do
    "Whatever."
  end

  defp shouting?(input) do
    input
    |> _letters
    |> _shouting?
  end

  defp _letters(input) do
    Enum.filter(String.codepoints(input), fn c ->
      (c =~ ~r/^\p{L}$/u)
    end)
  end

  defp _shouting?([]) do
    false
  end

  defp _shouting?(codepoints) do
    Enum.all?(codepoints, fn c ->
      (c =~ ~r/^\p{Lu}$/u)
    end)
  end
end
