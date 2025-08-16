use std::{collections::HashMap, fs::File, io::Read};

use crate::balatro::translations;

pub struct Translations {
    data: LuaTable   
}

impl Translations {
    pub fn from_string(contents: String) -> Self {
        println!("method called");
        let data = Self::parse_lua_table(&contents);
        Self { data }
    }

    fn parse_lua_table(input: &str) -> LuaTable {
        println!("parse");
        let mut chars = input.trim().chars().peekable();
        
        // Skip "return " if present
        if input.trim().starts_with("return ") {
            for _ in 0..7 {
                chars.next();
            }
        }
        
        println!("parse2");
        Self::parse_table_value(&mut chars)
    }

    fn parse_table_value(chars: &mut std::iter::Peekable<std::str::Chars>) -> LuaTable {
        Self::skip_whitespace(chars);
        
        println!("parse3");
        // EOF protection
        if chars.peek().is_none() {
            return LuaTable::Item(String::new());
        }
        
        if chars.peek() == Some(&'{') {
            chars.next(); // consume '{'
            Self::skip_whitespace(chars);
            
            // Check if this is a list (starts with a value without a key)
            let is_list = chars.peek() == Some(&'"') || 
                         chars.peek() == Some(&'{') || 
                         (chars.peek().map_or(false, |c| c.is_alphanumeric()) && 
                          !Self::has_equals_ahead(chars));
            
            if is_list {
                let mut list = Vec::new();
                
                loop {
                    Self::skip_whitespace(chars);
                    
                    if chars.peek() == Some(&'}') {
                        chars.next(); // consume '}'
                        break;
                    }
                    
                    // EOF protection
                    if chars.peek().is_none() {
                        break;
                    }
                    
                    // Check if we're at a comma (empty list element)
                    if chars.peek() == Some(&',') {
                        chars.next();
                        continue;
                    }
                    
                    // Parse value
                    let value = Self::parse_table_value(chars);
                    list.push(value);
                    
                    Self::skip_whitespace(chars);
                    
                    // Skip comma if present
                    if chars.peek() == Some(&',') {
                        chars.next();
                    }
                }
                
                LuaTable::List(list)
            } else {
                let mut map = HashMap::new();
                
                loop {
                    Self::skip_whitespace(chars);
                    
                    if chars.peek() == Some(&'}') {
                        chars.next(); // consume '}'
                        break;
                    }
                    
                    // EOF protection
                    if chars.peek().is_none() {
                        break;
                    }
                    
                    // Check if we're at a comma (skip it)
                    if chars.peek() == Some(&',') {
                        chars.next();
                        continue;
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
                        let id = Self::parse_identifier(chars);
                        // If we couldn't parse an identifier, skip this character to avoid infinite loop
                        if id.is_empty() && chars.peek().is_some() {
                            chars.next(); // consume the problematic character
                            continue;
                        }
                        id
                    };
                    
                    Self::skip_whitespace(chars);
                    
                    // Skip '='
                    if chars.peek() == Some(&'=') {
                        chars.next();
                    } else if !key.is_empty() {
                        // No '=' found but we have a key - this might be a parsing error
                        // Skip to avoid infinite loop
                        continue;
                    }
                    
                    Self::skip_whitespace(chars);
                    
                    // Parse value
                    let value = Self::parse_table_value(chars);
                    if !key.is_empty() {
                        map.insert(key, value);
                    }
                    
                    Self::skip_whitespace(chars);
                    
                    // Skip comma if present
                    if chars.peek() == Some(&',') {
                        chars.next();
                    }
                }
                
                LuaTable::Map(map)
            }
        } else if chars.peek() == Some(&'"') {
            LuaTable::Item(Self::parse_string(chars))
        } else {
            let id = Self::parse_identifier(chars);
            // If we couldn't parse anything and we're not at EOF, consume the character to avoid infinite loop
            if id.is_empty() && chars.peek().is_some() {
                // Try to parse as a literal value (like numbers, booleans, etc.)
                let mut literal = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == ',' || ch == '}' || ch == ']' || ch.is_whitespace() {
                        break;
                    }
                    literal.push(ch);
                    chars.next();
                }
                LuaTable::Item(literal)
            } else {
                LuaTable::Item(id)
            }
        }
    }
    
    fn has_equals_ahead(chars: &mut std::iter::Peekable<std::str::Chars>) -> bool {
        let mut temp_chars = chars.clone();
        
        // Skip identifier
        while let Some(&ch) = temp_chars.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                temp_chars.next();
            } else {
                break;
            }
        }
        
        // Skip whitespace
        while let Some(&ch) = temp_chars.peek() {
            if ch.is_whitespace() {
                temp_chars.next();
            } else {
                break;
            }
        }
        
        temp_chars.peek() == Some(&'=')
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
                } else {
                    // EOF after backslash
                    result.push('\\');
                    break;
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
        let text_table = current.get("text");
        
        let text = if let Some(text_table) = text_table {
            if let Some(text_item) = text_table.as_item() {
                Some(text_item.clone())
            } else if let Some(text_list) = text_table.as_list() {
                if text_list.is_empty() {
                    None
                } else {
                    Some(text_list.iter()
                        .filter_map(|item| item.as_item())
                        .cloned()
                        .collect::<Vec<String>>()
                        .join(" "))
                }
            } else {
                None
            }
        } else {
            None
        };

        let parsed_name = Self::parse_text(&name, &args);
        let parsed_text = text.as_ref().map(|t| Self::parse_text(t, &args));

        Some(Translation {
            name: parsed_name,
            text: parsed_text,
        })
    }

    pub fn render_single(&self, path: String) -> Option<String> {
        let components = path.split('.').collect::<Vec<&str>>();
        let mut current = &self.data;
        for component in components {
            current = current.get(component)?;
        }
        
        if let Some(list) = current.as_list() {
            if list.is_empty() {
                None
            } else {
                Some(list.iter()
                    .filter_map(|item| item.as_item())
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(" "))
            }
        } else {
            None
        }
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
    pub text: Option<String>
}

pub trait Translatable {
    fn translate(&self, translations: &Translations) -> Translation;
}

enum LuaTable {
    Item(String),
    Map(HashMap<String, LuaTable>),
    List(Vec<LuaTable>)
}

impl LuaTable {
    pub fn get(&self, key: &str) -> Option<&LuaTable> {
        match self {
            LuaTable::Item(_) => None,
            LuaTable::Map(map) => map.get(key),
            LuaTable::List(_) => None,
        }
    }

    pub fn as_item(&self) -> Option<&String> {
        match self {
            LuaTable::Item(item) => Some(item),
            LuaTable::Map(_) => None,
            LuaTable::List(_) => None,
        }
    }
    
    pub fn as_list(&self) -> Option<&Vec<LuaTable>> {
        match self {
            LuaTable::List(list) => Some(list),
            _ => None,
        }
    }
}

#[macro_export]
macro_rules! render {
    ($translations:expr, $path:expr) => {
        {
            let args: Vec<Box<dyn ToString>> = vec![];
            $translations.render($path.to_string(), args)
        }
    };
    ($translations:expr, $path:expr, $($arg:expr),+) => {
        {
            let args: Vec<Box<dyn ToString>> = vec![$(Box::new($arg)),+];
            $translations.render($path.to_string(), args)
        }
    };
}