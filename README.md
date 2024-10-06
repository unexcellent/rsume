# rsume
A tool for effortlessly generating resumes.

## Elevator Pitch
The hiring process as a software developer can be super tedious. You never hear back from the majority of companies you have applied to and if they reach out, you have to jump through a lot of hoops to land the job. Since it is recommended to customize your resume for every single application, it may mean that you have to create dozens of resume before finally getting hired. Who has time for that? This tool is here to simplify the process by generating a high-quality resume with minimal work required.

## Getting started
Currently, the only supported method for installing this program is by downloading or cloning this repo and building the binary yourself using `cargo` or `rustc`.

An instance of Google Chrome or Chromedriver is required for executing the program.

## Usage
`rsume`should be used from the command line like this:
```bash
rsume /path/to/resume_data.yaml /target/path.pdf --template "coruscant" --language "english"
```
The `--template` and `--language` options are optional.

The resume data should follow the [JSONResume](https://jsonresume.org/) schema and can either be stored as a `.json` or `.yaml` file.

## Known Issues
Currently, only a single template is available. In the future more template are planned.
- If the content of the resume is short enough that only one page is filled, an empty second page is generated regardless
- The box shadows in the coruscant template look as intended when viewed on all chromium-based browsers and Adobe Acrobat but too dark in Firefox and Preview
- Page breaks in the coruscant template may separate the section title (like "Education") from the first entry