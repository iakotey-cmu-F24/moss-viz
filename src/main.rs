use std::{
    io::Write,
    process::{Command, Stdio},
};

use dot_graph::{Edge, Graph, Kind, Style};
use lazy_static::lazy_static;
use regex::Regex;

use scraper::{Html, Selector};
fn main() -> std::io::Result<()> {
    lazy_static! {
        static ref NameREGEX: Regex =
            Regex::new(r"^(?P<name>.+)\s+\((?P<perc>\d{1,2})%\)$").unwrap();
    };

    let url = "http://moss.stanford.edu/results/<FAKE_URL>";
    let url = std::env::var("MOSS_URL").unwrap();
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    // println!("{:#?}", response);
    let document = Html::parse_document(&response);
    let matches: Selector = Selector::parse("body > table > tbody > tr").unwrap();
    let cols: Selector = Selector::parse("td").unwrap();
    let links: Selector = Selector::parse("a").unwrap();

    let list = document
        .select(&matches)
        .skip(1)
        .map(|x| x.select(&cols).take(3))
        .map(|mut x| {
            (
                x.next()
                    .unwrap()
                    .select(&links)
                    .next()
                    .map(|x| (x.inner_html(), x.value().attr("href").unwrap()))
                    .unwrap(),
                x.next()
                    .unwrap()
                    .select(&links)
                    .next()
                    .map(|x| (x.inner_html(), x.value().attr("href").unwrap()))
                    .unwrap(),
                x.next()
                    .unwrap()
                    .inner_html()
                    .trim()
                    .parse::<u32>()
                    .unwrap(),
            )
        })
        .map(|(p1, p2, lines)| {
            let m1 = NameREGEX.captures(&p1.0).unwrap();
            let m2 = NameREGEX.captures(&p2.0).unwrap();
            (
                m1.name("name").unwrap().as_str().to_string(),
                m2.name("name").unwrap().as_str().to_string(),
                p1.1,
                m1.name("perc").unwrap().as_str().parse::<u8>().unwrap(),
                lines,
            )
        })
        .collect::<Vec<_>>();

    let mut graph = Graph::new("Test", Kind::Graph);

    for edge in &list {
        if edge.3 >= 50 {
            graph.add_edge(
                Edge::new(&edge.0, &edge.1, "")
                    .style(Style::Bold)
                    .color(Some("red"))
                    .label(&format!("{}% ({})", edge.3, edge.4))
                    .url(edge.2.to_string()),
            );
        }
    }

    let dot_string = graph.to_dot_string().unwrap();

    // println!("{:#?}", list);
    // println!("{}", dot_string);

    let mut dot_pro = Command::new("circo")
        .arg("-T")
        .arg("svg")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = dot_pro.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(dot_string.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = dot_pro.wait_with_output().expect("Failed to read stdout");

    std::fs::File::create("test.svg")
        .unwrap()
        .write_all(&output.stdout)
}
