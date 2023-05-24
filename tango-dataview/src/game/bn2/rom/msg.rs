pub fn parser(charset: &[String]) -> crate::msg::Parser {
    crate::msg::Parser::builder()
        .with_ignore_unknown(true)
        .add_eof_rule(b"\xe7")
        .add_charset_rules(charset, 0xe5)
        .add_text_rule(b"\xe8", "\n")
        .add_command_rule(b"\xeb", 0)
        .add_command_rule(b"\xec\x00", 1)
        .add_command_rule(b"\xf1\x02", 0)
        .add_command_rule(b"\xf1\x03", 0)
        .build()
}
