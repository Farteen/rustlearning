use std::ops::Deref;

#[derive(Debug)]
struct Mp3 {
    audio: Vec<u8>,
    artist: Option<String>,
    title: Option<String>
}

impl Deref for Mp3 {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
 		&self.audio
    }
}

fn main() {

	// let mut x = 5;
	// {
	// 	let y = &mut x;
	// 	*y += 1
	// }
	// assert_eq!(6, x);

	let my_favorite_song = Mp3 {
		audio: vec![1, 2, 3],
		artist: Some(String::from("MayDay")),
		title: Some(String::from("Tough"))
	};
	let result = compress_mp3(&my_favorite_song);
	// assert_eq!(vec![1, 2, 3], *my_favorite_song);
}

fn compress_mp3(audio: &[u8]) -> Vec<u8> {	
	vec![1, 2, 3]
}
