+++
title = "Folder Trust"
weight = 7
[extra]
group = "Reference"
+++

# Folder Trust

A project `.maki` directory can run code on your machine before you type
anything. Maki loads none of it until you trust the folder.

| Gated file | What it can do |
|------------|----------------|
| `.maki/init.lua` | Runs Lua inside Maki at startup |
| `.maki/.env` | Sets environment variables, including `BASE_URL` and API keys |
| `.maki/mcp.toml` | Starts MCP server processes |
| `.maki/permissions.toml` | Adds allow rules and defaults |

The first interactive start in an untrusted project names the gated files it
found and asks. The default answer is no. A project that ships none of these
files is never asked about.

One answer covers one project root: the active Git checkout, or the working
directory outside Git. Linked worktrees answer for themselves. Starting Maki in
your home directory loads no project configuration, because `~/.maki` there is
your global configuration.

## What Trust Does Not Cover

Trust gates code. Text that a project puts into the prompt loads at any trust
level:

- `AGENTS.md` and the other instruction files
- Commands under `.maki/commands` and `.claude/commands`
- Skills under `.maki/skills`, `.claude/skills`, `.opencode/skills` and
  `.agents/skills`

A repository can still try to steer the agent through what the model reads, so
trust is not a sandbox. What limits the agent on each tool call is
[permissions](/docs/permissions/), at every trust level.

One gated thing applies without trust: the `deny` scopes in
`.maki/permissions.toml`. Its `allow` scopes and any `default` it sets are
dropped, so a repository can narrow what the agent may do inside it and never
widen it.

## In an Untrusted Folder

Maki writes nothing into a folder you declined. The project answers in a
[permission prompt](/docs/permissions/#permission-prompts) still work and last
until the session ends, labelled `Project (this session)`. For an answer that
outlives the session, use `A` or `D` to save it in your own
`~/.config/maki/permissions.toml`, or trust the folder.

## Managing Trust

```bash
maki trust add [PATH]        # asks before recording
maki trust add [PATH] --yes  # records a yes
maki trust remove [PATH]     # clears a yes or a no
maki trust list              # shows both kinds of decision
```

`PATH` defaults to the current directory. None of these commands start the Lua
host, so they are safe to run in a folder you have not read yet. Decisions are
stored outside the project and follow the checkout path.

## Containers and CI

Headless runs, the SDK, ACP, and utility subcommands never ask. An untrusted
folder is skipped, the skipped path is reported on standard error, and the run
continues on global configuration.

Pass `--trust` where the container is already the boundary you rely on:

```bash
maki --trust -p "run the test suite"
```

The flag loads the project configuration for that run and records no decision,
so a state directory shared by many containers collects no grants. There is no
environment variable that does the same, since it would reach every child
process.

## What a Yes Covers

Your yes covers the kinds of gated file the folder had that day, since that is
what the question named. A project that later adds a kind you were never asked
about asks again.

Maki records the file names rather than their contents, so Lua that changes in a
later pull runs under the answer you already gave. Run `maki trust remove` when
that stops being what you want.
