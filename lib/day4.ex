defmodule Day4.Part1 do
  @moduledoc """
  Documentation for Part1.
  """

  @doc ~S"""
  Checks if passphrase is legit. Passpharse should not
  contain same word more than once.

  ## Examples:
    iex> Part1.solve("aa bb cc dd ee\naa bb cc dd aa")
    1
    iex> Part1.solve("aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd ee")
    2
    iex> Part1.solve("aa bb cc dd aaa")
    1
  """
  def solve(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn(x) -> is_valid(x) end)
    |> Enum.reduce(0, fn(x, acc) -> if x, do: acc + 1, else: acc end) 
  end

  @doc """
  Checks if passphrase is legit. Passpharse should not
  contain same word more than once.

  ## Examples:
    iex> Part1.is_valid("aa bb cc dd ee")
    true
    iex> Part1.is_valid("aa bb cc dd aa")
    false
    iex> Part1.is_valid("aa bb cc dd aaa")
    true
  """
  def is_valid(passphrase) do
    splitted = String.split(passphrase, " ")
    unique = Enum.uniq(splitted)
    length(splitted) == length(unique)
  end
end
