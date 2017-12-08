defmodule Day8.Command do
  defstruct target_register: nil, operation: nil, condition_register: nil, condition: nil
end


defmodule Day8.Part1 do
  @moduledoc """
  Documentation for Part1.
  """

  alias Day8.Command

  @doc ~S"""
  Each instruction consists of several parts: the register to modify,
  whether to increase or decrease that register's value, the amount by which
  to increase or decrease it, and a condition. If the condition fails, skip
  the instruction without modifying the register.
  
  The registers all start at 0. The instructions look like this:

  b inc 5 if a > 1
  a inc 1 if b < 5
  c dec -10 if a >= 1
  c inc -20 if c == 10

  ## Examples:
    iex> Part1.solve("b inc 5 if a > 1\na inc 1 if b < 5\nc dec -10 if a >= 1\nc inc -20 if c == 10")
    1
  """
  def solve(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn row -> process_row(row) end)
    |> Enum.reduce(%{}, fn(command, registers) -> execute(command, registers) end)
    |> Map.values
    |> Enum.max
  end

  defp process_row(row) do
    [command | condition] = String.split(row, " if ", trim: true)
    
    condition = List.first(condition)
    |> String.split(" ")
    |> List.to_tuple
    |> parse_condition
  
    String.split(command, " ")
    |> List.to_tuple
    |> parse_command(condition)
  end

  defp parse_command(command, condition) do
    case command do
      {register, "inc", value} ->
        %Command{condition |
          target_register: register,
          operation: &(&1 + String.to_integer(value))}
      {register, "dec", value} ->
        %Command{condition |
          target_register: register,
          operation: &(&1 - String.to_integer(value))}
    end
  end

  defp parse_condition(condition) do
    case condition do
      {register, ">", value} ->
        %Command{condition_register: register, condition: &(&1 > String.to_integer(value))}
      {register, "<", value} ->
        %Command{condition_register: register, condition: &(&1 < String.to_integer(value))}
      {register, ">=", value} ->
        %Command{condition_register: register, condition: &(&1 >= String.to_integer(value))}
      {register, "==", value} ->
        %Command{condition_register: register, condition: &(&1 == String.to_integer(value))}
      {register, "<=", value} ->
        %Command{condition_register: register, condition: &(&1 <= String.to_integer(value))}
      {register, "!=", value} ->
        %Command{condition_register: register, condition: &(&1 != String.to_integer(value))}
    end
  end

  defp execute(%Command{} = command, registers) do
    condition_target = Map.get(registers, command.condition_register, 0)

    if command.condition.(condition_target) do
      Map.update(registers, command.target_register, command.operation.(0), command.operation)
    else
      registers
    end
  end
end
