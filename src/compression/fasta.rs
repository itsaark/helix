//! `FASTA` is a widely used format for storing DNA. 
//! It maps nucleic acids or amino acids to codes to 
//! reduce the storage size.
//! See [BLAST's documentation](https://blast.ncbi.nlm.nih.gov/Blast.cgi?CMD=Web&PAGE_TYPE=BlastDocs&DOC_TYPE=BlastHelp)
//!

#![allow(dead_code)]

// TODO: uncomment after TryFrom is stable.
// See issue: https://github.com/rust-lang/rust/issues/33417
// use std::convert::TryFrom;


#[derive(Clone, Debug)]
pub struct Fasta {
    definition: String,
    seq: Vec<u8>,
}

impl Fasta {
    pub fn new() -> Self {
        Self {
            definition: String::new(),
            seq: Vec::new(),
        }
    }

    /// Returns true if the sequence is valid and was updated
    /// 
     /// # Example
     /// ```
     /// let invalid_seq = String::from("ACGCKZ").into_bytes();
     /// let valid_seq = String::from("ACGCKZ").into_bytes();
     /// 
     /// let fasta_seq = Fasta::new();
     /// assert_eq!(fasta_seq.set_seq(invalid_seq), false); // returns false
     /// assert_eq!(fasta_seq.set_seq(valid_seq), true);    // returns true
     ///
     /// ```
    ///
    pub fn set_seq(&mut self, new_seq : Vec<u8>) -> bool {
        if Fasta::valid_seq(&new_seq) {
            self.seq = new_seq;
            return true;
        }
        false
    }

     /// Tests if a sequence is a valid IUB/IUPAC nucleic acid sequence.
     ///
     /// Sequences are expected to be represented in the standard IUB/IUPAC amino acid and 
     /// nucleic acid codes, with these exceptions: lower-case letters are accepted
     /// and are mapped into upper-case; a single hyphen or dash can be used to represent a
     /// gap of indeterminate length; and in amino acide sequences, U and T are acceptable letters
     ///
     /// Since we're using trying to store DNA I believe we should worry more about nucleic 
     /// acid codes than amino acid codes.
     /// Supported codes are:
     /// 
     ///	A  adenosine          C  cytidine             G  guanine
	 ///	T  thymidine          N  A/G/C/T (any)        U  uridine 
	 ///	K  G/T (keto)         S  G/C (strong)         Y  T/C (pyrimidine) 
	 ///	M  A/C (amino)        W  A/T (weak)           R  G/A (purine)        
	 ///    B  G/T/C              D  G/A/T                H  A/C/T      
	 ///	V  G/C/A              -  gap of indeterminate length
     ///
     ///
     /// # Source
     /// `https://blast.ncbi.nlm.nih.gov/Blast.cgi?CMD=Web&PAGE_TYPE=BlastDocs&DOC_TYPE=BlastHelp`
     ///
     /// # Example
     /// ```
     /// let invalid_seq = String::from("ACGCKZ").into_bytes();
     /// let valid_seq = String::from("ACGCKZ").into_bytes();
     /// 
     /// Fasta::valid(&invalid_seq);  // returns false
     /// Fasta::valid(&valid_seq);    // returns true
     ///
     /// ```
     ///
    fn valid_seq(to_test : &Vec<u8>) -> bool {
        for c in to_test {
            let mut lowercase_c :u8 = *c;
            if lowercase_c >= b'a' {
                lowercase_c -= 32;
            }
            match lowercase_c {
                b'A' => {},
                b'C' => {},
                b'G' => {},
                b'T' => {},
                b'N' => {},
                b'U' => {},
                b'K' => {},
                b'S' => {},
                b'Y' => {},
                b'M' => {},
                b'W' => {},
                b'R' => {},
                b'B' => {},
                b'D' => {},
                b'H' => {},
                b'V' => {},
                b'-' => {},
                _ => return false
            };
        }
        true
    }
}

/*
TODO: implement once TryFrom is stable. See issue: https://github.com/rust-lang/rust/issues/33417

#[unstable(feature = "try_from", issue = "33417")]
impl TryFrom<String> for Fasta {
    fn try_from(src : String) -> Result<Fasta,None> {
        let new_seq = src.to_lowercase().into_bytes();
        if !Fasta::valid_seq(&new_seq) {
            // panic!("sequence is not a valid FASTA string");
            return Err("failed");
        }

        let f = Fasta {
            definition: String::new(),
            seq: new_seq,
        };
        Ok(f)
    }
}

impl TryFrom<Vec<u8>> for Fasta {
    fn try_from(new_seq: Vec<u8>) -> Fasta {
        if !Fasta::valid_seq(&new_seq) {
            panic!("sequence is not a valid FASTA string");
        }

        Fasta {
            definition: String::new(),
            seq: new_seq,
        }
    }
}
*/



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fasta_valid_functional() {
        let valid_codes = [b'A', b'C', b'G', b'T', b'N', b'U', b'K', b'S', 
                           b'Y', b'M', b'W', b'R', b'B', b'D', b'H', b'V', b'-'];
        let mut seq: Vec<u8> = Vec::new();
        loop {
            let index :usize = rand::random();
            seq.push(valid_codes[index % valid_codes.len()]);
            if seq.len() >= 1000 {
                break;
            }
        }
        // test a valid
        let mut fasta_seq = Fasta::new();
        assert_eq!(fasta_seq.set_seq(seq.clone()),true);

        // test a valid string with lowercase code
        seq.push(b'a');
        assert_eq!(fasta_seq.set_seq(seq.clone()),true);

        // test an invalid code
        seq.push(b'z');
        assert_eq!(fasta_seq.set_seq(seq.clone()),false);
    }

}
