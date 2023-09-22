

fn main() {
    println!("hello world");

    use docx_rust::document::Paragraph;
    use docx_rust::DocxFile;
    
    let docx_file = DocxFile::from_file("11.docx").unwrap();
    let docx = docx_file.parse().unwrap();

    //println!("{:#?}", docx);
    //println!("{:#?}", docx.document);

    //println!("{:#?}", docx.document.body);

    println!("{:#?}", docx.document.body.content);



}
