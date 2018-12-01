defmodule Hextille.Offset do
  alias Hextille.Offset, as: Offset
  alias Hextille.Cube
  use Bitwise

  @moduledoc """
  Hexagon module that represents hexagon tiles using offset coordinates.
  Instead of names x, y this module uses names col and row.

  Coordinates in q-offset system represent hexagons in pointy top orientation,
  r-offset in flat top orientation.

  By default this module uses even-r and even-q offset coordinates,
  but the offset can be optionally specified.

  This module should only be used to display coordinates as offset coordinates.
  All math should be done by using cube coordinates.
  ```
  """

  defstruct col: 0, row: 0

  @doc """
  Converts hexagon in Cube coordinates to pointy top r-offset coordinates.

  ## Examples:
    iex> h = %Cube{q: 4, r: 3, s: -7}
    iex> Offset.roffset_from_cube(h)
    %Offset{col: 5, row: 3}
    iex> Offset.roffset_from_cube(h, 0)
    %Offset{col: 5, row: 3}
    iex> Offset.roffset_from_cube(h, 1)
    %Offset{col: 6, row: 3}
  """
  def roffset_from_cube(%Cube{} = h, offset \\ 0) do
    col = h.q + div((h.r + offset * (h.r &&& 1)), 2)
    row = h.r

    %Offset{col: col, row: row}
  end

  @doc """
  Converts Offset in pointy top r-offset coordinates to Cube.

  ## Examples:
    iex> a = %Offset{col: 5, row: 3}
    iex> b = %Offset{col: 6, row: 3}
    iex> Offset.roffset_to_cube(a)
    %Cube{q: 4, r: 3, s: -7}
    iex> Offset.roffset_to_cube(a, 0)
    %Cube{q: 4, r: 3, s: -7}
    iex> Offset.roffset_to_cube(b, 1)
    %Cube{q: 4, r: 3, s: -7}
  """
  def roffset_to_cube(%Offset{} = h, offset \\ 0) do
    q = h.col - div((h.row + offset * (h.row &&& 1)), 2)
    r = h.row
    s = -q - r

    %Cube{q: q, r: r, s: s}
  end

  @doc """
  Converts Cube in cube coordinates to flat top q-offset coordinates.

  ## Examples:
    iex> h = %Cube{q: 3, r: 4, s: -7}
    iex> Offset.qoffset_from_cube(h)
    %Offset{col: 3, row: 5}
    iex> Offset.qoffset_from_cube(h, 0)
    %Offset{col: 3, row: 5}
    iex> Offset.qoffset_from_cube(h, 1)
    %Offset{col: 3, row: 6}
  """
  def qoffset_from_cube(%Cube{} = h, offset \\ 0) do
    col = h.q
    row = h.r + div((h.q + offset * (h.q &&& 1)), 2)

    %Offset{col: col, row: row}
  end

  @doc """
  Converts Offset in flat top q-offset coordinates to Cube.

  ## Examples:
    iex> a = %Offset{col: 3, row: 5}
    iex> b = %Offset{col: 3, row: 6}
    iex> Offset.qoffset_to_cube(a)
    %Cube{q: 3, r: 4, s: -7}
    iex> Offset.qoffset_to_cube(a, 0)
    %Cube{q: 3, r: 4, s: -7}
    iex> Offset.qoffset_to_cube(b, 1)
    %Cube{q: 3, r: 4, s: -7}
  """
  def qoffset_to_cube(%Offset{} = h, offset \\ 0) do
    q = h.col;
    r = h.row - div((h.col + offset * (h.col &&& 1)), 2)
    s = -q - r

    %Cube{q: q, r: r, s: s}
  end
end
