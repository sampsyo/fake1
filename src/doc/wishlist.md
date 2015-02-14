- File-change tracking that is more robust than mtime. Content hashes, or inotify/kqueue/fsevents (like Tup).
- More generally: incremental execution when depending on non-files (e.g., parts of files, or web service results), like Shake.
- Call directly into other languages without serialization and command-line flags. Work as glue code/incrementalism tool without needing to go through the Unix interface.
- Dynamic dependency computation: e.g., make BibTeX dependency finder for LaTeX builds work robustly.
- Multiple products from a single action, like Ninja.


## Niceness

- Force flag: `fake -f something` should always re-make it, temporarily disabling incrementalism.
- Tab-completion in the shell.
- Web frontend. Export some targets as pushable buttons. A quick and dirty way to provide automated actions for remote humans or scripts.


## Debuggability

- Convenient debugging: Show me a tree that describes why a target was re-made, or why it *wasn't* re-made, or a search log to describe why a target can't be made.
- GraphViz dependency output for debugging.
- Auditable tasks: record and make visible the stdout/stderr from every invocation if you need to check what went wrong in your pipeline (like Drake).


## Clusters and Scale

- Non-file dependencies can be stored in a database (replace CWMemo for running experiments). Maybe the database is a giant JSON object so you can depend on arbitrary pieces of it.
- Distributed make (i.e., on a cluster).
- Scale to millions of targets/tasks. Should be suitable for running a large parameter sweep across a cluster while preserving incrementality (memoization) at a fine grain.
