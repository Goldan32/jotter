# Jotter

Jotter is a command line task tracker to help the user keep track of personal deadlines.

## Getting started

To get started, place or link the executable to a directory in `PATH`, then you can:

Create a task:

```bash
jotter add "Example task tittle" -d "Description of the task" -t "Tomorrow"
```

The created task will have a status of `Todo` besides all the supplied arguments.
Next, list the tasks:

```bash
jotter ls todo
```

This lists all tasks with the todo status. Then you can also edit a task description:

```bash
jotter open 1
```

Tasks get an id when created, use this id to refer to them.
You can also display a task:

```bash
jotter show 1
```

## Configuration

### Config file

These configs can currently be put in `~/.config/jotter/config.toml`:

- `root_dir`
  - default: `${HOME}/.local/share/bjl`
  - Where the program will store its persistent files
- `work_dir`
  - default: `${HOME}/.cache/bjl`
  - Where temporary files are created during runtime
- `task_db`
  - default: `${HOME}/.local/share/bjl/jotter.db3`
  - Path to a `.db3` (sqlite3) binary database file
  - If it doesn't exist, the program creates it
- `editor`
  - default: `nvim`
  - Exact binary that will be called when opening a task to edit

Take a look at `./config-default.toml` on how to structure your own config file. If a config is not defined in your config file, the value from the default config will be used.

### Environment variables

The configs above are available via environment variables too. Environment variables have a higher priority, so these will overwrite your config file.

To use them, create the variable with a name like so: `root_dir` -> `BJL_ROOT_DIR` (prepend `BJL_` and capitalize)

## Detailed documentation

Full documentation coming when time permits and most features are complete.

## How it works

The code is structured into 3 distinct units:

### Frontend

This is where the user interaction happens.
Currently the only possible way to interact with the program is the CLI,
but it is possible to create other means of communication.
All one would have to do is implement the `FrontEndInput` and `FrontEndOutput` traits.

### Database

A database is used to store the task data.
Currently only `sqlite3` is used, with a local file.
One can implement the `DatabaseOps` trait to add another database.

### Middleware

This is a connection layer between the previous two.
The selection of which frontend and database to use also happens here.

## Extend functionalites

It's possible to add new functions in a structured way.

- Define a new `FrontEndCapabilites` function.
- Implement this function for the `cli::Cli` struct.
- Make sure that this option is covered in the `FrontEndInput` trait.
- Add the new command in the `cli` module.
- Cover the new command in `mw::MiddleWare::main()` as well.
- Expand the `DatabaseOps` trait if necessary.
- Implement the new function in the trait.
- Check back to `mw::MiddleWare::main()` to see if everything is correct.

As the app develops, there will be less and less need for new database operations.
