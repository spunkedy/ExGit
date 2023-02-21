#![allow(unused_imports)]
use git2::build::RepoBuilder;
use git2::string_array::StringArray;
use git2::{Cred, Credentials, Direction, ErrorCode, Oid, RemoteCallbacks, Repository};
use rustler::{Atom, Error, Term};
use std::env;
use std::path::Path;

mod atoms {
    rustler::atoms! {
        ok,
        error,
        unknown,
        fast_forward_only,
        genericerror,
        notfound,
        exists,
        ambiguous,
        bufsize,
        user,
        barerepo,
        unbornbranch,
        unmerged,
        notfastforward,
        invalidspec,
        conflict,
        locked,
        modified,
        auth,
        certificate,
        applied,
        peel,
        eof,
        invalid,
        uncommitted,
        directory,
        mergeconflict,
        hashsummismatch,
        indexdirty,
        applyfail,
        owner
    }
}

macro_rules! handle_git_error {
    ($e:expr) => {
        match $e {
            Ok(inner) => inner,
            Err(ref error) => {
                eprintln!("Error: {error}");
                match error.code() {
                    ErrorCode::GenericError  => return Err(Error::Term(Box::new(atoms::genericerror()))),
                    ErrorCode::NotFound  => return Err(Error::Term(Box::new(atoms::notfound()))),
                    ErrorCode::Exists  => return Err(Error::Term(Box::new(atoms::exists()))),
                    ErrorCode::Ambiguous  => return Err(Error::Term(Box::new(atoms::ambiguous()))),
                    ErrorCode::BufSize  => return Err(Error::Term(Box::new(atoms::bufsize()))),
                    ErrorCode::User  => return Err(Error::Term(Box::new(atoms::user()))),
                    ErrorCode::BareRepo  => return Err(Error::Term(Box::new(atoms::barerepo()))),
                    ErrorCode::UnbornBranch  => return Err(Error::Term(Box::new(atoms::unbornbranch()))),
                    ErrorCode::Unmerged  => return Err(Error::Term(Box::new(atoms::unmerged()))),
                    ErrorCode::NotFastForward  => return Err(Error::Term(Box::new(atoms::notfastforward()))),
                    ErrorCode::InvalidSpec  => return Err(Error::Term(Box::new(atoms::invalidspec()))),
                    ErrorCode::Conflict  => return Err(Error::Term(Box::new(atoms::conflict()))),
                    ErrorCode::Locked  => return Err(Error::Term(Box::new(atoms::locked()))),
                    ErrorCode::Modified  => return Err(Error::Term(Box::new(atoms::modified()))),
                    ErrorCode::Auth  => return Err(Error::Term(Box::new(atoms::auth()))),
                    ErrorCode::Certificate  => return Err(Error::Term(Box::new(atoms::certificate()))),
                    ErrorCode::Applied  => return Err(Error::Term(Box::new(atoms::applied()))),
                    ErrorCode::Peel  => return Err(Error::Term(Box::new(atoms::peel()))),
                    ErrorCode::Eof  => return Err(Error::Term(Box::new(atoms::eof()))),
                    ErrorCode::Invalid  => return Err(Error::Term(Box::new(atoms::invalid()))),
                    ErrorCode::Uncommitted  => return Err(Error::Term(Box::new(atoms::uncommitted()))),
                    ErrorCode::Directory  => return Err(Error::Term(Box::new(atoms::directory()))),
                    ErrorCode::MergeConflict  => return Err(Error::Term(Box::new(atoms::mergeconflict()))),
                    ErrorCode::HashsumMismatch  => return Err(Error::Term(Box::new(atoms::hashsummismatch()))),
                    ErrorCode::IndexDirty  => return Err(Error::Term(Box::new(atoms::indexdirty()))),
                    ErrorCode::ApplyFail  => return Err(Error::Term(Box::new(atoms::applyfail()))),
                    ErrorCode::Owner  => return Err(Error::Term(Box::new(atoms::owner())))
                }
            }
        }
    };
}

fn get_credentials<'cb>() -> RemoteCallbacks<'cb> {
    // Prepare callbacks.
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
        )
    });
    callbacks
}

fn get_builder<'cb>() -> RepoBuilder<'cb> {
    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    let callbacks = get_credentials();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);
    builder
}

#[rustler::nif]
fn clone(to_clone: &str, destination: &str) -> Result<(Atom, String), Error> {
    let result = get_builder().clone(to_clone, Path::new(destination));
    handle_git_error!(result);
    Ok((atoms::ok(), String::from("clone success")))
}

#[rustler::nif]
fn init(destination: &str) -> Result<(Atom, String), Error> {
    let result = Repository::init(destination);
    let repo = handle_git_error!(result);
    create_initial_commit(&repo);
    Ok((atoms::ok(), String::from("init success")))
}

#[rustler::nif]
fn init_bare(destination: &str) -> Result<(Atom, String), Error> {
    let result = Repository::init_bare(destination);
    let repo = handle_git_error!(result);
    create_initial_commit(&repo);
    Ok((atoms::ok(), String::from("init success")))
}

fn add_all(repo: &git2::Repository) {
    let mut index = repo.index().unwrap();
    index
        .add_all(&["."], git2::IndexAddOption::DEFAULT, None)
        .unwrap();

    index.write().unwrap();
}

fn create_initial_commit(repo: &git2::Repository) {
    let signature = repo.signature().unwrap();
    let oid = repo.index().unwrap().write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[],
    )
    .unwrap();
}

fn commit(repo: &git2::Repository, commit_message: &str) -> Result<Oid, git2::Error> {
    let mut index = repo.index().unwrap();
    let oid = index.write_tree().unwrap();
    let signature = repo.signature().unwrap();
    let parent_commit = repo.head().unwrap().peel_to_commit().unwrap();
    let tree = repo.find_tree(oid).unwrap();
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        commit_message,
        &tree,
        &[&parent_commit],
    )
}

#[rustler::nif]
fn add_commit(destination: &str, commit_message: &str) -> Result<(Atom, String), Error> {
    let repo = Repository::open(destination).unwrap();

    add_all(&repo);
    let commit_response = commit(&repo, commit_message);
    handle_git_error!(commit_response);
    Ok((atoms::ok(), String::from("Commit success")))
}

#[rustler::nif]
fn latest_message(destination: &str) -> Result<(Atom, String), Error> {
    let repo = Repository::open(destination).unwrap();
    let parent_commit = repo.head().unwrap().peel_to_commit().unwrap();
    let message = parent_commit.message().unwrap();
    Ok((atoms::ok(), String::from(message)))
}

#[rustler::nif]
fn push_remote(destination: &str, branch: &str) -> Result<(Atom, String), Error> {
    let repo = Repository::open(destination).unwrap();
    let mut remote = repo.find_remote("origin").unwrap();

    let mut po = git2::PushOptions::new();
    let callbacks = get_credentials();
    po.remote_callbacks(callbacks);

    // remote.connect(Direction::Push);
    let result = remote.push(
        &[format!("refs/heads/{}:refs/heads/{}", branch, branch)],
        Some(&mut po),
    );
    handle_git_error!(result);
    Ok((atoms::ok(), String::from("Pushed Successfully")))
}

#[rustler::nif]
fn fast_forward(path: &str, branch: &str) -> Result<(Atom, String), Error> {
    let repo = Repository::open(path).unwrap();

    let mut remote = repo.find_remote("origin").unwrap();
    remote
        .connect_auth(Direction::Fetch, Some(get_credentials()), None)
        .unwrap();

    let mut fo = git2::FetchOptions::new();
    let callbacks = get_credentials();
    fo.remote_callbacks(callbacks);
    remote.fetch(&[branch], Some(&mut fo), None).unwrap();

    let fetch_head = repo.find_reference("FETCH_HEAD").unwrap();
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head).unwrap();
    let analysis = repo.merge_analysis(&[&fetch_commit]).unwrap();

    if analysis.0.is_up_to_date() {
        Ok((atoms::ok(), String::from("Up to date already")))
    } else if analysis.0.is_fast_forward() {
        let refname = format!("refs/heads/{}", branch);
        let mut reference = repo.find_reference(&refname).unwrap();
        reference
            .set_target(fetch_commit.id(), "Fast-Forward")
            .unwrap();
        repo.set_head(&refname).unwrap();
        let _force = repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()));
        Ok((atoms::ok(), String::from("Updated")))
    } else {
        Err(Error::Term(Box::new(atoms::fast_forward_only())))
    }
}

#[rustler::nif]
fn list_references_rust(path: &str) -> Result<(Atom, String), Error> {
    let repo = Repository::open(path).unwrap();
    let response = repo.references();
    let references = handle_git_error!(response);
    let to_pass_back = references.fold(String::new(), |a, b| a + b.unwrap().name().unwrap() + ",");
    Ok((atoms::ok(), to_pass_back))
}

#[rustler::nif]
fn checkout_branch(path: &str, branch_name: &str) -> Result<(Atom, String), Error> {
    let repo = Repository::open(path).unwrap();

    let mut remote = repo.find_remote("origin").unwrap();
    remote
        .connect_auth(Direction::Fetch, Some(get_credentials()), None)
        .unwrap();
    let mut fo = git2::FetchOptions::new();
    let callbacks = get_credentials();
    fo.remote_callbacks(callbacks);
    remote.fetch(&[branch_name], Some(&mut fo), None).unwrap();

    let head = repo.head().unwrap();
    let oid = head.target().unwrap();
    let commit = repo.find_commit(oid).unwrap();

    let _branch = repo.branch(branch_name, &commit, false);

    let obj = repo
        .revparse_single(&("refs/heads/".to_owned() + branch_name))
        .unwrap();

    let _response = repo.checkout_tree(&obj, None);

    let _response = repo.set_head(&("refs/heads/".to_owned() + branch_name));
    Ok((atoms::ok(), String::from("Checked out")))
}

rustler::init!(
    "Elixir.ExGit",
    [
        init,
        init_bare,
        latest_message,
        clone,
        add_commit,
        push_remote,
        fast_forward,
        list_references_rust,
        checkout_branch
    ]
);
