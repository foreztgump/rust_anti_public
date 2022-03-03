use std::fs;
use std::io::Write;
use indicatif::{ProgressBar, ProgressStyle};
use rusqlite::{Connection, Result, Statement, params, NO_PARAMS};
use crate::utils::{FileHandler, lines_from_file};

mod utils;

fn main() -> Result<()> {
    let new_database = false;
    let vul_path = FileHandler::read_file("links.txt");
    let database_path = FileHandler::read_file("vulnerable.db");
    let output_path = FileHandler::read_file("output_links.txt");

    let mut output_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(output_path.get_path())
        .unwrap();
    let conn = Connection::open(database_path.get_path())?;
    if new_database {
        conn.execute(
            "CREATE TABLE links ( link TEXT NOT NULL )",
            params![],
        )?;
    }

    println!("Starting the process...");

    let mut duplicate = 0;
    let mut new = 0;
    let mut links_to_insert = vec![String::new()];
    let links: Vec<String> = lines_from_file(vul_path.get_path()).expect("Could not read line from file");

    //make progress bar
    let pb = ProgressBar::new(links.len() as u64);
    pb.set_style(ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] (Dorks : {pos}/{len})",
    ));

    for line in links.into_iter() {
        pb.inc(1);
        let mut stmt = conn.prepare("SELECT link FROM links where link = :line")?; //"SELECT link VALUE (:link) FROM links""SELECT * FROM test where link = :link"
        //println!("Searching DB: {}", line);

        let mut rows = stmt.query_named(&[(":line",&line)])?;
        if let Some(_row) = rows.next()? {
            duplicate += 1;
        } else {
            links_to_insert.push(line.clone());
            write!(output_file, "{}\n", line).unwrap();
            new += 1;
        };
    }

    pb.finish();
    println!("New Links: {}", new);
    println!("Duplicated Links: {}", duplicate);

    let mut links_context = DbContext::new(&conn);
    links_context.conn.execute_batch("BEGIN TRANSACTION;")?;

    for links in links_to_insert.into_iter() {
        links_context.create_link(&links)?;
    }

    links_context.conn.execute_batch("COMMIT TRANSACTION;")?;
    println!("Done");

    Ok(())
}

struct DbContext<'a> {
    conn: &'a Connection,
    create_url_statement: Option<Statement<'a>>,
}

impl<'a> DbContext<'a> {
    fn new(conn: &'a Connection) -> Self {
        DbContext {
            conn,
            create_url_statement: None,
        }
    }

    fn create_link(&mut self, link: &String) -> Result<i64> {
        if let None = &self.create_url_statement {
            let stmt = self.conn.prepare("INSERT INTO links (link) VALUES (:link)")?;
            self.create_url_statement = Some(stmt);
        };
        self.create_url_statement.as_mut().unwrap().execute_named(&[(":link", &link)])?;
        return Ok(self.conn.last_insert_rowid());
    }
}

#[allow(dead_code)]
fn get_names(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT link FROM links")?;
    let mut rows = stmt.query(NO_PARAMS)?;

    let mut names = Vec::new();
    while let Some(row) = rows.next()? {
        names.push(row.get(0)?);
    }

    Ok(names)
}