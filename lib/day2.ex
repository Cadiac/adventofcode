defmodule Day2.Part1 do
  @moduledoc """
  Documentation for Part1.
  """

  @doc """
  Solves the spreadsheet checksum with min-max values

  ## Examples:
    iex> Part1.solve("5,1,9,5;7,5,3;2,4,6,8")
    18
  """
  def solve(input) do
    input
    |> String.split(";", trim: true)
    |> Enum.map(fn(x) -> String.split(x, ",", trim: true) end)
    |> Enum.map(fn(x) -> checksum_row(x) end)
    |> Enum.sum
  end

  defp checksum_row(row) do
    row_as_integers = Enum.map(row, fn(x) -> String.to_integer(x) end)

    Enum.max(row_as_integers) - Enum.min(row_as_integers)
  end
end

defmodule Day2.Part2 do
  @moduledoc """
  Documentation for Part2.
  """

  @doc """
  Solves the spreadsheet checksum with evenly divisible values

  ## Examples:
    iex> Part2.solve("5,9,2,8;9,4,7,3;3,8,6,5")
    9
  """
  def solve(input) do
    input
    |> String.split(";", trim: true)
    |> Enum.map(fn(x) -> String.split(x, ",", trim: true) end)
    |> Enum.map(fn(x) -> checksum_row(x) end)
    |> Enum.sum
  end

  defp checksum_row(row) do
    r = row
    |> Enum.map(fn(x) -> String.to_integer(x) end)
    |> Enum.with_index

    r
    |> Enum.reduce_while(0, fn(e, acc) -> find_even_division(e, acc, r) end)
  end

  defp find_even_division({value, index}, _acc, row) do
    even = Enum.find(row, fn({v, i}) -> index != i && rem(value, v) == 0 end)

    if is_nil(even) do
      {:cont, 0}
    else
      {:halt, div(value, elem(even, 0))}
    end
  end
end