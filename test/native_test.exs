defmodule ExHtml5ever.NativeTest do
  use ExUnit.Case
  alias ExHtml5ever.Native, as: Html5ever

  describe "lint/1" do
    test "parses valid HTML without errors" do
      assert :ok ==
               Html5ever.lint("""
               <!DOCTYPE html>
               <head><title>title</title></head>
               <body>
                 <p>Hello!
                 <p>There!
               """)
    end

    test "missing closing </ul> tag causes error" do
      assert {:error, [{7, "Unexpected open tag at end of body"}]} ==
               Html5ever.lint("""
               <!DOCTYPE html>
               <head><title>title</title></head>
               <body>
                 <ul>
                   <li>Hello!
                   <li>There!
               </body>
               """)
    end
  end
end
