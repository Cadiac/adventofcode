defmodule Day1.Part1 do
  @moduledoc """
  Documentation for Part1.
  """

  @doc """
  Solves the inverse captcha puzzle

  ## Examples:
    iex> Part1.solve("1122")
    3
    iex> Part1.solve("1111")
    4
    iex> Part1.solve("1234")
    0
    iex> Part1.solve("91212129")
    9
  """
  def solve(input) do
    String.split(input, "", trim: true)
    |> Enum.with_index
    |> Enum.map(fn({value, index}) -> compare(input, value, String.at(input, index + 1)) end)
    |> Enum.sum
  end

  defp compare(input, current, next) when is_nil(next) do
    if current == String.first(input) do
      String.to_integer(current)
    else
      0
    end
  end

  defp compare(_input, current, next) do
    if current == next do
      String.to_integer(current)
    else
      0
    end
  end
end

defmodule Day1.Part2 do
  @moduledoc """
  Documentation for Part2.
  """

  @doc """
  Solves the inverse captcha puzzle part 2

  ## Examples:
    iex> Part2.solve("1212")
    6
    iex> Part2.solve("1221")
    0
    iex> Part2.solve("123425")
    4
    iex> Part2.solve("123123")
    12
    iex> Part2.solve("12131415")
    4
  """
  def solve(input) do
    length = String.length(input)

    String.split(input, "", trim: true)
    |> Enum.with_index
    |> Enum.map(fn({value, index}) -> compare(value, String.at(input, next_index(index, length))) end)
    |> Enum.sum
  end

  @doc """
  Calculates next index

  ## Examples:
    iex> Part2.next_index(0, 4)
    2
    iex> Part2.next_index(1, 4)
    3
    iex> Part2.next_index(2, 4)
    0
    iex> Part2.next_index(3, 4)
    1
  """
  def next_index(index, length) do
    next = index + div(length, 2)

    rem(next, length)
  end

  defp compare(current, next) do
    if current == next do
      String.to_integer(current)
    else
      0
    end
  end
end