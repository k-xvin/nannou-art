use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Path;
use svg::node::element::Path;
use svg::parser::Event;
use svg::Document;

fn main() {
    //let path = "image.svg";
    let path = "mantaray-grid.svg";
    let mut content = String::new();
    for event in svg::open(path, &mut content).unwrap() {
        match event {
            Event::Tag(Path, _, attributes) => {
                let data = attributes.get("d").unwrap();
                let data = Data::parse(data).unwrap();
                for command in data.iter() {
                    println!("{:?}", command);
                    match command {
                        &Command::Move(..) => println!("Move!"),
                        &Command::Line(..) => println!("Line!"),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
