use std::{
    fs::File,
    io::{self, BufRead, Error, Write},
};

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(required = true)]
    filenames: Vec<String>,

    #[arg(default_value = "false", short = 'n', long = "number")]
    print_line_numbers: bool,

    #[arg(default_value = "false", short = 'b', long = "number-nonblank")]
    print_non_empty_line_numbers: bool,

    #[arg(default_value = "false", short = 'E', long = "show-ends")]
    show_ends: bool,

    #[arg(default_value = "false", short = 'T', long = "show-tabs")]
    show_tabs: bool,

    #[arg(default_value = "false", short = 'A', long = "show-all")]
    show_all: bool,
}

enum Show {
    Tabs,
    Ends,
    All,
}

struct Options {
    print_line_numbers: bool,
    print_non_empty_line_numbers: bool,
    show: Option<Show>,
}

impl From<&Args> for Options {
    fn from(args: &Args) -> Self {
        let show = if args.show_all || (args.show_ends && args.show_tabs) {
            Some(Show::All)
        } else if args.show_ends {
            Some(Show::Ends)
        } else if args.show_tabs {
            Some(Show::Tabs)
        } else {
            None
        };

        Options {
            print_line_numbers: args.print_line_numbers,
            print_non_empty_line_numbers: args.print_non_empty_line_numbers,
            show,
        }
    }
}

pub fn run(args: Args) -> io::Result<()> {
    let mut line_number = 0 as usize;

    let options = Options::from(&args);

    for filename in &args.filenames {
        print_lines(filename, &mut line_number, &options)
            .map_err(|error| Error::new(error.kind(), format!("{filename}: {error}")))?;
    }

    Ok(())
}

fn print_lines(filename: &str, line_number: &mut usize, options: &Options) -> Result<(), Error> {
    let file = File::open(filename)?;
    let mut reader = io::BufReader::new(file);
    let mut line = String::new();
    let mut output = io::stdout().lock();

    while reader.read_line(&mut line)? != 0 {
        write!(output, "{}", prepare_string(&line, line_number, options))?;
        line.clear();
    }

    output.flush()
}

fn prepare_string(line: &str, line_number: &mut usize, options: &Options) -> String {
    let has_newline = line.ends_with('\n');
    let line = line.strip_suffix('\n').unwrap_or(line);
    let mut ans = String::new();

    if line != "" || !options.print_non_empty_line_numbers {
        *line_number += 1;
    }

    if (options.print_line_numbers || options.print_non_empty_line_numbers)
        && (!options.print_non_empty_line_numbers || !line.is_empty())
    {
        ans.push_str(&line_number.to_string());
        ans.push(' ');
    }

    let show_tabs = matches!(options.show, Some(Show::Tabs | Show::All));
    let show_ends = matches!(options.show, Some(Show::Ends | Show::All));

    for char in line.chars() {
        if char == '\t' && show_tabs {
            ans.push_str("^I");
        } else {
            ans.push(char);
        }
    }

    if has_newline {
        if show_ends {
            ans.push('$');
        }
        ans.push('\n');
    }

    ans
}
