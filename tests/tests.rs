use emet::Emet;
use std::fs;

#[test]
fn test_seal_and_check() -> Result<(), Box<dyn std::error::Error>> {
    
    // To test it, firstly create a my_test.txt with content and then execute this
    let my_private_key = "private123";
    let original_path = "my_test.txt";
    
    let mut emet = Emet::up(my_private_key.to_string());

    let seal = emet.seal(original_path)?;

    emet.save_seal(original_path, &seal)?;

    let is_checked = emet.check(original_path, "my_test.txt.emet");

    assert!(is_checked.is_ok(), "");

    Ok(())
}



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