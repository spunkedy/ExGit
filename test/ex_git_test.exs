defmodule ExGitTest do
  use ExUnit.Case

  setup_all do
    {:ok, %{}}
  end

  defp check_delete(path) do
    if File.exists?(path) do
      File.rm_rf!(path)
    end
  end

  @clone_prefix "test/git_testing/"
  test "clone tests" do
    check_delete(@clone_prefix <> "to_clone")

    assert {:ok, "clone success"} ==
             ExGit.clone("https://github.com/alexcrichton/git2-rs", @clone_prefix <> "to_clone")

    assert {:error, :exists} ==
             ExGit.clone("https://github.com/alexcrichton/git2-rs", @clone_prefix <> "to_clone")
  end

  test "commit test" do
    check_delete(@clone_prefix <> "to_commit_a")

    {:ok, _} = ExGit.init(@clone_prefix <> "to_commit_a")
    :ok = File.write(@clone_prefix <> "to_commit_a/test.txt", "123")
    ExGit.add_commit(@clone_prefix <> "to_commit_a", "testing 123")

    assert {:ok, "testing 123"} ==
             ExGit.latest_message(@clone_prefix <> "to_commit_a")
  end

  test "full example" do
    # cleanup if need
    check_delete(@clone_prefix <> "init_bare")
    check_delete(@clone_prefix <> "push_a")
    check_delete(@clone_prefix <> "push_b")

    # init bare repo
    {:ok, _} = ExGit.init_bare(@clone_prefix <> "init_bare")

    # clone to another directory
    assert {:ok, "clone success"} ==
             ExGit.clone(@clone_prefix <> "init_bare", @clone_prefix <> "push_a")

    # clone another copy
    assert {:ok, "clone success"} ==
             ExGit.clone(@clone_prefix <> "init_bare", @clone_prefix <> "push_b")

    # commit to a
    :ok = File.write(@clone_prefix <> "push_a/test.txt", "123")
    ExGit.add_commit(@clone_prefix <> "push_a", "push 123")

    {:ok, remotes} = ExGit.list_references(@clone_prefix <> "push_a")

    to_push =
      remotes
      |> Enum.at(0)
      |> String.replace("refs/heads/", "")

    # push back changes so b can see it
    assert {:ok, "Pushed Successfully"} == ExGit.push_remote(@clone_prefix <> "push_a", to_push)

    # pull to check the file from b
    assert {:ok, "Updated"} ==
             ExGit.fast_forward(@clone_prefix <> "push_b", to_push)

    assert {:ok, "Up to date already"} ==
             ExGit.fast_forward(@clone_prefix <> "push_b", to_push)

    # cause collision

    # commit to a
    :ok = File.write(@clone_prefix <> "push_a/test2.txt", "1234")
    ExGit.add_commit(@clone_prefix <> "push_a", "push 123")
    assert {:ok, "Pushed Successfully"} == ExGit.push_remote(@clone_prefix <> "push_a", to_push)

    # commit to b and push
    :ok = File.write(@clone_prefix <> "push_b/test2.txt", "5678")
    ExGit.add_commit(@clone_prefix <> "push_b", "push 123")
    assert {:error, :notfastforward} == ExGit.push_remote(@clone_prefix <> "push_b", to_push)
  end

  # test "remote repo" do
  #   path = "/tmp/to_test8"
  #   {:ok, "clone success"} = ExGit.clone("git@test.git", path)

  #   :ok = File.write(path <> "/test2.txt", "1234")
  #   ExGit.add_commit(path, "push 123")
  #   assert {:ok, "Pushed Successfully"} == ExGit.push_remote(path, "main")
  # end
end
