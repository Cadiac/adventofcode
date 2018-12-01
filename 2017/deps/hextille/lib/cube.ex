defmodule Hextille.Cube do
  alias Hextille.Cube, as: Cube

  @moduledoc """
  Cube module that represents hexagon tiles using Cube coordinates.
  Instead of names x, y, z this module uses names q, r, s.

  Cube coordinates have a constraint `q + r + s = 0`,
  even with floating point cube coordinates. This has to be always respected.

  Axes are aligned in the following order:
  ```
         -r
    +s   .^.   +q
      .´     `.
      |       |
      `.     .´
    -q   `.´   -s
         +r
  ```
  """

  defstruct q: 0, r: 0, s: 0

  @doc """
  Creates a Cube or throws `ArgumentError` if the given arguments don't
  satisfy constraint `q + r + s = 0`. This function should be the preferred way
  to create new Hexagons when using this module.

  ## Examples:
    iex> Cube.create!(1, -2, 1)
    %Cube{q: 1, r: -2, s: 1}
    iex> Cube.create!(4, -2, 1)
    ** (ArgumentError) Invalid coordinates, constraint q + r + s = 0
  """
  def create!(q, r, s) do
    if q + r + s == 0 do
      %Cube{q: q, r: r, s: s}
    else
      raise ArgumentError, message: "Invalid coordinates, constraint q + r + s = 0"
    end
  end

  @doc """
  Addition of cube coordinates.
  Returns a new Cube with added coordinates.

  ## Examples:
    iex> a = %Cube{q: 1, r: -2, s: 1}
    iex> b = %Cube{q: 3, r: -2, s: -1}
    iex> Cube.add(a, b)
    %Cube{q: 4, r: -4, s: 0}
  """
  def add(%Cube{} = a, %Cube{} = b) do
    %Cube{
      q: a.q + b.q,
      r: a.r + b.r,
      s: a.s + b.s
    }
  end

  @doc """
  Subtraction of cube coordinates.
  Returns a new Cube with subtracted coordinates.

  ## Examples:
    iex> a = %Cube{q: 1, r: -2, s: 1}
    iex> b = %Cube{q: 3, r: -2, s: -1}
    iex> Cube.subtract(a, b)
    %Cube{q: -2, r: 0, s: 2}
  """
  def subtract(%Cube{} = a, %Cube{} = b) do
    %Cube{
      q: a.q - b.q,
      r: a.r - b.r,
      s: a.s - b.s
    }
  end


  @doc """
  Scale cube coordinates with given multiplier `k`.
  Returns a new scaled Cube.

  ## Examples:
    iex> a = %Cube{q: 1, r: -2, s: 1}
    iex> Cube.scale(a, 3)
    %Cube{q: 3, r: -6, s: 3}
  """
  def scale(%Cube{} = h, k) when is_integer(k) do
    %Cube{
      q: h.q * k,
      r: h.r * k,
      s: h.s * k
    }
  end

  @doc ~S"""
  Rotate cube coordinates 60° to given direction `:left` or `:right`.
  Returns a new Cube at rotated position.

  Rotations visualized
  ```

           .^.     .^.     .^.
        .´2   0`.´     `.´0   2`.
        |       |       |       |
       .^. -2  .^.     .^. -2  .^.
    .´     `.´     `.´     `.´     `.
    |       |       |       |       |
    `.     .^.     .^.     .^.     .^.
       `.´     `.´s   q`.´     `.´-2  2`.
        |       |       |       |       |
        `.     .^.  r  .^.     .^.  0  .´
           `.´     `.´     `.´     `.´
            |       |       |       |
            `.     .^.     .^.     .´
               `.´     `.´     `.´

  ```

  ## Examples:
    iex> a = %Cube{q: 2, r: -2, s: 0}
    iex> Cube.rotate(a, :left)
    %Cube{q: 0, r: -2, s: 2}
    iex> Cube.rotate(a, :right)
    %Cube{q: 2, r: 0, s: -2}
  """
  def rotate(%Cube{} = h, direction) do
    case direction do
      :left -> %Cube{q: -h.s, r: -h.q, s: -h.r}
      :right -> %Cube{q: -h.r, r: -h.s, s: -h.q}
    end
  end

  @doc ~S"""
  Returns a new Cube representing the `direction` as a vector.

  Directions are enumerated in following order:
  ```

  :north_west        :north_east
               .^.
            .´s   q`.
    :west   |       |  :east
            `.  r  .´
               `.´
  :south_west        :south_east

  ```

  ## Examples:
    iex> Cube.directions(:north_east)
    %Cube{q: 1, r: -1, s: 0}

    iex> Cube.directions(:east)
    %Cube{q: 1, r: 0, s: -1}

    iex> Cube.directions(:south_east)
    %Cube{q: 0, r: 1, s: -1}

    iex> Cube.directions(:south_west)
    %Cube{q: -1, r: 1, s: 0}

    iex> Cube.directions(:west)
    %Cube{q: -1, r: 0, s: 1}

    iex> Cube.directions(:north_west)
    %Cube{q: 0, r: -1, s: 1}
  """
  def directions(direction) do
    case direction do
      :north_east -> %Cube{q: 1, r: -1, s: 0}
      :east -> %Cube{q: 1, r: 0, s: -1}
      :south_east -> %Cube{q: 0, r: 1, s: -1}
      :south_west -> %Cube{q: -1, r: 1, s: 0}
      :west -> %Cube{q: -1, r: 0, s: 1}
      :north_west -> %Cube{q: 0, r: -1, s: 1}
    end
  end

  @doc ~S"""
  Finds the neighbouring tile of hexagon `a` at the given `direction`.
  Returns a new Cube representing the neighbour Cube.

  Directions are enumerated in following order:
  ```

  :north_west        :north_east
               .^.
            .´s   q`.
    :west   |       |  :east
            `.  r  .´
               `.´
  :south_west        :south_east

  ```

  Neighbours visualized
  ```

           .^.     .^.
        .´1   1`.´0   2`.
        |       |       |
       .^. -2  .^. -2  .^.
    .´1   0`.´0   1`.´-1  2`.
    |       |       |       |
    `. -1  .^. -1  .^. -1  .´
       `.´0   0`.´-1  1`.´
        |       |       |
        `.  0  .^.  0  .´
           `.´     `.´

  ```

  ## Examples:

    iex> a = %Cube{q: 1, r: -1, s: 0}
    iex> Cube.neighbour(a, :north_east)
    %Cube{q: 2, r: -2, s: 0}
    iex> Cube.neighbour(a, :north_west)
    %Cube{q: 1, r: -2, s: 1}
  """
  def neighbour(%Cube{} = h, direction) do
    Cube.add(h, Cube.directions(direction))
  end

  @doc ~S"""
  Returns the distance of hexagon `a` from origo as an integer.

  ## Examples:

    iex> a = %Cube{q: 1, r: -2, s: 1}
    iex> Cube.length(a)
    2

    iex> b = %Cube{q: -2, r: -3, s: 5}
    iex> Cube.length(b)
    5
  """
  def length(%Cube{} = h) do
    div((abs(h.q) + abs(h.r) + abs(h.s)), 2)
  end

  @doc ~S"""
  Calculates the distance between hexagons `a` and `b`.
  Return value is an integer value.

  ## Examples:
    iex> a = %Cube{q: 1, r: -2, s: 1}
    iex> b = %Cube{q: -2, r: -3, s: 5}
    iex> Cube.distance(a, b)
    4
  """
  def distance(%Cube{} = a, %Cube{} = b) do
    Cube.length(Cube.subtract(a, b))
  end

  @doc ~S"""
  Rounds up an hexagon with float values to integer coordinates.

  ## Examples:
    iex> a = %Cube{q: 1.5, r: -2.25, s: 0.75}
    iex> Cube.round_hex(a)
    %Cube{q: 1, r: -2, s: 1}

    iex> b = %Cube{q: 1.2, r: 2.5, s: -3.7}
    iex> Cube.round_hex(b)
    %Cube{q: 1, r: 3, s: -4}
  """
  def round_hex(%Cube{} = h) do
    q = round(h.q)
    r = round(h.r)
    s = round(h.s)

    q_diff = abs(q - h.q)
    r_diff = abs(r - h.r)
    s_diff = abs(s - h.s)

    cond do
      (q_diff > r_diff && q_diff > s_diff) ->
        %Cube{q: -r - s, r: r, s: s}
      (r_diff > s_diff) ->
        %Cube{q: q, r: -q - s, s: s}
      true ->
        %Cube{q: q, r: r, s: -q - r}
    end
  end
end
