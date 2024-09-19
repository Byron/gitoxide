---
marp: true
theme: gaia
_class: lead
paginate: false
backgroundColor: white
---

<style>
  :root {
    --color-foreground: #333;
    --color-highlight: #dea584;
  }
  a {
    --color-highlight: #aaa;
  }
</style>

# **Gitoxide**

**What it is, and isn't**

https://github.com/Byron/gitoxide

---

<!-- _class: lead -->

# **Gitoxide**

**(and wants to be)**

https://github.com/Byron/gitoxide

---

![bg 80% sepia right](https://www.svgrepo.com/show/484850/traffic-sign-stop.svg)

# Terms

* `git2` - A Rust crate :heart: binding to `libgit2` 
* `libgit2` 
    * needs no introduction 🙇‍♂️
* Git - the mothership :ship:
    * `libgit-rs` - a Rust crate binding to `libgit.a`

---

<!-- _class: "lead" -->

# **Gitoxide**
### **is**

**the high-performance Rust crate that acts like Git would**

<!-- benefits clients, no need to re-implement stuff users expect from you -->
<!-- respect configuration, but allow overrides -->

---

<!-- _class: "lead" -->

# **Gitoxide**
### **is**

**the binaries `gix` and `ein`**

<!-- `gix` dev tool, `ein` client + tooling -->

---

<!-- _class: "lead" -->

# **`Gi`to`x`ide**
### **isn't**

**a Git replica**

<!-- `gix` will never be `git`, never stable, won't mimmick it -->
<!-- But `gitoxide` should be versatile enough to implement the majority of Git -->

---

<!-- _class: "lead" -->

# **Gitoxide**
### **isn't**

**always quite as easy-to-use as `git2`**

---

<!-- _class: "lead" -->

# **Gitoxide**
### **isn't**

**usable from any other language but Rust**

<!-- in great contrast to `git2`  -->

---

<!-- _class: "lead" -->

# **Gitoxide**
### **isn't**

**stable (yet)**

---

# Aspirations of **Gitoxide** :star:

* be the go-to Git library for anyone, usable from any language
* be the best choice for large monorepos
* power an entire Git forge
  * most-efficient on server 
* be the first with new (experimental) Git features 

---

# Who uses **Gitoxide** :zany_face:

<!-- disclaimer: not entirely just yet, still `git2` in there -->

* Cargo
* rustsec & cargo-deny
* starship
* OneFetch
* `jj`
* Sapling
* GitButler 

---

# Why would you use **Gitoxide** :question:

- new project with Git integration, `gitoxide` features suffice
    * or you want to contribute what's missing
    * and you can figure out how to use it (`gix::Repository`)
* need Git-level repository compatbility
* need to read from untrusted repositories
* need next-level performance and fearless concurrency
* you deal with massive monorepos

---

# **Gitoxide** includes

- read & write thread-safe & lock-free object database
    * decode and encode all object types
      * *also step-wise*
* read + write ref-database
* lossless git-config reading and writing
* read and write index

---

# **Gitoxide** includes

- .gitignore (with *precious* files) & .gitattributes
* pathspecs: `/**hello.*`
* revspecs: `@~1` 
* refspecs: `refs/heads/*:refs/remotes/origin/*`

---

# **Gitoxide** includes

- read commit-graphs and use them for traversals
   * describe commit
   * find merge-base
   * perform fetch negotiations
* rev-parse

---

# **Gitoxide** includes

- diff
    - tree-tree
    - index-worktree
    - (*missing*: tree-index)
* find untracked files/classify worktree

---

# **Gitoxide** includes

- (shallow) fetch 
   - ssh
   - http/https
   - git-native
* first checkout after fetch
   - with built-in filters
   - with custom filters (all protocols)

---

![bg](https://github.com/user-attachments/assets/ad337f05-dc5a-427d-bc25-923911280712)

---

![bg](https://github.com/user-attachments/assets/6eb59416-4f93-4ffa-b22c-8299a3cd2680)

---

# **Gitoxide** lacks

- *full* status (tree-index diff missing)
* commit (index-add, index-to-tree)
* merge (blob-diff works🎉, but WIP)
* push (but can build packs by object-copy)
* rebase
* cherry-pick
* reset/checkout
* blame (but [WIP](https://github.com/Byron/gitoxide/pull/1453))

---

# Principles during Development

## **Correctness**

* baseline tests with Git where possible
* fence-post tests (at least)
* leave notes and TODOs if corners were cut
* fuzzing of parsers

---
# Principles during Development

## **Performance**

* be mindful about allocations
* stepwise computation when feasible
* optional multi-threading where possible 

--- 

# Principles during Development

## **Everything Else**

- progress-reporting and are interrupt-handling
* easy-to-use API without hiding any knob
* `async`-support for networked IO
* [exhaustive docs](https://docs.rs/gix/latest/gix/)
* **Security**
  - by Eliah Kagan (and sponsored by the **Radicle Foundation**)

---

## Bonus Round: API still needs improvements

Real world code written by a real API user, courtesy of GitButler.

---

## Bonus Round: API still needs improvements

### **git2**
```Rust
#[tauri::command(async)]
pub fn git_clone_repository(repository_url: &str, target_dir: &Path) -> Result<(), Error> {
  git2::Repository::clone(repository_url, target_dir).context("Cloning failed")?;
  Ok(())
}
```

---

## Bonus Round: API still needs improvements

### **Gitoxide**

```Rust
#[tauri::command(async)]
pub fn git_clone_repository(repository_url: &str, target_dir: &Path) -> Result<(), Error> {
    let url =
        gix::url::parse(repository_url.into()).context("Failed to parse repository URL")?;
    let should_interrupt = AtomicBool::new(false);
    let mut prepared_clone =
        gix::prepare_clone(url, target_dir).context("Failed to prepare clone")?;
    let (mut prepared_checkout, _) = prepared_clone
        .fetch_then_checkout(Discard, &should_interrupt)
        .context("Failed to fetch")?;
    let should_interrupt = AtomicBool::new(false);
    prepared_checkout
        .main_worktree(Discard, &should_interrupt)
        .context("Failed to checkout main worktree")?;
    Ok(())
}
```

---

## Bonus Round: API still needs improvements

### **Gitoxide** - less noisy

```Rust
#[tauri::command(async)]
pub fn git_clone_repository(repository_url: &str, target_dir: &Path) -> Result<(), Error> {
    let url = gix::url::parse(repository_url.into()).map_err(anyhow::Error::from)?;
    let should_interrupt = AtomicBool::new(false);

    let (mut checkout, _outcome) = gix::prepare_clone(url, target_dir)
        .map_err(anyhow::Error::from)?
        .fetch_then_checkout(Discard, &should_interrupt)
        .map_err(anyhow::Error::from)?;
    checkout
        .main_worktree(Discard, &should_interrupt)
        .map_err(anyhow::Error::from)?;
    Ok(())
}
```
---

## Bonus Round: API still needs improvements

### **Gitoxide** - *even* less noisy

```Rust
#[tauri::command(async)]
pub fn git_clone_repository(repository_url: &str, target_dir: &Path) -> Result<(), UnmarkedError> {
    let should_interrupt = AtomicBool::new(false);

    gix::prepare_clone(repository_url, target_dir)?
        .fetch_then_checkout(gix::progress::Discard, &should_interrupt)
        .map(|(checkout, _outcome)| checkout)?
        .main_worktree(gix::progress::Discard, &should_interrupt)?;
    Ok(())
}
````

---

## Bonus Round: API still needs improvements

### Comparison

```Rust
#[tauri::command(async)]
pub fn git_clone_repository(repository_url: &str, target_dir: &Path) -> Result<(), Error> {
  git2::Repository::clone(repository_url, target_dir).context("Cloning failed")?;
  Ok(())
}
```

```Rust
#[tauri::command(async)]
pub fn git_clone_repository(repository_url: &str, target_dir: &Path) -> Result<(), UnmarkedError> {
    let should_interrupt = AtomicBool::new(false);

    gix::prepare_clone(repository_url, target_dir)?
        .fetch_then_checkout(gix::progress::Discard, &should_interrupt)
        .map(|(checkout, _outcome)| checkout)?
        .main_worktree(gix::progress::Discard, &should_interrupt)?;
    Ok(())
}
````