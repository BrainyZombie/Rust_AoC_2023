use std::fs;
pub fn file_io<'a, T: 'a, F>(mut args: Box<T>, f: F) -> Result<String, String>
where
    T: Iterator<Item = String>,
    F: FnOnce(&str, Box<dyn Iterator<Item = String> + 'a>) -> Result<String, String>,
{
    let input_file_path = match args.next() {
        Some(s) => s,
        None => return Err(String::from("No input file path specified!")),
    };
    let output_file_path = match args.next() {
        Some(s) => s,
        None => return Err(String::from("No output file path specified!")),
    };
    let input_content = match fs::read_to_string(input_file_path) {
        Ok(s) => s,
        Err(e) => return Err(format!("Read file error: {e}")),
    };
    f(input_content.as_str(), args)
        .map_or_else(Err, |output| {
            Ok((fs::write(output_file_path, &output), output))
        })
        .map_or_else(Err, |(write_result, output)| {
            if let Err(e) = write_result {
                Err(format!("Write file error: {e}"))
            } else {
                Ok(output)
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_input_path() {
        let args = Box::from(vec![].into_iter());
        let result = file_io(args, |s, _| Ok(s.to_string()));
        assert_eq!(result, Err(String::from("No input file path specified!")));
    }
    #[test]
    fn no_output_path() {
        let args = Box::from(vec![String::from("assets/input1.txt")].into_iter());
        let result = file_io(args, |s, _| Ok(s.to_string()));
        assert_eq!(result, Err(String::from("No output file path specified!")));
    }
    #[test]
    fn invalid_input_path() {
        let args = Box::from(
            vec![
                String::from("assets/test_1.txt"),
                String::from("assets/test_output.txt"),
            ]
            .into_iter(),
        );
        let result = file_io(args, |s, _| Ok(s.to_string()));
        assert!(result.is_err_and(|s| s.starts_with("Read file error:")));
    }
    #[test]
    fn internal_error() {
        let args = Box::from(
            vec![
                String::from("assets/test.txt"),
                String::from("assets/test_output.txt"),
            ]
            .into_iter(),
        );
        let result = file_io(args, |s, _| Err(s.to_string()));
        assert_eq!(result, Err(String::from("test")));
    }
    #[test]
    fn success() {
        let args = Box::from(
            vec![
                String::from("assets/test.txt"),
                String::from("assets/test_output.txt"),
            ]
            .into_iter(),
        );
        let result = file_io(args, |s, _| Ok(s.to_string()));
        assert_eq!(result, Ok(String::from("test")));
    }
}
