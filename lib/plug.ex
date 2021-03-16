defmodule ExHtml5Ever.Plug do
  require Logger

  def init(opts), do: opts

  def call(conn, _opts) do
    Plug.Conn.register_before_send(conn, &lint/1)
  end

  defmodule Error do
    defexception message: "HTML linter returned errors"
  end

  defp lint(conn) do
    body_string = to_string(conn.resp_body)
    result = ExHtml5ever.Native.lint(body_string)

    case result do
      :ok ->
        conn

      {:error, errors} ->
        case Mix.env() do
          :dev ->
            Logger.warn(format_errors(conn, errors, body_string))
            conn

          :test ->
            raise Error, format_errors(conn, errors, body_string)
        end
    end
  end

  defp format_errors(conn, errors, body_string) do
    lines = String.split(body_string, "\n")

    msg =
      errors
      |> Enum.map(fn {line, msg} ->
        "  Line #{line}: #{msg}\n" <>
          case Enum.at(lines, line - 1) do
            nil ->
              ""

            line ->
              "    " <> String.trim(String.slice(line, 0, 400)) <> "\n"
          end
      end)
      |> Enum.join("")

    "Errors/warnings for HTML returned for #{conn.request_path}:\n" <> msg
  end
end
