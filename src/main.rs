use clap::{App, Arg, ArgMatches};
use colored::Colorize;
use rand::seq::SliceRandom;
use std::fs;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MemeTemplate {
    name: String,
    ascii_art: String,
    placeholder: String,
}

fn load_templates() -> Result<Vec<MemeTemplate>> {
    let templates = vec![
        MemeTemplate {
            name: "drake".to_string(),
            ascii_art: r#"
             ___________
            /           \
           /             \
          /               \
         /                 \
        /                   \
       /                     \
      /                       \
     /                         \
    /                           \
   /                             \
  /                               \
 /                                 \
/                                   \
\                                   /
 \                                 /
  \                               /
   \                             /
    \                           /
     \                         /
      \                       /
       \                     /
        \                   /
         \                 /
          \               /
           \             /
            \___________/
            "#.to_string(),
            placeholder: "{{TOP}}\n{{BOTTOM}}".to_string(),
        },
        MemeTemplate {
            name: "distracted_boyfriend".to_string(),
            ascii_art: r#"
             ___________
            /           \
           /             \
          /               \
         /                 \
        /                   \
       /                     \
      /                       \
     /                         \
    /                           \
   /                             \
  /                               \
 /                                 \
/                                   \
\                                   /
 \                                 /
  \                               /
   \                             /
    \                           /
     \                         /
      \                       /
       \                     /
        \                   /
         \                 /
          \               /
           \             /
            \___________/
            "#.to_string(),
            placeholder: "{{TOP}}\n{{BOTTOM}}".to_string(),
        },
    ];
    Ok(templates)
}

fn generate_meme(template: &MemeTemplate, top_text: &str, bottom_text: &str) -> String {
    let mut result = template.ascii_art.clone();
    result = result.replace("{{TOP}}", top_text);
    result = result.replace("{{BOTTOM}}", bottom_text);
    result
}

fn main() -> Result<()> {
    let matches = App::new("MemeCLI")
        .version("1.0")
        .author("Your Name")
        .about("Create ASCII art memes")
        .arg(Arg::new("template")
            .short('t')
            .long("template")
            .help("Template name (e.g., 'drake', 'distracted_boyfriend')")
            .takes_value(true))
        .arg(Arg::new("top_text")
            .short('T')
            .long("top-text")
            .help("Top text for the meme")
            .takes_value(true))
        .arg(Arg::new("bottom_text")
            .short('B')
            .long("bottom-text")
            .help("Bottom text for the meme")
            .takes_value(true))
        .arg(Arg::new("export")
            .short('e')
            .long("export")
            .help("Export meme to file")
            .takes_value(true))
        .arg(Arg::new("random")
            .short('r')
            .long("random")
            .help("Generate a random meme")
            .takes_value(false))
        .get_matches();

    let templates = load_templates()?;
    let mut rng = rand::thread_rng();

    let (template, top_text, bottom_text) = if matches.is_present("random") {
        let template = templates.choose(&mut rng).unwrap();
        let random_texts = vec!["When you", "But you", "Instead", "Now you"];
        let top = random_texts.choose(&mut rng).unwrap().to_string();
        let bottom = random_texts.choose(&mut rng).unwrap().to_string();
        (template, top, bottom)
    } else {
        let template = match matches.value_of("template") {
            Some(t) => templates.iter().find(|x| x.name == t)
                .unwrap_or_else(|| templates.first().unwrap()),
            None => templates.first().unwrap(),
        };
        (
            template,
            matches.value_of("top_text").unwrap_or(""),
            matches.value_of("bottom_text").unwrap_or(""),
        )
    };

    let meme = generate_meme(template, &top_text, &bottom_text);
    println!("{}", meme.green().bold());

    if let Some(filename) = matches.value_of("export") {
        fs::write(filename, meme)?;
        println!("Meme saved to {}", filename.blue());
    }

    Ok(())
}
