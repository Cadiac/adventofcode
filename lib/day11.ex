
defmodule Day11.Part1 do
  @moduledoc """
  --- Day 11: Hex Ed ---

  Crossing the bridge, you've barely reached the other side of the stream when
  a program comes up to you, clearly in distress. "It's my child process," she says,
  "he's gotten lost in an infinite grid!"

  Fortunately for her, you have plenty of experience with infinite grids.

  Unfortunately for you, it's a hex grid.

  The hexagons ("hexes") in this grid are aligned such that adjacent hexes can be found
  to the north, northeast, southeast, south, southwest, and northwest:

    \ n  /
  nw +--+ ne
    /    \
  -+      +-
    \    /
  sw +--+ se
    / s  \
  
  """
  alias Hextille

  @doc ~S"""
  You have the path the child process took. Starting where he started, you need to
  determine the fewest number of steps required to reach him.
  (A "step" means to move from the hex you are in to any adjacent hex.) 

  I really really want to use my own Hextille library here,
  but unfortunately it deals with pointy topped hexagon tiles :<

  Lets try anyways.

  Since Hextille deals with pointy-topped hexagons, I need to change how
  directions work by turning them 60 degrees counter clockwise, so that
  "n" becomes :north_west etc.
  
  ## Examples:
    iex> Part1.solve("ne,ne,ne")
    3
    iex> Part1.solve("ne,ne,sw,sw")
    0
    iex> Part1.solve("ne,ne,s,s")
    2
    iex> Part1.solve("se,sw,se,sw,sw")
    3
  """
  def solve(input) do
    input
    |> String.split(",", trim: true)
    |> Enum.reduce(Hextille.Cube.create!(0, 0, 0), fn(direction, position) ->
      case direction do
        "n"  -> Hextille.Cube.neighbour(position, :north_west)
        "ne" -> Hextille.Cube.neighbour(position, :north_east)
        "se" -> Hextille.Cube.neighbour(position, :east)
        "s"  -> Hextille.Cube.neighbour(position, :south_east)
        "sw" -> Hextille.Cube.neighbour(position, :south_west)
        "nw" -> Hextille.Cube.neighbour(position, :west)
      end
    end)
    |> Hextille.Cube.length
  end
end

defmodule Day11.Part2 do
  alias Hextille

  @doc ~S"""
  --- Part Two ---

  How many steps away is the furthest he ever got from his starting position?

  ## Examples:
    iex> Part2.solve("ne,ne,ne")
    3
    iex> Part2.solve("ne,ne,sw,sw")
    2
    iex> Part2.solve("ne,ne,s,s")
    2
    iex> Part2.solve("se,sw,se,sw,sw")
    3
  """
  def solve(input) do
    input
    |> String.split(",", trim: true)
    |> Enum.reduce({Hextille.Cube.create!(0, 0, 0), 0}, 
      fn(direction, {position, furthest}) ->
        neighbour = find_neighbour(position, direction)
        distance = Hextille.Cube.length(neighbour)

        if distance > furthest do
          {neighbour, distance}
        else
          {neighbour, furthest}
        end
      end)
    |> elem(1)
  end

  defp find_neighbour(position, direction) do
    case direction do
      "n"  -> Hextille.Cube.neighbour(position, :north_west)
      "ne" -> Hextille.Cube.neighbour(position, :north_east)
      "se" -> Hextille.Cube.neighbour(position, :east)
      "s"  -> Hextille.Cube.neighbour(position, :south_east)
      "sw" -> Hextille.Cube.neighbour(position, :south_west)
      "nw" -> Hextille.Cube.neighbour(position, :west)
    end
  end
end
