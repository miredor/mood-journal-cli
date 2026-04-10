use crate::journal::{parse_tags, Journal, JournalError};
use crate::prompts::PROMPTS;
use clap::{Args, Parser, Subcommand};
use rand::seq::SliceRandom;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "mood-journal",
    version,
    about = "A tiny CLI journal with mood tracking, tags, and stats"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Optional custom storage path for the JSON database
    #[arg(long, global = true)]
    pub file: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add a new journal entry
    Add(AddArgs),
    /// List recent entries
    List(ListArgs),
    /// Show overall mood statistics
    Stats,
    /// Print a random journaling prompt
    Prompt,
}

#[derive(Debug, Args)]
pub struct AddArgs {
    /// Mood score from 1 to 5
    #[arg(long)]
    pub mood: u8,

    /// Comma-separated tags, for example: work,health,ideas
    #[arg(long, default_value = "")]
    pub tags: String,

    /// The entry text
    pub text: String,
}

#[derive(Debug, Args)]
pub struct ListArgs {
    /// Filter entries by a single tag
    #[arg(long)]
    pub tag: Option<String>,

    /// Maximum number of entries to show
    #[arg(long)]
    pub limit: Option<usize>,
}

pub fn run(cli: Cli) -> Result<(), JournalError> {
    let mut journal = match cli.file {
        Some(path) => Journal::load_from(path)?,
        None => Journal::load_default()?,
    };

    match cli.command {
        Commands::Add(args) => add_command(&mut journal, args),
        Commands::List(args) => list_command(&journal, args),
        Commands::Stats => stats_command(&journal),
        Commands::Prompt => prompt_command(),
    }

    Ok(())
}

fn add_command(journal: &mut Journal, args: AddArgs) {
    if !(1..=5).contains(&args.mood) {
        eprintln!("Mood must be between 1 and 5.");
        std::process::exit(2);
    }

    let tags = parse_tags(&args.tags);
    let entry = journal
        .add_entry(args.mood, tags, args.text)
        .expect("validated journal write");

    println!("Saved entry #{}", entry.id);
    println!("Mood: {}/5", entry.mood);
    if entry.tags.is_empty() {
        println!("Tags: none");
    } else {
        println!("Tags: {}", entry.tags.join(", "));
    }
    println!("Stored in: {}", journal.path().display());
}

fn list_command(journal: &Journal, args: ListArgs) {
    let entries = journal.list_entries(args.tag.as_deref(), args.limit);
    if entries.is_empty() {
        println!("No entries found.");
        return;
    }

    for entry in entries {
        println!(
            "#{} [{}] mood {}/5",
            entry.id,
            entry.created_at.format("%Y-%m-%d %H:%M"),
            entry.mood
        );
        if entry.tags.is_empty() {
            println!("tags: none");
        } else {
            println!("tags: {}", entry.tags.join(", "));
        }
        println!("{}", entry.text);
        println!();
    }
}

fn stats_command(journal: &Journal) {
    let stats = journal.stats();
    println!("Entries: {}", stats.count);
    if stats.count == 0 {
        println!("Average mood: n/a");
        println!("Best mood: n/a");
        println!("Worst mood: n/a");
        return;
    }

    println!("Average mood: {:.2}/5", stats.average);
    println!("Best mood: {}/5", stats.best_mood.unwrap_or(0));
    println!("Worst mood: {}/5", stats.worst_mood.unwrap_or(0));
}

fn prompt_command() {
    let mut rng = rand::thread_rng();
    let prompt = PROMPTS
        .choose(&mut rng)
        .expect("prompt list should not be empty");
    println!("Journal prompt:\n{prompt}");
}
