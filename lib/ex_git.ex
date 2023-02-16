defmodule ExGit do
  @moduledoc """
  The shim between elixir and rust that gets overloaded
  """
  use Rustler, otp_app: :ex_git, crate: "exgit"

  def clone(_to_clone, _destination), do: :erlang.nif_error(:nif_not_loaded)
  def init(_destination), do: :erlang.nif_error(:nif_not_loaded)
  def init_bare(_destination), do: :erlang.nif_error(:nif_not_loaded)
  def add_commit(_destination, _commit_message), do: :erlang.nif_error(:nif_not_loaded)
  def latest_message(_destination), do: :erlang.nif_error(:nif_not_loaded)
  def push_remote(_destination, _branch), do: :erlang.nif_error(:nif_not_loaded)
  def fast_forward(_destination, _branch), do: :erlang.nif_error(:nif_not_loaded)
end
