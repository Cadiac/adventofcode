defmodule Hextille.Mixfile do
  use Mix.Project

  def project do
    [
      app: :hextille,
      version: "0.1.0",
      elixir: "~> 1.5",
      start_permanent: Mix.env == :prod,
      description: description(),
      package: package(),
      deps: deps(),
      source_url: "https://github.com/Cadiac/hextille"
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger],
      mod: {Hextille.Application, []}
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:ex_doc, "~> 0.16", only: :dev, runtime: false}
    ]
  end

  defp description() do
    "Module for common math operations in hexagonal grids."
  end

  defp package() do
    [
      # These are the default files included in the package
      files: ["lib", "mix.exs", "README*", "LICENSE"],
      maintainers: ["Jaakko Husso"],
      licenses: ["MIT"],
      links: %{"GitHub" => "https://github.com/Cadiac/hextille"}
    ]
  end
end
