defmodule RotationalCipher do
  @doc """
  Given a plaintext and amount to shift by, return a rotated string.

  Example:
  iex> RotationalCipher.rotate("Attack at dawn", 13)
  "Nggnpx ng qnja"
  """
  @spec rotate(text :: String.t(), shift :: integer) :: String.t()
  def rotate(text, shift) do
    text
    |> String.to_charlist()
    |> Enum.map(&(rotate_codepoint(&1, shift)))
    |> to_string
  end

  defp rotate_codepoint(c, shift) when c >= ?a and c <= ?z do
    ?a + rem(((c - ?a) + shift), 26)
  end

  defp rotate_codepoint(c, shift) when c >= ?A and c <= ?Z do
    ?A + rem(((c - ?A) + shift), 26)
  end

  defp rotate_codepoint(c, _) do
    c
  end
end
