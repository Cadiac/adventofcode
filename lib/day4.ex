defmodule Day4.Part1 do
  @moduledoc """
  Documentation for Part1.
  """

  @doc ~S"""
  Checks if passphrase is legit. Passpharse should not
  contain same word more than once.

  ## Examples:
    iex> Part1.solve("aa bb cc dd ee\naa bb cc dd aa")
    1
    iex> Part1.solve("aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd ee")
    2
    iex> Part1.solve("aa bb cc dd aaa")
    1
  """
  def solve(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn(x) -> is_valid(x) end)
    |> Enum.reduce(0, fn(x, acc) -> if x, do: acc + 1, else: acc end) 
  end

  @doc """
  Checks if passphrase is legit. Passpharse should not
  contain same word more than once.

  ## Examples:
    iex> Part1.is_valid("aa bb cc dd ee")
    true
    iex> Part1.is_valid("aa bb cc dd aa")
    false
    iex> Part1.is_valid("aa bb cc dd aaa")
    true
  """
  def is_valid(passphrase) do
    splitted = String.split(passphrase, " ")
    unique = Enum.uniq(splitted)
    length(splitted) == length(unique)
  end
end

defmodule Day4.Part2 do
  @moduledoc """
  Documentation for Part2.
  """

  @doc ~S"""
  Checks if passphrase is legit. Valid passphrase must contain
  no two words that are anagrams of each other.

  ## Examples:
    iex> Part2.solve("abcde fghij\nabcde xyz ecdab")
    1
    iex> Part2.solve("iiii oiii ooii oooi oooo\nabcde fghij\nabcde xyz ecdab")
    2
    iex> Part2.solve("oiii ioii iioi iiio")
    0
  """
  def solve(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn(x) -> is_valid(x) end)
    |> Enum.reduce(0, fn(x, acc) -> if x, do: acc + 1, else: acc end) 
  end

  @doc """
  Valid passphrase must contain no two words that are anagrams of each other.

  ## Examples:
    iex> Part2.is_valid("abcde fghij")
    true
    iex> Part2.is_valid("abcde xyz ecdab")
    false
    iex> Part2.is_valid("iiii oiii ooii oooi oooo")
    true
    iex> Part2.is_valid("oiii ioii iioi iiio")
    false
  """
  def is_valid(passphrase) do
    signatures = passphrase
    |> String.split(" ")
    |> Enum.map(fn(w) -> word_signature(w) end)
    
    uniq_signatures = Enum.uniq(signatures)

    length(signatures) == length(uniq_signatures)
  end

  @doc """
  Finds a signature for word from its graphemes.
  
  ## Examples:
    iex> Part2.word_signature("foobar")
    [{"a", 1}, {"b", 1}, {"f", 1}, {"o", 2}, {"r", 1}]
    iex> Part2.word_signature("oofarb")
    [{"a", 1}, {"b", 1}, {"f", 1}, {"o", 2}, {"r", 1}]
    iex> Part2.word_signature("oofarr")
    [{"a", 1}, {"f", 1}, {"o", 2}, {"r", 2}]
  """
  def word_signature(word) do
    graphemes = String.graphemes(word)

    graphemes
    |> Enum.uniq
    |> Enum.map(fn(c) ->
      {c, length(Enum.filter(graphemes, fn(g) -> c == g end))} 
    end)
    |> Enum.sort
  end
end
