defmodule SecretHandshake do
  use Bitwise, only_operators: true
  #import Bitwise, only: [band: 2, bxor: 2]

  @doc """
  Determine the actions of a secret handshake based on the binary
  representation of the given `code`.

  If the following bits are set, include the corresponding action in your list
  of commands, in order from lowest to highest.

  1 = wink
  10 = double blink
  100 = close your eyes
  1000 = jump

  10000 = Reverse the order of the operations in the secret handshake
  """
  @spec commands(code :: integer) :: list(String.t())
  def commands(code) do
    _process_command(code, [])
  end

  defp _process_command(0, result) do
    result
  end

  defp _process_command(code, result) when (code &&& 1) != 0 do
    _process_command(code &&& 0b11110, result ++ ["wink"])
  end

  defp _process_command(code, result) when (code &&& 2) != 0 do
    _process_command(code &&& 0b11101, result ++ ["double blink"])
  end

  defp _process_command(code, result) when (code &&& 4) != 0 do
    _process_command(code &&& 0b11011, result ++ ["close your eyes"])
  end

  defp _process_command(code, result) when (code &&& 8) != 0 do
    _process_command(code &&& 0b10111, result ++ ["jump"])
  end

  defp _process_command(code, result) when (code &&& 16) != 0 do
    _process_command(code &&& 0b01111, Enum.reverse(result))
  end

  defp _process_command(code, result) when (code &&& 0b11111) == 0 do
    []
  end
end
