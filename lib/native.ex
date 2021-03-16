defmodule ExHtml5ever.Native do
  use Rustler, otp_app: :ex_html5ever, crate: "ex_html5ever"

  def add(_a, _b), do: error()

  def lint(_html_string), do: error()

  defp error(), do: :erlang.nif_error(:nif_not_loaded)
end
