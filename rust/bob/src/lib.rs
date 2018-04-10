pub fn reply(message: &str) -> &str {   
    let msg = message.trim();

    return match msg {
        o if o.is_empty() => "Fine. Be that way!",
        o if is_yell(o) && o.ends_with("?") => "Calm down, I know what I'm doing!",
        o if is_yell(o) => "Whoa, chill out!",
        o if o.ends_with("?") => "Sure.",
        _ => "Whatever.",
    };
}

fn is_yell(message: &str) -> bool {
    let temp: String = message.chars().filter(|c| c.is_alphabetic()).collect::<String>() ;
    !temp.is_empty() && temp.chars().all(|c| c.is_uppercase())
}