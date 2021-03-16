defmodule ExHtml5ever.MixProject do
  use Mix.Project

  def project do
    [
      app: :ex_html5ever,
      version: "0.1.0",
      elixir: "~> 1.11",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:plug, "~> 1.11"},
      {:rustler,
       git: "git://github.com/rusterlium/rustler",
       sparse: "rustler_mix",
       tag: "49a3c26c35e86f652ce64dc5e359d7cdfd179274"}
    ]
  end
end
