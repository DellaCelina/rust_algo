fn main() {
    let out = String::from(
        r#"
         ,r'"7
r`-_   ,'  ,/
 \. ". L_r'
   `~\/
      |
      |
"#,
    );
    print!("{}", out.trim_start_matches('\n'));
}
