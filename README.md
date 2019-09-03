# forkie-talkie

A tool for fast, automatic internationalization.

## Usage

After using `smileyman`(https://github.com/Kvalifik/smileyman) to fetch human text into markdown files, and then having a language guy translate those markdown files,
this tool will automatically generate proper Vue boilerplate for i18n language handling. This is done using the given markdown files, and will automatically internationalize the app.

### Example

- Have markdown file with danish text.
- Have markdown file with english text.
- Have Vue component with elements corresponding to those in the markdown files.
- ForkieTalkie(tm) generates and inserts nice procedural code into the Vue component.