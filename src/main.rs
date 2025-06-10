use clap::{Arg, Command};
use colored::Colorize;
use rand::seq::SliceRandom;
use std::fs;
use anyhow::Result;

mod models;
mod templates;

use models::MemeTemplate;



fn load_templates() -> Result<Vec<MemeTemplate>> {
    Ok(templates::load_templates())
}

#[allow(dead_code)]
fn old_load_templates() -> Vec<MemeTemplate> {
    vec![
        MemeTemplate {
            name: "drake".to_string(),
            ascii_art: r#"
             ___________
            /           \
           /             \
          /     NO:       \
         /   {{TOP}}       \
        /                   \
       /___________________/
      /                     \
     /      YES:             \
    /    {{BOTTOM}}           \
   /                           \
  /___________________________\
            "#.to_string(),
            placeholder: "{{TOP}}\n{{BOTTOM}}".to_string(),
        },
        MemeTemplate {
            name: "distracted_boyfriend".to_string(),
            ascii_art: r#"
    👨‍💼 ME: {{TOP}}
       |
       |    👀
       |   /
       v  /
    💃 {{BOTTOM}}
            "#.to_string(),
            placeholder: "{{TOP}}\n{{BOTTOM}}".to_string(),
        },
    ]
}

fn generate_meme(template: &MemeTemplate, top_text: &str, bottom_text: &str) -> String {
    let mut result = template.ascii_art.clone();
    result = result.replace("{{TOP}}", top_text);
    result = result.replace("{{BOTTOM}}", bottom_text);
    result
}

fn main() -> Result<()> {
    let matches = Command::new("MemeCLI")
        .version("1.0")
        .author("Your Name")
        .about("Create ASCII art memes")
        .arg(Arg::new("template")
            .short('t')
            .long("template")
            .help("Template name (e.g., 'drake', 'distracted_boyfriend')")
            .value_name("TEMPLATE"))
        .arg(Arg::new("top_text")
            .short('T')
            .long("top-text")
            .help("Top text for the meme")
            .value_name("TEXT"))
        .arg(Arg::new("bottom_text")
            .short('B')
            .long("bottom-text")
            .help("Bottom text for the meme")
            .value_name("TEXT"))
        .arg(Arg::new("export")
            .short('e')
            .long("export")
            .help("Export meme to file")
            .value_name("FILENAME"))
        .arg(Arg::new("random")
            .short('r')
            .long("random")
            .help("Generate a random meme")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("list-templates")
            .short('l')
            .long("list-templates")
            .help("List all available templates")
            .action(clap::ArgAction::SetTrue))
        .get_matches();

    let templates = load_templates()?;
    let mut rng = rand::thread_rng();

    if matches.get_flag("list-templates") {
        println!("Available templates:");
        for template in &templates {
            println!("- {}", template.name);
        }
        return Ok(());
    }

    let (template, top_text, bottom_text) = if matches.get_flag("random") {
        let template = templates.choose(&mut rng).unwrap();
        let random_texts = vec!["When you", "But you", "Instead", "Now you"];
        let top = random_texts.choose(&mut rng).unwrap().to_string();
        let bottom = random_texts.choose(&mut rng).unwrap().to_string();
        (template, top.clone(), bottom.clone())
    } else {
        let template = match matches.get_one::<String>("template") {
            Some(t) => templates.iter().find(|x| x.name == *t)
                .unwrap_or_else(|| templates.first().unwrap()),
            None => templates.first().unwrap(),
        };
        (
            template,
            matches.get_one::<String>("top_text").cloned().unwrap_or_else(|| "Default Top Text".to_string()),
            matches.get_one::<String>("bottom_text").cloned().unwrap_or_else(|| "Default Bottom Text".to_string()),
        )
    };

    let meme = generate_meme(template, &top_text, &bottom_text);
    println!("{}", meme.green().bold());

    if let Some(filename) = matches.get_one::<String>("export") {
        fs::write(filename, meme)?;
        println!("Meme saved to {}", filename.blue());
    }

    Ok(())
}