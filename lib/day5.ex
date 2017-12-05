defmodule Day5.Part1 do
  @moduledoc """
  Documentation for Part1.
  """

  @doc ~S"""
  Counts the number of jumps it takes to escape the list of offset jump operations.
  Offsets are increased by one every time they are used.

  ## Examples:
    iex> Part1.solve("0\n3\n0\n1\n-3")
    5
  """
  def solve(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn(x) -> String.to_integer(x) end)
    |> jump_until_miss(0, 0)
  end

  defp jump_until_miss(_array, jumps, index) when index < 0, do: jumps

  # Jumps until misses, and returns then the amount of jumps it took.
  defp jump_until_miss(array, jumps, index) do
    case Enum.fetch(array, index) do
      {:ok, value} ->
        jump_until_miss(List.update_at(array, index, &(&1 + 1)), jumps + 1, index + value)
      :error ->
        jumps
    end
  end
end

defmodule Day5.Part2 do
  @moduledoc """
  Documentation for Part2.
  """

  @doc ~S"""
  Counts the number of jumps it takes to escape the list of offset jump operations.
  Every time offsets are used they are increased by one if they are three or more, and decreased otherwise.

  ## Examples:
    iex> Part1.solve("0\n3\n0\n1\n-3")
    10
  """
  def solve(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn(x) -> String.to_integer(x) end)
    |> jump_until_miss(0, 0)
  end

  # Jumps until misses, and returns then the amount of jumps it took.
  defp jump_until_miss(_array, jumps, index) when index < 0, do: jumps

  defp jump_until_miss(array, jumps, index) do
    case Enum.fetch(array, index) do
      {:ok, value} ->
        if value >= 3 do
          jump_until_miss(List.update_at(array, index, &(&1 - 1)), jumps + 1, index + value)
        else
          jump_until_miss(List.update_at(array, index, &(&1 + 1)), jumps + 1, index + value)
        end
      :error ->
        jumps
    end
  end
end
