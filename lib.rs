const FYRS: [&str; 8] = ["ФЫР", "фыр", "ФЫр", "фыР", "ФыР", "фЫр", "фЫР", "Фыр"];

pub fn ser(text: &str) -> String {
	let bytes = text.as_bytes();
	let mut s = "".to_string();
	for i in bytes {
		let a = &(format!("{}{:o}", if i / 100 >= 1 {""} else {"0"}, i.clone()).to_string());
		for j in a.chars(){
			let n: u8 = j.to_digit(10).unwrap() as u8;
			s += &FYRS[n as usize].to_string();
		}
	}
	s
}

pub fn de(text: &str) -> String {
	let mut s = "".to_string();
	let mut a = "".to_string();
	//let mut b = "".to_string();
	let mut vec = Vec::<u8>::new();
	//let mut n: i8 = -1;
	for i in text.chars(){
		a.push(i);
		if a.chars().count() == 3 {
			let index = FYRS.iter().position(|&r| r == a).unwrap();
			s += &index.to_string();
			a = "".to_string()
		}
		if s.chars().count() == 3 {
			vec.push(u8::from_str_radix(&s, 8).unwrap());
			s = "".to_string()
		}
	}
	std::str::from_utf8(&vec).unwrap().to_string()
}