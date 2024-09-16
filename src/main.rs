use std::env;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::Path;
use csv::Reader;

const HTML_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Video Evidence of {current_dir}</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>

<h2>Video Evidence of {current_dir}</h2>
<p>Below is a list of evidence that proves the violation:</p>

<p>Click the blue link to View Video. Or, </p>

<p><a href="{current_dir}-all-videos.html">View all videos on a single scrolling page</a></p>

<table>
  <thead>
    <tr>
      <th>Datetime</th>
      <th>Description</th>
      <th>Event ID</th>
      <th>Monitor</th>
      <th>Video File</th>
    </tr>
  </thead>
  <tbody>
    {table_rows}
  </tbody>
</table>

<p><a href="{current_dir}-all-videos.html">View all videos on a single page</a></p>

</body>
</html>
"#;

const ALL_VIDEOS_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>All Videos for {current_dir}</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>

<h2>All Videos for {current_dir}</h2>
<p>Below are all the videos related to this violation:</p>

<div class="video-container">
    {videos}
</div>

</body>
</html>
"#;

fn generate_table_rows(csv_path: &str) -> String {
    let mut table_rows = String::new();
    let file = File::open(csv_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut csv_reader = Reader::from_reader(reader);

    for result in csv_reader.records() {
        let record = result.expect("Error reading CSV record");
        let datetime = &record[0];
        let mut description = record[1].to_string();
        let event_id = &record[2];
        let monitor = &record[3];
        let video_file = record[4].replace(".webp", ".webm");

        if let Ok(hour) = datetime.split_whitespace().nth(1).unwrap_or("").split(':').next().unwrap_or("").parse::<u32>() {
            if hour >= 20 {
                description += " <strong style='color: red;'>AFTER PERMITTED HOURS</strong>";
            }
        }

        table_rows.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td><a href='{}' target='_blank'>View Video</a></td></tr>\n",
            datetime, description, event_id, monitor, video_file
        ));
    }

    table_rows
}

fn generate_video_elements(csv_path: &str) -> String {
    let mut videos_html = String::new();
    let file = File::open(csv_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut csv_reader = Reader::from_reader(reader);

    for result in csv_reader.records() {
        let record = result.expect("Error reading CSV record");
        let datetime = &record[0];
        let mut description = record[1].to_string();
        let video_file = record[4].replace(".webp", ".webm");

        if let Ok(hour) = datetime.split_whitespace().nth(1).unwrap_or("").split(':').next().unwrap_or("").parse::<u32>() {
            if hour >= 20 {
                description += " <strong style='color: red;'>AFTER PERMITTED HOURS</strong>";
            }
        }

        videos_html.push_str(&format!(
            "<div class=\"video\">
                <h3>{} - {}</h3>
                <video controls>
                    <source src=\"{}\" type=\"video/webm\">
                    Your browser does not support the video tag.
                </video>
            </div>\n",
            description, datetime, video_file
        ));
    }

    videos_html
}

fn main() {
    let current_dir = env::current_dir().unwrap();
    let current_dir_str = current_dir.file_name().unwrap().to_str().unwrap();

    let csv_file_path = format!("{}.csv", current_dir_str);
    let zmdata_html_file = format!("{}-zmdata.html", current_dir_str);
    let all_videos_html_file = format!("{}-all-videos.html", current_dir_str);
    let index_html_file = "index.html";
    let template_1_file = format!("{}-1.html", current_dir_str);

    let table_rows = generate_table_rows(&csv_file_path);
    let videos_html = generate_video_elements(&csv_file_path);

    // Write zmdata HTML file
    let zmdata_html_content = HTML_TEMPLATE.replace("{current_dir}", current_dir_str).replace("{table_rows}", &table_rows);
    let mut zmdata_html_file = File::create(&zmdata_html_file).expect("Unable to create zmdata HTML file");
    zmdata_html_file.write_all(zmdata_html_content.as_bytes()).expect("Unable to write zmdata HTML file");

    // Write all videos HTML file
    let all_videos_html_content = ALL_VIDEOS_TEMPLATE.replace("{current_dir}", current_dir_str).replace("{videos}", &videos_html);
    let mut all_videos_html_file = File::create(&all_videos_html_file).expect("Unable to create all videos HTML file");
    all_videos_html_file.write_all(all_videos_html_content.as_bytes()).expect("Unable to write all videos HTML file");

    // Combine template -1 and zmdata into index.html
    let mut index_file = File::create(index_html_file).expect("Unable to create index HTML file");
    if Path::new(&template_1_file).exists() {
        let template_content = fs::read_to_string(&template_1_file).expect("Unable to read template file");
        index_file.write_all(template_content.as_bytes()).expect("Unable to write template content to index HTML file");
    }
    index_file.write_all(zmdata_html_content.as_bytes()).expect("Unable to write zmdata content to index HTML file");

    println!("HTML files {:?} and {:?} generated successfully.", zmdata_html_file, all_videos_html_file);
}

