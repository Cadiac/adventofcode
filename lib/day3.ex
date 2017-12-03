defmodule Day3.Cell do
  @moduledoc """
  Documentation for a Cell in matrix
  """
  alias Day3.Cell

  defstruct x: 0, y: 0, v: nil, going: nil, sum: nil

  def add(%Cell{} = a, %Cell{} = b) do
    %Cell{x: a.x + b.x, y: a.y + b.y}
  end

  def add(%Cell{} = a, %Cell{} = b, v, going) do
    %Cell{x: a.x + b.x, y: a.y + b.y, v: v, going: going}
  end
end

defmodule Day3.Matrix do
  @moduledoc """
  Documentation for Matrix which stores Cells
  """

  alias Day3.Cell, as: Cell

  @doc """
  Creates a Matrix

  ## Examples:
    iex> Matrix.create |> Stream.drop(4) |> Enum.take(1) |> List.first
    [%Cell{going: :south, v: 5, x: -1, y: 1},
     %Cell{going: :west, v: 4, x: 0, y: 1},
     %Cell{going: :west, v: 3, x: 1, y: 1},
     %Cell{going: :north, v: 2, x: 1, y: 0},
     %Cell{going: :east, v: 1, x: 0, y: 0}]
  """
  def create() do
    first = %Cell{x: 0, y: 0, v: 1, going: :east}

    Stream.unfold([first], fn cells -> {cells, create_cells(cells)} end)
  end

  defp create_cells([%Cell{} = previous | _tail] = cells) do
    [next_cell(previous) | cells]
  end

  @doc """
  Creates Stream of Cells representing a Matrix with neighbour values sum as value

  ## Examples:
    iex> Matrix.create_with_neighbours() |> Stream.drop(4) |> Enum.take(1) |> List.first
    [%Cell{going: :south, sum: 5, v: 5, x: -1, y: 1},
     %Cell{going: :west, sum: 4, v: 4, x: 0, y: 1},
     %Cell{going: :west, sum: 2, v: 3, x: 1, y: 1},
     %Cell{going: :north, sum: 1, v: 2, x: 1, y: 0},
     %Cell{going: :east, sum: 1, v: 1, x: 0, y: 0}]
  """
  def create_with_neighbours() do
    first = %Cell{x: 0, y: 0, v: 1, going: :east, sum: 1}

    Stream.unfold([first], fn cells -> {cells, create_cells_with_neighbours(cells)} end)
  end

  defp create_cells_with_neighbours([%Cell{} = previous | _tail] = cells) do
    next = next_cell(previous)
    sum = find_neighbours_values(cells, next)

    [%{next | sum: sum} | cells]
  end

  @doc """
  Finds next Cell and the direction it is 'going' to, forming a counter-clockwise spiral

  ## Examples:
    iex> Matrix.next_cell(%Cell{v: 1, x: 0, y: 0, going: :east})
    %Cell{v: 2, x: 1, y: 0, going: :north}
    iex> Matrix.next_cell(%Cell{v: 2, x: 1, y: 0, going: :north})
    %Cell{v: 3, x: 1, y: 1, going: :west}
    iex> Matrix.next_cell(%Cell{v: 3, x: 1, y: 1, going: :west})
    %Cell{v: 4, x: 0, y: 1, going: :west}
    iex> Matrix.next_cell(%Cell{v: 4, x: 0, y: 1, going: :west})
    %Cell{v: 5, x: -1, y: 1, going: :south}
    iex> Matrix.next_cell(%Cell{v: 5, x: -1, y: 1, going: :south})
    %Cell{v: 6, x: -1, y: 0, going: :south}
    iex> Matrix.next_cell(%Cell{v: 6, x: -1, y: 0, going: :south})
    %Cell{v: 7, x: -1, y: -1, going: :east}
    iex> Matrix.next_cell(%Cell{v: 7, x: -1, y: -1, going: :east})
    %Cell{v: 8, x: 0, y: -1, going: :east}
    iex> Matrix.next_cell(%Cell{v: 8, x: 0, y: -1, going: :east})
    %Cell{v: 9, x: 1, y: -1, going: :east}
    iex> Matrix.next_cell(%Cell{v: 9, x: 1, y: -1, going: :east})
    %Cell{v: 10, x: 2, y: -1, going: :north}
  """
  def next_cell(%Cell{v: prev_v, going: going} = previous) do
    prev_w = width(prev_v)
    prev_h = height(prev_v)

    next_v = prev_v + 1
    
    next_w = width(next_v)
    next_h = height(next_v)

    case going do
      :east ->
        if prev_w < next_w do
          Cell.add(previous, directions(:east), next_v, :north)
        else
          Cell.add(previous, directions(:east), next_v, :east)
        end
      :west ->
        if prev_w < next_w do
          Cell.add(previous, directions(:west), next_v, :south)
        else
          Cell.add(previous, directions(:west), next_v, :west)
        end
      :north ->
        if prev_h < next_h do
          Cell.add(previous, directions(:north), next_v, :west)
        else
          Cell.add(previous, directions(:north), next_v, :north)
        end
      :south ->
        if prev_h < next_h do
          Cell.add(previous, directions(:south), next_v, :east)
        else
          Cell.add(previous, directions(:south), next_v, :south)
        end
    end
  end

  # Returns a vector representing movement to direction
  defp directions(direction) do
    case direction do
      :west       -> %Cell{x: -1, y: 0}
      :east       -> %Cell{x: 1, y: 0}
      :north      -> %Cell{x: 0, y: 1}
      :south      -> %Cell{x: 0, y: -1}
      :northeast  -> %Cell{x: 1, y: 1}
      :southeast  -> %Cell{x: 1, y: -1}
      :southwest  -> %Cell{x: -1, y: -1}
      :northwest  -> %Cell{x: -1, y: 1}
    end
  end

  # Finds the required width of grid to draw the spiral ending to number
  defp width(number) do
    round(Float.ceil(:math.sqrt(number)))
  end

  # Finds the required height of grid to draw the spiral ending to number
  defp height(number) do
    round(:math.sqrt(number))
  end

  # Finds all neighbours that currently exist for cell in cells
  defp find_neighbours_values(cells, %Cell{} = cell) do
    potential_neighbours = [
      Cell.add(cell, directions(:north)),
      Cell.add(cell, directions(:northeast)),
      Cell.add(cell, directions(:east)),
      Cell.add(cell, directions(:southeast)),
      Cell.add(cell, directions(:south)),
      Cell.add(cell, directions(:southwest)),
      Cell.add(cell, directions(:west)),
      Cell.add(cell, directions(:northwest))
    ]

    potential_neighbours
    |> Enum.map(fn(neighbour) ->
        Enum.find(cells, fn(c) -> 
          neighbour.x == c.x && neighbour.y == c.y
        end)
      end)
    |> Enum.reduce(0, fn(x, acc) ->
      if is_nil(x), do: acc, else: acc + x.sum
    end)
  end
end

defmodule Day3.Part1 do
  @moduledoc """
  Documentation for Part1.
  """

  alias Day3.Matrix, as: Matrix

  @doc """
  Solves the distance from input to middle

  36  35  34  33  32  31
  17  16  15  14  13  30
  18   5   4   3  12  29
  19   6   1   2  11  28
  20   7   8   9  10  27
  21  22  23  24  25  26

  ## Examples:
    iex> Part1.solve(1)
    0
    iex> Part1.solve(12)
    3
    iex> Part1.solve(23)
    2
    iex> Part1.solve(1024)
    31
    iex> Part1.solve(347991)
    480
  """
  def solve(input) do
    Matrix.create
    |> Stream.drop(input - 1)
    |> Enum.take(1)
    |> List.first
    |> List.first
    |> distance
  end

  defp distance(position) do
    abs(position.x) + abs(position.y)
  end
end

defmodule Day3.Part2 do
  @moduledoc """
  Documentation for Part2.
  """

  alias Day3.Matrix, as: Matrix
  alias Day3.Cell, as: Cell
  
  @doc """
  Returns value written that is at v

  147  142  133  122   59
  304    5    4    2   57
  330   10    1    1   54
  351   11   23   25   26
  362  747  806--->   ...

  ## Examples:
    iex> Part2.solve_value(1)
    1
    iex> Part2.solve_value(2)
    1
    iex> Part2.solve_value(3)
    2
    iex> Part2.solve_value(4)
    4
    iex> Part2.solve_value(5)
    5
    iex> Part2.solve_value(6)
    10
    iex> Part2.solve_value(7)
    11
    iex> Part2.solve_value(8)
    23
    iex> Part2.solve_value(9)
    25
    iex> Part2.solve_value(10)
    26
  """
  def solve_value(input) do
    Matrix.create_with_neighbours
    |> Stream.drop_while(fn([head | _tail]) -> head.v < input end)
    |> Enum.take(1)
    |> List.first
    |> List.first
    |> pick_sum
  end

  @doc """
  Returns the first value written that is larger than puzzle input

  147  142  133  122   59
  304    5    4    2   57
  330   10    1    1   54
  351   11   23   25   26
  362  747  806--->   ...

  ## Examples:
    iex> Part2.solve(1)
    1
    iex> Part2.solve(20)
    23
    iex> Part2.solve(60)
    122
    iex> Part2.solve(750)
    806
    iex> Part2.solve(347991)
    349975
  """
  def solve(input) do    
    Matrix.create_with_neighbours
    |> Stream.drop_while(fn([head | _tail]) -> head.sum < input end)
    |> Enum.take(1)
    |> List.first
    |> List.first
    |> pick_sum
  end

  defp pick_sum(%Cell{sum: sum}) do
    sum
  end
end