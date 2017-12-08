defmodule Day7.Prgm do
  alias Day7.Prgm
  
  defstruct name: nil, weight: 0, towers: [], total_weight: 0
  
  def pick_name(prgm) when is_nil(prgm), do: nil
  def pick_name(%Prgm{} = prgm) do
    prgm.name
  end

  def split_towers(towers) when length(towers) == 0, do: nil
  def split_towers([towers | _]) do
    String.split(towers, ", ", trim: true)
  end

  def find_root_prgrm(rows_with_towers) do
    Enum.find(rows_with_towers, fn(row) ->
      result = Enum.find(rows_with_towers, fn(r) -> 
        Enum.member?(r.towers, row.name)
      end)

      if is_nil(result) do
        true
      else
        false
      end
    end)
  end

  def find_rows_with_towers(rows) do
    Enum.filter(rows, fn(row) -> length(row.towers) != 0 end)
  end

  def add_towers_to_prgm(prgm, _rows) when is_nil(prgm), do: nil
  def add_towers_to_prgm(%Prgm{} = prgm, rows) do
    %{prgm | towers: Enum.map(prgm.towers, fn(name) -> 
      add_towers_to_prgm(Enum.find(rows, fn(row) -> row.name == name end), rows)
    end)}
  end

  defp subtower_weights(%Prgm{} = prgm) do
    Enum.reduce(prgm.towers, 0, fn(tower, acc) -> acc + tower.weight + subtower_weights(tower) end)
  end

  def add_total_weights(%Prgm{} = prgm) do
    prgm_with_weight = %{prgm | total_weight: prgm.weight + subtower_weights(prgm)}

    %{prgm_with_weight | towers: Enum.map(prgm_with_weight.towers, fn(tower) -> 
      add_total_weights(tower)
    end)}
  end

  # Goes up the trees until a balanced layer is found, and calculates the correct weight
  def find_unbalanced_prgm(%Prgm{} = prgm, prev_balance \\ 0) do
    groups = Enum.group_by(prgm.towers, fn x -> x.total_weight end)

    try do
      {balance, _balanced} = Enum.max_by(groups, fn({_k, v}) -> length(v) end)
      {unbalance, unbalanced} = Enum.min_by(groups, fn({_k, v}) -> length(v) end)
      
      if balance == unbalance do
        prev_balance - prgm.total_weight + prgm.weight
      else
        List.first(unbalanced) |> find_unbalanced_prgm(balance)
      end
    rescue
      Enum.EmptyError -> nil
    end
  end
end

defmodule Day7.Part1 do
  alias Day7.Prgm

  @moduledoc """
  Documentation for Part1.
  """

  @doc ~S"""
  Finds the name of the bottom program from the tree of recursive programs call stacks.

  ## Examples:
    iex> input = "pbga (66)\nxhth (57)\nebii (61)\nhavc (66)\nktlj (57)\nfwft (72) -> ktlj, cntj, xhth\nqoyq (66)\npadx (45) -> pbga, havc, qoyq\ntknk (41) -> ugml, padx, fwft\njptl (61)\nugml (68) -> gyxo, ebii, jptl\ngyxo (61)\ncntj (57)"
    iex> Part1.solve(input)
    "tknk"
  """
  def solve(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn(row) ->
      [head | tail] = String.split(row, " -> ", trim: true)

      towers = Prgm.split_towers(tail)
      [name | [weight | _]] = String.split(head, ~r{ \(|\)}, trim: true)

      if is_nil(towers) do
        %Prgm{name: name, weight: String.to_integer(weight)}
      else
        %Prgm{name: name, weight: String.to_integer(weight), towers: towers}
      end
    end)
    |> Prgm.find_rows_with_towers
    |> Prgm.find_root_prgrm
    |> Prgm.pick_name
  end
end


defmodule Day7.Part2 do
  alias Day7.Prgm

  @moduledoc """
  Documentation for Part2.
  """

  @doc ~S"""
  Finds the name of the bottom program from the tree of recursive programs call stacks.

  ## Examples:
    iex> input = "pbga (66)\nxhth (57)\nebii (61)\nhavc (66)\nktlj (57)\nfwft (72) -> ktlj, cntj, xhth\nqoyq (66)\npadx (45) -> pbga, havc, qoyq\ntknk (41) -> ugml, padx, fwft\njptl (61)\nugml (68) -> gyxo, ebii, jptl\ngyxo (61)\ncntj (57)"
    iex> Part2.solve(input)
    60
  """
  def solve(input) do
    rows = input
      |> String.split("\n", trim: true)
      |> Enum.map(fn(row) ->
        [head | tail] = String.split(row, " -> ", trim: true)

        towers = Prgm.split_towers(tail)
        [name | [weight | _]] = String.split(head, ~r{ \(|\)}, trim: true)

        if is_nil(towers) do
          %Prgm{name: name, weight: String.to_integer(weight)}
        else
          %Prgm{name: name, weight: String.to_integer(weight), towers: towers}
        end
      end)

    rows
    |> Prgm.find_rows_with_towers
    |> Prgm.find_root_prgrm
    |> Prgm.add_towers_to_prgm(rows)
    |> Prgm.add_total_weights
    |> Prgm.find_unbalanced_prgm
  end
end
