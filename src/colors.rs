pub const NC: anstyle::Reset = anstyle::Reset;

pub const GREEN: anstyle::Style = anstyle::Style::new()
    .bold()
    .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green)));

pub const YELLOW: anstyle::Style = anstyle::Style::new()
    .bold()
    .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow)));

pub const RED: anstyle::Style = anstyle::Style::new()
    .bold()
    .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red)));

pub const CYAN: anstyle::Style = anstyle::Style::new()
    .bold()
    .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Cyan)));
