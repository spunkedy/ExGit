defmodule ExGit.MixProject do
  use Mix.Project

  def project do
    [
      app: :ex_git,
      description: """
      a lightweight rust shim for gitrs
      """,
      package: package(),
      licenses: ["MIT"],
      version: "0.6.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      docs: [
        # The main page in the docs
        main: "ExGit"
      ]
    ]
  end

  defp package do
    [
      maintainers: ["Ed Bond"],
      licenses: ["MIT"],
      files: ~w(
        lib
        priv
        native/exgit/src
        native/exgit/Cargo.lock
        native/exgit/Cargo.toml
        .formatter.exs
        mix.exs
        README*
        LICENSE*),
      links: %{"GitHub" => "https://github.com/spunkedy/ExGit"}
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.27.0"},
      {:ex_doc, "~> 0.29.1", only: :dev, runtime: false}
      # {:dep_from_hexpm, "~> 0.3.0"},
      # {:dep_from_git, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"}
    ]
  end
end
