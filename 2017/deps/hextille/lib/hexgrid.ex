defmodule Hextille.HexGrid do
  require Integer
  alias Hextille.Offset
  alias Hextille.Cube

  @moduledoc """
  Module for creating HexGrids
  """

  @doc ~S"""
  Returns a MapSet containing grid of `row` rows and `col` columns of
  hexagon tiles in Cube coordinates.
  Top left corner of the grid is at %Cube{q: 0, r: 0, s: 0}, unless offset is shifted.
  """
  def create(rows, columns, offset_col \\ 0, offset_row \\ 0) do
    create_grid(MapSet.new, rows, columns, offset_col, offset_row)
  end


  @doc ~S"""
  Finds the columns and rows representing boundaries of a MapSet.

  iex> a = HexGrid.create(4, 4, -2, -2)
  iex> b = HexGrid.create(4, 4, -3, -5)
  iex> HexGrid.boundaries(a)
  %{max_col: 2, max_row: 2, min_col: -2, min_row: -2}
  iex> HexGrid.boundaries(b)
  %{max_col: 1, max_row: -1, min_col: -3, min_row: -5}
  """
  def boundaries(%MapSet{} = mapset) do
    %{:min_col => mapset |> min_col,
      :max_col => mapset |> max_col,
      :min_row => mapset |> min_row,
      :max_row => mapset |> max_row}
  end

  defp min_col(%MapSet{} = mapset), do: Enum.min_by(cube_to_offset_mapset(mapset), fn(x) -> x.col end).col
  defp max_col(%MapSet{} = mapset), do: Enum.max_by(cube_to_offset_mapset(mapset), fn(x) -> x.col end).col
  defp min_row(%MapSet{} = mapset), do: Enum.min_by(cube_to_offset_mapset(mapset), fn(x) -> x.row end).row
  defp max_row(%MapSet{} = mapset), do: Enum.max_by(cube_to_offset_mapset(mapset), fn(x) -> x.row end).row

  @doc ~S"""
  Checks if hexagon has any neighbour in direction at MapSet

  iex> h = HexGrid.create(4, 4, -2, -2)
  iex> HexGrid.has_neighbour(h, %Cube{q: 1, r: -2, s: 1}, :east)
  true
  iex> HexGrid.has_neighbour(h, %Cube{q: 1, r: -2, s: 1}, :north_east)
  false
  """
  def has_neighbour(%MapSet{} = mapset, %Cube{} = h, direction) do
    mapset
    |> MapSet.member?(Cube.neighbour(h, direction))
  end

  @doc ~S"""
  Checks if hexagon has any neighbours in MapSet

  iex> h = HexGrid.create(4, 4, -2, -2)
  iex> HexGrid.has_neighbour(h, %Cube{q: 1, r: -2, s: 1})
  %{east: true, north_east: false, north_west: false, south_east: true,
  south_west: true, west: true}
  """
  def has_neighbour(%MapSet{} = mapset, %Cube{} = h) do
    %{:north_east => has_neighbour(mapset, h, :north_east),
      :east => has_neighbour(mapset, h, :east),
      :south_east => has_neighbour(mapset, h, :south_east),
      :south_west => has_neighbour(mapset, h, :south_west),
      :west => has_neighbour(mapset, h, :west),
      :north_west => has_neighbour(mapset, h, :north_west)}
  end

  defp create_grid(mapset, rows, _columns, _offset_col, _offset_row) when rows < 0 do
    mapset
  end

  defp create_grid(mapset, rows, columns, offset_col, offset_row) do
    mapset = create_row(mapset, rows, columns, offset_col, offset_row)
    create_grid(mapset, rows - 1, columns, offset_col, offset_row)
  end

  defp create_row(mapset, _rows, columns, _offset_col, _offset_row) when columns < 0 do
    mapset
  end

  defp create_row(mapset, rows, columns, offset_col, offset_row) do
    hexagon = Offset.roffset_to_cube(%Offset{
      col: columns + offset_col, row: rows + offset_row})

    mapset
    |> MapSet.put(hexagon)
    |> create_row(rows, columns - 1, offset_col, offset_row)
  end

  defp cube_to_offset_mapset(%MapSet{} = mapset) do
    Enum.map(mapset, fn(h) -> Offset.roffset_from_cube(h) end)
  end
end
