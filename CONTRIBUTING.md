hey! thanks for the interest.
there are a few things that help me out to maintain CI/CD properly
for myself and contributors.

## Conventional Commits

super dweeby,
but i can auto generate a changelog and release notes
if you follow the conventional commits spec:
[conventional commits](https://www.conventionalcommits.org/en/v1.0.0/)

## tools

we use [`git-cliff`](https://crates.io/crates/git-cliff) to generate a changelog,
thus the conventional commits.

we use GitHub Actions to run CI/CD flows,
but they're gated to my user account cuz i ain't made of money.
so i might take your PR and open a branch.
i'll make sure to credit you in that PR.
