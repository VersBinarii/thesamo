use crate::Error;

#[derive(Debug, Clone)]
pub struct FileTags {
    pub open_tag: String,
    pub close_tag: String,
}

impl FileTags {
    pub fn new(open_tag: &str, close_tag: &str) -> Self {
        Self {
            open_tag: open_tag.to_owned(),
            close_tag: close_tag.to_owned(),
        }
    }
}

/// When the parts of the files are marked with the opening and closing tags
/// this function will extract the body out of every tag block
pub fn extract_body_from_tags(
    file_content: &str,
    tags: &FileTags,
) -> Vec<String> {
    file_content
        .split(&tags.open_tag)
        .skip(1) // skip the first element
        .map(|cap| {
            cap.split(&tags.close_tag)
                .take(1) // skip the last element
                .map(|cc| cc.to_owned())
                .collect()
        })
        .collect()
}

/// We want to replace the body from between the tags and replace it
pub fn replace_in_tags(
    file: &str,
    body: &[String],
    tags: &FileTags,
) -> Result<String, Error> {
    let mut final_file = String::new();
    let mut cursor = 0;
    let mut block_count = 0;

    loop {
        if let Some(open_idx) = file[cursor..].find(&tags.open_tag) {
            // Insert the part of file fefore the opening tag on each iteration
            final_file.push_str(
                &file[cursor..cursor + open_idx + tags.open_tag.len()],
            );
            cursor += open_idx + tags.open_tag.len();
            if let Some(end_idx) = file[cursor..].find(&tags.close_tag) {
                // Insert the part betwen the tags that needs to be replaced
                final_file.push_str(&body[block_count]);
                final_file.push_str(&tags.close_tag);
                cursor += end_idx + tags.close_tag.len();
                block_count += 1;
            } else {
                return Err(Error::ReplaceTags(String::from(
                    "Failed to find closing tag.",
                )));
            }
        } else {
            // Finally insert tail of the file after the last closing tag
            final_file.push_str(&file[cursor..]);

            // TODO:Check for mismatch between actual blocks and expected
            break;
        }
    }

    Ok(final_file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_extract_body_from_tags() {
        let open_tag = "%%>";
        let close_tag = "<%%";
        let test_file_content = r#"
   some content that should be ommited
%%>
the part to be extracted
    with      some     spaces

and ending.
<%%

and a tail of the file
"#;

        let expected = r#"
the part to be extracted
    with      some     spaces

and ending.
"#
        .to_owned();

        let tags = FileTags::new(open_tag, close_tag);
        let extra = extract_body_from_tags(test_file_content, &tags);
        assert!(extra == vec![expected])
    }

    #[test]
    fn should_extract_all_bodies_from_tags() {
        let open_tag = "%%>";
        let close_tag = "<%%";
        let test_file_content = r#"
some content that should be ommited
%%>
the part to be extracted
    with      some     spaces

and ending.
<%%

some other config items

%%>
interesting
part of file
<%%

Tail of file
"#;

        let expected1 = r#"
the part to be extracted
    with      some     spaces

and ending.
"#
        .to_owned();

        let expected2 = r#"
interesting
part of file
"#
        .to_owned();

        let tags = FileTags::new(open_tag, close_tag);
        let extra = extract_body_from_tags(test_file_content, &tags);
        println!("{:?}", extra);
        assert!(extra == vec![expected1, expected2])
    }

    #[test]
    fn should_replace_all_tag_content_with_appropriate_replacement() {
        let open_tag = "%%>";
        let close_tag = "<%%";
        let test_file_content = r#"
some content that should be ommited
%%>
the part to be extracted
    with      some     spaces

and ending.
<%%

some other config items

%%>
interesting
part of file
<%%

Tail of file
"#;

        let expected = r#"
some content that should be ommited
%%>
just this
<%%

some other config items

%%>
and this
<%%

Tail of file
"#
        .to_owned();
        let replacement1 = "\njust this\n".to_owned();
        let replacement2 = "\nand this\n".to_owned();

        let tags = FileTags::new(open_tag, close_tag);

        let result = replace_in_tags(
            test_file_content,
            &[replacement1, replacement2],
            &tags,
        );

        let res = result.unwrap();

        println!("{}", res);
        assert_eq!(expected, res);
    }
}
