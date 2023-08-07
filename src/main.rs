

fn main() {
    println!("hello world");

    use docx_rust::document::Paragraph;
    use docx_rust::DocxFile;
    
    let docx = DocxFile::from_file("11.docx").unwrap();
    let mut docx = docx.parse().unwrap();

    //println!("{:#?}", docx);

    println!("{:#?}", docx.document);


    for (key, value) in docx.headers.iter() {
        println!("key: {}", key);
      }

    
    let para = Paragraph::default().push_text("Lorem Ipsum");
    docx.document.push(para);
    
    docx.write_file("12.docx").unwrap();
}
