defmodule Hextille.Logger do
  require Integer
  require Logger
  use Bitwise
  alias Hextille.Cube
  alias Hextille.Offset
  import Hextille.HexGrid, only: [boundaries: 1]

  @moduledoc """
  Utility for logging hexagonal maps for debugging purposes
  """

  @doc ~S"""
  Returns string visualization of `mapset` using cube coordinates to console with logger level info.
  """
  def visualize_cube(%MapSet{} = mapset) do
    %{:min_col => min_col,
      :min_row => min_row,
      :max_row => max_row} = boundaries(mapset)

    rows = Enum.map(min_row..max_row, fn(row) -> filter_by_row(mapset, row) end)

    Enum.map(rows, fn(row) -> draw_hexagon_row(row, min_row, max_row, min_col) end)
  end

  @doc ~S"""
  Logs visualization of `mapset` using offset coordinates to console with logger level info.
  """
  def visualize_offset(%MapSet{} = mapset) do
    %{:min_col => min_col,
      :min_row => min_row,
      :max_row => max_row} = boundaries(mapset)

    rows = Enum.map(min_row..max_row, fn(row) -> filter_by_row(mapset, row) end)

    Enum.map(rows, fn(row) -> draw_hexagon_row(
      Enum.map(row, fn(h) -> Offset.roffset_from_cube(h) end), min_row, max_row, min_col)
    end)
  end

  def draw_cube(%MapSet{} = mapset) do
    Logger.debug([ "\n" | visualize_cube(mapset) ])
  end

  def draw_offset(%MapSet{} = mapset) do
    Logger.debug([ "\n" | visualize_offset(mapset) ])
  end

  defp offset_hexagon_partial(i, type) when i == 0 do
    case type do
      :top           -> "   .^.   "
      :top_coords    -> ".´     `."
      :middle        -> "|<%= col %> <%= row %>|"
      :bottom_coords -> "^.     .^"
      :bottom        -> "   `.´   "
    end
  end

  defp offset_hexagon_partial(_, type) do
    case type do
      :top           -> "  .^.   "
      :top_coords    -> "´     `."
      :middle        -> "<%= col %> <%= row %>|"
      :bottom_coords -> ".     .^"
      :bottom        -> "  `.´   "
    end
  end

  defp cube_hexagon_partial(i, type) when i == 0 do
    case type do
      :top           -> "   .^.   "
      :top_coords    -> ".´<%= s %> <%= q %>`."
      :middle        -> "|       |"
      :bottom_coords -> "^. <%= r %> .^"
      :bottom        -> "   `.´   "
    end
  end

  defp cube_hexagon_partial(_, type) do
    case type do
      :top           -> "  .^.   "
      :top_coords    -> "´<%= s %> <%= q %>`."
      :middle        -> "       |"
      :bottom_coords -> ". <%= r %> .^"
      :bottom        -> "  `.´   "
    end
  end

  defp draw_hexagon_row_partial(%Offset{} = h, i, type) do
    partial = offset_hexagon_partial(i, type)

    case type do
      :middle -> partial
        |> EEx.eval_string([
          col: h.col |> Integer.to_string |> String.pad_leading(3),
          row: h.row |> Integer.to_string |> String.pad_trailing(3)
        ])
      _default -> partial
    end
  end

  defp draw_hexagon_row_partial(%Cube{} = h, i, type) do
    partial = cube_hexagon_partial(i, type)

    case type do
      :top_coords -> partial
        |> EEx.eval_string([
          s: h.s |> Integer.to_string |> String.pad_trailing(2),
          q: h.q |> Integer.to_string |> String.pad_leading(2)
        ])
      :bottom_coords -> partial
        |> EEx.eval_string([
          r: h.r |> Integer.to_string |> String.pad_leading(3)
        ])
      _default -> partial
    end
  end


  defp is_first_row([%Cube{} = head | _], min_row) do
    head.r == min_row
  end

  defp is_first_row([%Offset{} = head | _], min_row) do
    head.row == min_row
  end

  defp is_last_row([%Cube{} = head | _], max_row) do
    head.r == max_row
  end

  defp is_last_row([%Offset{} = head | _], max_row) do
    head.row == max_row
  end

  defp empty_columns(%Offset{} = h, min_col), do: h.col - min_col
  defp empty_columns(%Cube{} = h, min_col), do: Offset.roffset_from_cube(h).col - min_col

  defp even_row_offset(%Cube{} = h), do: (h.r + 1) &&& 1
  defp even_row_offset(%Offset{} = h), do: (h.row + 1) &&& 1

  defp offset_whitespace([head | _], min_col) do
    # Indent for 8 spaces per each empty Hexagon
    empty_cols = empty_columns(head, min_col)
    # Indent for 4 spaces on even rows
    even = even_row_offset(head)

    String.duplicate(" ", 8 * empty_cols + 4 * even)
  end

  defp maybe_draw_top(result, row, whitespace, min_row) do
    if is_first_row(row, min_row) do
      result |> draw_section(row, whitespace, :top)
    else
      result
    end
  end

  defp maybe_draw_bottom(result, row, whitespace, max_row) do
    if is_last_row(row, max_row) do
      result |> draw_section(row, whitespace, :bottom)
    else
      result
    end
  end

  defp draw_section(result, row, whitespace, type) do
    result ++
    [whitespace |
      row
      |> Enum.with_index
      |> Enum.map(fn({v, k}) -> draw_hexagon_row_partial(v, k, type) end)
      |> Kernel.++(["\n"])]
  end

  defp draw_hexagon_row(row, min_row, max_row, min_col) do
    whitespace = offset_whitespace(row, min_col)

    maybe_draw_top([], row, whitespace, min_row)
    |> draw_section(row, whitespace, :top_coords)
    |> draw_section(row, whitespace, :middle)
    |> draw_section(row, whitespace, :bottom_coords)
    |> maybe_draw_bottom(row, whitespace, max_row)
    |> Enum.join()
  end

  defp filter_by_row(%MapSet{} = mapset, row) do
    mapset
    |> Enum.filter(fn(h) -> h.r == row end)
    |> Enum.sort(fn(a, b) -> a.q <= b.q end)
  end
end
