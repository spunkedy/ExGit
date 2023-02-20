defmodule ExGit do
  @moduledoc """
  The shim between elixir and rust that gets overloaded


  You can see more examples in the tests but here is a quick example:

  ```
    clone_prefix = "/tmp/example_git"
    # init bare repo
    {:ok, _} = ExGit.init_bare(clone_prefix <> "init_bare")

    # clone to another directory
    assert {:ok, "clone success"} ==
             ExGit.clone(clone_prefix <> "init_bare", clone_prefix <> "push_a")

    # clone another copy
    assert {:ok, "clone success"} ==
             ExGit.clone(clone_prefix <> "init_bare", clone_prefix <> "push_b")

    # commit to a
    :ok = File.write(clone_prefix <> "push_a/test.txt", "123")
    ExGit.add_commit(clone_prefix <> "push_a", "push 123")

    # push back changes so b can see it
    assert {:ok, "Pushed Successfully"} == ExGit.push_remote(clone_prefix <> "push_a", "master")

    # pull to check the file from b
    assert {:ok, "Updated"} ==
             ExGit.fast_forward(clone_prefix <> "push_b", "master")

    assert {:ok, "Up to date already"} ==
             ExGit.fast_forward(clone_prefix <> "push_b", "master")

  ```
  """
  use Rustler, otp_app: :ex_git, crate: "exgit"

  def clone(_to_clone, _destination), do: :erlang.nif_error(:nif_not_loaded)
  def init(_destination), do: :erlang.nif_error(:nif_not_loaded)
  def init_bare(_destination), do: :erlang.nif_error(:nif_not_loaded)
  def add_commit(_destination, _commit_message), do: :erlang.nif_error(:nif_not_loaded)
  def latest_message(_destination), do: :erlang.nif_error(:nif_not_loaded)
  def push_remote(_destination, _branch), do: :erlang.nif_error(:nif_not_loaded)
  def fast_forward(_destination, _branch), do: :erlang.nif_error(:nif_not_loaded)
  def list_references_rust(_path), do: :erlang.nif_error(:nif_not_loaded)

  def list_references(path) do
    {:ok, references} = list_references_rust(path)

    {:ok,
     references
     |> String.trim(",")
     |> String.split(",")}
  end
end
