# GUIDE

## Commit newly generated bindings

You last commit must contain `[generate bindings]`. You can make an empty
commit with this message:

```sh
git commit -m "[generate bindings]" --allow-empty
```

Then after successfully running workflows, GitHub Actions will push a commit
with the updated bindings onto your PR branch.

# GUIDE

## Commit newly generated bindings

Your last commit must contain `[generate bindings]`. You can make an empty
commit with this message:

```sh
git commit -m "[generate bindings]" --allow-empty
```

Then after successfully running workflows, GitHub Actions will push a commit
with the updated bindings onto your PR branch.

## Precomputed bindings

### How to update the precomputed bindings?

The precomputed bindings are continuously updated on `generated_bindings` branch,
but it doesn't propagate into `master` branch automatically; when we need to
reflect the recent changes, create a pull request from `generated_bindings` branch.
This link is the shortcut for this:

<https://github.com/extendr/libR-sys/compare/master...generated_bindings?expand=1>

### How to address conflicts in `generated_bindings`?

You can just delete the branch. Since the GitHub Actions CI runs periodically,
it will be created again from the latest `master` in a few days (or you can
retrigger the build manually).
