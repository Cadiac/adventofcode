defmodule Day6.Part1 do
  @moduledoc """
  Documentation for Part1.
  """

  @doc ~S"""
  Given the initial block counts in your puzzle input, how many redistribution cycles
  must be completed before a configuration is produced that has been seen before?

  ## Examples:
    iex> Part1.solve("0,2,7,0")
    5
  """
  def solve(input) do
    input
    |> String.split(",", trim: true)
    |> Enum.map(fn c -> String.to_integer(c) end)
    |> step_until_repeat
    |> length
  end

  def step_until_repeat(array, seen \\ []) do
    {index, max} = find_max_with_index(array)
    
    spread = spread_max(array, max, index)

    if Enum.member?(seen, spread) do
      [spread | seen]
    else
      step_until_repeat(spread, [spread | seen])
    end
  end

  @doc ~S"""
  Takes the value of max, and then spreads it evenly the value to all slots, starting from next index.

  ## Examples:
    iex> Part1.spread_max([0, 2, 7, 0], 7, 2)
    [2, 4, 1, 2]
    iex> Part1.spread_max([2, 4, 1, 2], 4, 1)
    [3, 1, 2, 3]
    iex> Part1.spread_max([3, 1, 2, 3], 3, 0)
    [0, 2, 3, 4]
  """
  def spread_max(array, max, index) do
    arr = List.update_at(array, index, fn _ -> 0 end)

    Enum.reduce(1..max, arr, fn(x, acc) -> 
      List.update_at(
        acc,
        rem(index + x, length(acc)),
        fn val -> val + 1 end)
    end)
  end

  defp find_max_with_index(array) do
    max = Enum.max(array)
    index = Enum.find_index(array, fn x -> x == max end)

    {index, max}
  end
end

defmodule Day6.Part2 do
  @moduledoc """
  Documentation for Part2.
  """

  alias Day6.Part1

  @doc ~S"""
  How many cycles are in the infinite loop that arises from the
  configuration in your puzzle input?

  ## Examples:
    iex> Part2.solve("0,2,7,0")
    4
  """
  def solve(input) do
    array = input
    |> String.split(",", trim: true)
    |> Enum.map(fn c -> String.to_integer(c) end)
    |> Part1.step_until_repeat

    first_repeating = hd(array)
    reversed = Enum.reverse(array)

    length(array) - Enum.find_index(reversed, fn x -> x == first_repeating end) - 1
  end
end
