# Emet (אמת)

> **"Mathematics does not lie. Truth is immutable."**

**Emet** (Hebrew for *Truth*) is a command-line tool built in **Rust** designed to guarantee the integrity and authorship of digital files. Whether sealing the source code of a critical system or the chapters of an epic 23-volume saga (for example), Emet creates a mathematical proof that a specific content existed at a specific time and was signed by you.

At its core, Emet utilizes **Tequel 0.7.6**, a custom low-level hashing library, ensuring that even a single-bit alteration is detected.



---

## 🚀 Features (v0.1.0)

* **Seal:** Generates a digital "fingerprint" of the file by combining the content, a UTC timestamp, and your private key.
* **Audit:** Verifies if the current file matches the original seal through the `check` command.
* **Double-Check Security:** Validates both **Integrity** (has the physical file changed?) and **Authenticity** (does the signature belong to the true author?).
* **Transparent Persistence:** Generates `.emet` files in JSON format, allowing for human auditing and easy archiving.

---

## 🛠️ The Workflow of Truth

The sealing and verification process follows a rigorous cryptographic pipeline:

1.  **Hashing:** Emet reads the file bytes and uses the **Tequel** algorithm to generate a high-entropy hash.
2.  **Signing:** The hash is concatenated with the *timestamp* and your `private_key`. The result of this mix is hashed again to create the `digital_signature`.
3.  **Verification:** When running `check`, Emet reconstructs the signature with current data. If a single character in the original file or the date in the seal is modified, the "Truth is Violated."

---

## 💻 Usage Example

In your `main.rs`:

```rust
// Instance Emet
let mut emet = Emet::up("private_key");

// Sealing a file
let sealed = emet.seal("chapter_01.md")?;
emet.save_seal("chapter_01.md", &sealed)?;

// Verifying integrity
match emet.check("chapter_01.md", "chapter_01.md.emet") {
    Ok(_) => println!("✅ Good! The truth remains intact."),
    Err(e) => eprintln!("❌ Error: {}", e), // e.g., Truth Violated
}
```

or via CLI:

```bh
emet up my_private_key
emet seal ./myfile.txt
emet check ./myfile.txt ./myfile.txt.emet
```

## 🧬 Tecnnical Specifications

- Language: Rust (2024 Edition)
- Hashing Engine: [Tequel v0.7.6](https://crates.io/crates/tequel-rs) (7.999885 bits/byte entropy)
- Seal Format: JSON (via `Serde`)
- Time Standard: UTC RCF3339 (via `Chrono`)


## 📜 Philosophy

Emet is not just a checksum utility; it is a personal digital notary. It was built to protect the vision of creators who understand that while the future may be uncertain, the past and authorship must be incontestable.

**Developed by Gabriel "dotxav" Xavier.** *If the truch can be corrupted, it is not the truth.*


## 📄 License

Licensed under the MIT License. Free to use, modify, and distribute.
