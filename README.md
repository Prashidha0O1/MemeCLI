# MemeCLI

A command-line tool for creating and sharing ASCII art memes right in your terminal! 🚀

## Features

- Multiple meme templates to choose from
- Customizable top and bottom text
- Random meme generation
- Export memes to text files
- ASCII art templates including:
  - Classic memes (Drake, Distracted Boyfriend)
  - Cider-themed templates
  - Brainrot templates
  - And more!

## Installation

1. Make sure you have [Rust installed](https://www.rust-lang.org/tools/install) on your system.

2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/MemeCLI.git
   cd MemeCLI
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

4. (Optional) Add the binary to your PATH or run it directly from `target/release/memecli`

## Usage

### Basic Usage

```bash
# Show help
memecli --help

# List all available templates
memecli --list-templates

# Create a meme with a specific template
memecli --template cat_feature --top-text "Can I haz cheezburger?" --bottom-text "Plz?"

# Generate a random meme
memecli --random

# Export a meme to a file
memecli --template deal_with_it --top-text "When it works on first try" --export meme.txt
```

### Examples

#### 1. Create a "Deal With It" meme
```bash
memecli --template deal_with_it --top-text "When you finally fix all the bugs" --bottom-text "And then find a new one"
```

#### 2. Create a coding life meme
```bash
memecli --template coding_life --top-text "eat();" --bottom-text "code();"
```

#### 3. Generate a random meme with random text
```bash
memecli --random
```

## Available Templates

To see all available templates, run:
```bash
memecli --list-templates
```

## Adding Custom Templates

1. Open `src/templates.rs`
2. Add a new `MemeTemplate` to the `load_templates()` function
3. Rebuild the project with `cargo build --release`

## Dependencies

- [clap](https://crates.io/crates/clap) - Command line argument parsing
- [colored](https://crates.io/crates/colored) - Colored terminal output
- [rand](https://crates.io/crates/rand) - Random number generation
- [anyhow](https://crates.io/crates/anyhow) - Error handling
- [serde](https://crates.io/crates/serde) - Serialization/deserialization

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Example Output

```
(•_•)
( •_•)>⌐■-■
(⌐■_■)   When you finally fix all the bugs
And then find a new one
```

---

Made with ❤️ and Rust
