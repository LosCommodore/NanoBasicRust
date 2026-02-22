 use super::{Line, Result};
    use crate::tokenizer::tokenize;
    #[test]
    fn test_lines() -> Result<()> {
        // -- Read input
        let lines = [
            vec!["10 LET A = (2 + 3)*5 + B*-10".to_string()],
            vec!["20 GOTO 20+B".to_string()],
            vec!["30 GOTOSUB 40".to_string()],
            vec!["40 IF B<>33 THEN GOTO 42".to_string()],
        ];

        for line in &lines {
            println!("{:#?}", line);

            println!("* Tokenizing");
            let tokens = tokenize(&line)?;

            println!("* Parsing");
            let mut iter_token = tokens.iter().peekable();
            let result = Line::parse(&mut iter_token);
            println!("{:?}", result);
        }
        Ok(())
    }