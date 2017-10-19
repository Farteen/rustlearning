#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


pub trait Summarizable {
    // add code here
    // fn summary(&self) -> String;
    fn summary(&self) -> String {
        format!("Read More...")
    }

}

#[derive(Debug)]
pub struct NewsArticle {
    pub headerline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
	fn summary(&self) -> String {
		format!("{}, by {} ({})",self.headerline,self.author, self.content)
	}
}

#[derive(Debug)]
struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
	fn summary(&self) -> String {
	    format!("")
	}
}