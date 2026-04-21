**Column setup**

```rust
// commands.rs
use comfy_table::{Table, Column, Cell, Color, Attribute, ContentArrangement};
use colored::Colorize;

pub fn list_tasks(tasks: &[Task]) {
    let mut table = Table::new();

    table
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("ID").add_attribute(Attribute::Dim),
            Cell::new("TITLE").add_attribute(Attribute::Dim),
            Cell::new("STATUS").add_attribute(Attribute::Dim),
            Cell::new("UPDATED").add_attribute(Attribute::Dim),
        ]);

    for task in tasks {
        table.add_row(vec![
            Cell::new(&task.id[..8]).fg(Color::DarkGrey),
            Cell::new(format_title(&task)),
            Cell::new(format_status(&task.status)),
            Cell::new(format_relative_time(&task.updated_at)).fg(Color::DarkGrey),
        ]);
    }

    println!("{table}");
    println!("{}", format_footer(tasks));
}
```

**Title with optional sub-line**

```rust
fn format_title(task: &Task) -> String {
    match &task.description {
        Some(desc) => format!("{}\n{}", task.title, desc.dimmed()),
        None => task.title.clone(),
    }
}
```

**Status badge** — `comfy-table` can't do rounded badges, but colored background blocks read well enough:

```rust
fn format_status(status: &Status) -> Cell {
    match status {
        Status::Todo  => Cell::new(" todo  ").fg(Color::Grey).bg(Color::AnsiValue(237)),
        Status::Doing => Cell::new(" doing ").fg(Color::Blue).bg(Color::AnsiValue(17)),
        Status::Done  => Cell::new(" done  ").fg(Color::Green).bg(Color::AnsiValue(22)),
    }
}
```

The `AnsiValue`s here are 256-color dark grays/blues/greens that approximate the mockup's badge backgrounds. Adjust to taste — `17` is a very dark navy, `22` a very dark green.

**Relative timestamps** using `chrono`:

```rust
fn format_relative_time(dt: &DateTime<Utc>) -> String {
    let secs = Utc::now().signed_duration_since(*dt).num_seconds();
    match secs {
        s if s < 3600        => format!("{}m ago", s / 60),
        s if s < 86400       => format!("{}h ago", s / 3600),
        s if s < 86400 * 2   => "yesterday".into(),
        s                    => format!("{}d ago", s / 86400),
    }
}
```

**Footer summary line**

```rust
fn format_footer(tasks: &[Task]) -> String {
    let done  = tasks.iter().filter(|t| t.status == Status::Done).count();
    let doing = tasks.iter().filter(|t| t.status == Status::Doing).count();
    let todo  = tasks.iter().filter(|t| t.status == Status::Todo).count();

    format!(
        "{} tasks  ·  {} done  ·  {} doing  ·  {} todo",
        tasks.len().to_string().dimmed(),
        done.to_string().bold(),
        doing.to_string().bold(),
        todo.to_string().bold(),
    )
}
```

**Table styling** — remove the default borders to get the clean divider-only look from the mockup:

```rust
use comfy_table::presets::UTF8_HORIZONTAL_ONLY;

table.load_preset(UTF8_HORIZONTAL_ONLY);
```

This gives you only the horizontal rule under the header, which matches the design exactly. The `UTF8_FULL` preset (default) adds box-drawing characters around the whole table — not what you want here.

**`Cargo.toml` additions** if you haven't added these yet:

```toml
[dependencies]
comfy-table = "7"
colored     = "2"
chrono      = { version = "0.4", features = ["serde"] }
```

The main limitation to be aware of: `colored` respects the `NO_COLOR` env var and auto-disables on non-TTY output (e.g. piped to a file), which is exactly the right behavior for a CLI tool.