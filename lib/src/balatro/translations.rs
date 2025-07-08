use std::{collections::HashMap, fs::File, io::Read};

use crate::balatro::translations;

pub struct Translations {
    data: LuaTable   
}

impl Translations {
    pub fn from<R: Read>(mut input: R) -> Self {
        let mut contents = String::new();
        input.read_to_string(&mut contents).expect("Failed to read file");
        
        let data = Self::parse_lua_table(&contents);
        Self { data }
    }

    fn parse_lua_table(input: &str) -> LuaTable {
        let mut chars = input.trim().chars().peekable();
        
        // Skip "return " if present
        if input.trim().starts_with("return ") {
            for _ in 0..7 {
                chars.next();
            }
        }
        
        Self::parse_table_value(&mut chars)
    }

    fn parse_table_value(chars: &mut std::iter::Peekable<std::str::Chars>) -> LuaTable {
        Self::skip_whitespace(chars);
        
        if chars.peek() == Some(&'{') {
            chars.next(); // consume '{'
            let mut map = HashMap::new();
            
            loop {
                Self::skip_whitespace(chars);
                
                if chars.peek() == Some(&'}') {
                    chars.next(); // consume '}'
                    break;
                }
                
                // Parse key
                let key = if chars.peek() == Some(&'"') {
                    Self::parse_string(chars)
                } else if chars.peek() == Some(&'[') {
                    chars.next(); // consume '['
                    Self::skip_whitespace(chars);
                    let key = Self::parse_string(chars);
                    Self::skip_whitespace(chars);
                    if chars.peek() == Some(&']') {
                        chars.next(); // consume ']'
                    }
                    key
                } else {
                    Self::parse_identifier(chars)
                };
                
                Self::skip_whitespace(chars);
                
                // Skip '='
                if chars.peek() == Some(&'=') {
                    chars.next();
                }
                
                Self::skip_whitespace(chars);
                
                // Parse value
                let value = Self::parse_table_value(chars);
                map.insert(key, value);
                
                Self::skip_whitespace(chars);
                
                // Skip comma if present
                if chars.peek() == Some(&',') {
                    chars.next();
                }
            }
            
            LuaTable::Map(map)
        } else if chars.peek() == Some(&'"') {
            LuaTable::Item(Self::parse_string(chars))
        } else {
            LuaTable::Item(Self::parse_identifier(chars))
        }
    }

    fn parse_string(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
        if chars.peek() == Some(&'"') {
            chars.next(); // consume opening quote
        }
        
        let mut result = String::new();
        while let Some(&ch) = chars.peek() {
            if ch == '"' {
                chars.next(); // consume closing quote
                break;
            } else if ch == '\\' {
                chars.next(); // consume backslash
                if let Some(&escaped) = chars.peek() {
                    chars.next();
                    match escaped {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '\\' => result.push('\\'),
                        '"' => result.push('"'),
                        _ => {
                            result.push('\\');
                            result.push(escaped);
                        }
                    }
                }
            } else {
                result.push(ch);
                chars.next();
            }
        }
        
        result
    }

    fn parse_identifier(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
        let mut result = String::new();
        while let Some(&ch) = chars.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                result.push(ch);
                chars.next();
            } else {
                break;
            }
        }
        result
    }

    fn skip_whitespace(chars: &mut std::iter::Peekable<std::str::Chars>) {
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() {
                chars.next();
            } else {
                break;
            }
        }
    }

    pub fn render(&self, path: String, args: Vec<Box<dyn ToString>>) -> Option<Translation> {
        let components = path.split('.').collect::<Vec<&str>>();
        let mut current = &self.data;
        for component in components {
            current = current.get(component)?;
        }
        let name = current.get("name")?.as_item()?;
        let text = current.get("text")?.as_item()?;

        let parsed_name = Self::parse_text(&name, &args);
        let parsed_text = Self::parse_text(&text, &args);

        Some(Translation {
            name: parsed_name,
            text: parsed_text,
        })
    }

    fn parse_text(text: &str, args: &[Box<dyn ToString>]) -> String {
        let mut result = String::new();
        let mut chars = text.chars().peekable();
        
        while let Some(ch) = chars.next() {
            match ch {
                '{' => {
                    while let Some(ch) = chars.next() {
                        if ch == '}' {
                            break;
                        }
                    }
                }
                '#' => {
                    if let Some(digit_ch) = chars.peek() {
                        if digit_ch.is_ascii_digit() {
                            let digit_ch = chars.next().unwrap();
                            if let Some('#') = chars.peek() {
                                chars.next();
                                let index = digit_ch.to_digit(10).unwrap() as usize;
                                if index > 0 && index <= args.len() {
                                    result.push_str(&args[index - 1].to_string());
                                }
                            } else {
                                result.push('#');
                                result.push(digit_ch);
                            }
                        } else {
                            result.push(ch);
                        }
                    } else {
                        result.push(ch);
                    }
                }
                _ => result.push(ch),
            }
        }
        
        result
    }
}

pub struct Translation {
    pub name: String,
    pub text: String
}

pub trait Translatable {
    fn translate(&self, translations: &Translations) -> Translation;
}

enum LuaTable {
    Item(String),
    Map(HashMap<String, LuaTable>)
}

impl LuaTable {
    pub fn get(&self, key: &str) -> Option<&LuaTable> {
        match self {
            LuaTable::Item(_) => None,
            LuaTable::Map(map) => map.get(key),
        }
    }

    pub fn as_item(&self) -> Option<&String> {
        match self {
            LuaTable::Item(item) => Some(item),
            LuaTable::Map(_) => None,
        }
    }
}