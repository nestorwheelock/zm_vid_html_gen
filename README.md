
# zm_video_html_generator

**zm_video_html_generator** is a Rust-based tool designed to process video evidence related to leasehold premises and potential lease violations. This tool works in conjunction with other tools in the same suite, including **zm_clean_export** and **zm_web_proc**, to process, organize, and display video event data in a structured HTML format.

### Background

This tool is specifically designed to help landlords, property managers, or legal professionals organize video evidence collected from the leasehold premises, such as from security systems like ZoneMinder. The tool processes video event data and generates web pages that can be used to document and display potential lease violations, making the review of these events straightforward and organized.

The tool is highly customizable and can be integrated with lease terms, additional documents, or video links, providing an all-in-one solution for managing and reviewing video evidence from security footage.

---

## Features

- **Video Evidence Organization**: Converts a CSV file of video event data into structured HTML pages for easy review.
- **Integration with `zm_clean_export`**: This tool cleans and formats exported ZoneMinder data, making it suitable for further use with `zm_video_html_generator`.
- **Integration with `zm_web_proc`**: This tool processes ZoneMinder exports and prepares the video files by converting them to `.webm` format.
- **Time-Based Alerts**: Highlights video events that occur after permitted hours (after 8 PM) to help quickly identify lease violations.
- **Combines into Index**: Generates a combined `index.html` page that links to additional resources like lease terms and legal documents via a `template-1.html` file.

---

## Toolchain

- **zm_clean_export**: Processes and cleans ZoneMinder exports to format CSV data for video events. You can find this tool at:
  - [zm_clean_export GitHub](https://github.com/your-username/zm_clean_export)

- **zm_web_proc**: Extracts and processes ZoneMinder video files, converting them to `.webm` format for use in this tool. You can find this tool at:
  - [zm_web_proc GitHub](https://github.com/your-username/zm_web_proc)

---

## Installation

### Step 1: Clone the Repository

```bash
git clone https://github.com/your-username/zm_video_html_generator.git
cd zm_video_html_generator
```

### Step 2: Build the Project

```bash
cargo build --release
```

## Usage

1. **Prepare CSV Data**: Make sure the CSV file is generated using **zm_clean_export** or manually constructed. The CSV file should be named after the current directory, e.g., if the directory is called `violations`, the CSV should be `violations.csv`.
2. **Prepare Video Files**: Process your video files using **zm_web_proc** to convert `.mp4` files to `.webm` format, ready to be linked in the HTML pages.
3. **Run the Tool**: 

```bash
./target/release/zm_video_html_generator
```

This will generate the following files in your working directory:
- `<directory_name>-zmdata.html`: Displays the video evidence in a table format.
- `<directory_name>-all-videos.html`: Displays all video events on a single scrolling page.
- `index.html`: A page combining the generated HTML files with navigation links.

### Example

```bash
./target/release/zm_video_html_generator
```

This command will generate the HTML files based on the data in the CSV file located in the same directory.

### Input CSV Format

The CSV file must have the following columns:

- **Datetime**: The timestamp of the video event (format: `%m/%d/%y %H:%M:%S`).
- **Description**: A description of the event.
- **Event ID**: The unique identifier for the event.
- **Monitor**: The camera or monitor used for recording.
- **Video File**: The path to the video file.

---

## Example of the `template-1.html`

The `template-1.html` file can be used to link additional documents or lease terms related to the premises. It will be automatically included in the generated `index.html` file.

Here is an example:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Lease Terms and Documents</title>
</head>
<body>
    <h2>Relevant Lease Terms and Documents</h2>
    <p>The following documents and terms are applicable to this case:</p>
    <ul>
        <li><a href="lease_terms.pdf" target="_blank">Lease Terms</a></li>
        <li><a href="legal_notice.pdf" target="_blank">Legal Notice of Violation</a></li>
    </ul>
</body>
</html>
```

This file should be placed in the same directory as the program for proper inclusion in the generated `index.html`.

---

## Example of `style.css`

You can add custom styling by placing a `style.css` file in the working directory. Here’s an example for basic styling:

```css
body {
    font-family: Arial, sans-serif;
    background-color: #f9f9f9;
}

h2 {
    color: #333;
}

table {
    width: 100%;
    border-collapse: collapse;
    margin-top: 20px;
}

th, td {
    border: 1px solid #ddd;
    padding: 8px;
}

th {
    background-color: #4CAF50;
    color: white;
}

tr:nth-child(even) {
    background-color: #f2f2f2;
}

a {
    color: #1a73e8;
    text-decoration: none;
}

a:hover {
    text-decoration: underline;
}
```

---

## Example Output

Once the program is run, the following HTML files will be generated:

- `<directory_name>-zmdata.html`: Contains the video evidence in a table.
- `<directory_name>-all-videos.html`: Contains all video events in a scrollable format.
- `index.html`: Combines the generated pages and includes links to external documents from `template-1.html`.

Example output:

```
HTML files violations-zmdata.html and violations-all-videos.html generated successfully.
```

## License

This project is licensed under the GNU GPLv3 License. See the [LICENSE](LICENSE) file for more details.
