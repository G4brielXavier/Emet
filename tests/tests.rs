use emet::Emet;
use std::fs;

#[test]
fn test_pdf_book_simulatio() -> Result<(), Box<dyn std::error::Error>> {

    let book_path = "mock_book.pdf";
    let emet_path = "mock_book.pdf.emet";
    let private_key = "secret_author_key_123".to_string();


    let mut emet = Emet::up(private_key);
    let mut book_content = Vec::new();

    for i in 0..1_000_000 {
        book_content.push((i % 255) as u8);
    }

    fs::write(book_path, &book_content)?;

    println!("Sealing the book...");
    let seal = emet.seal(book_path)?;
    emet.save_seal(book_path, &seal)?;

    println!("Checking Integrity (should pass) ...");
    emet.check(book_path, emet_path)?;

    println!("Original PDF is authentic!");

    fs::remove_file(book_path)?;
    fs::remove_file(emet_path)?;    

    Ok(())

}