use crate::*;

use rustkorean::{compose_korean, create_double_consonant, classify_korean, KoreanType, check_korean, syllable_check, SyllableType};

pub fn change(num: &mut u32) {
	*num += 1;
	println!("{num}");
}

fn input_alphabet(
	kor: char, kor_double: char, uc: char, lc: char, is_typing: &bool, is_korean: &bool, is_capslock: &bool, typed: &mut String, typing: &mut Vec<char>, modifiers_state: &ModifiersState,
) {
	let shift_on = modifiers_state.shift();
	let is_capslock_here = {
		if shift_on {
			!*is_capslock
		} else {
			*is_capslock
		}
	};
	
	//let mut typed_here = typed.to_owned();
	if *is_typing {
		if *is_korean {
			if shift_on {
				typing.push(kor_double);
			} else {
				typing.push(kor);
			};
		} else {
			let chars_with_dc = create_double_consonant(typing.to_owned());
			let chars_with_dv = create_double_vowel(chars_with_dc);
			let composed_string = compose_korean(chars_with_dv);
			*typed += &composed_string;
			*typing = [].to_vec();
			
			//let mut typed_here = typed.to_owned();
			
			if is_capslock_here {
				//typed_here.push(uc);
				typed.push(uc);
			} else {
				//typed_here.push(lc);
				typed.push(lc);
			};
			//*typed = typed_here;
		};
	};
}

pub fn change_typed<T>(typed: &mut String, typing: &mut Vec<char>, event: &Event<T>, is_typing: &mut bool, is_korean: &mut bool, is_capslock: &mut bool, pixels: &mut Pixels) {	
	match event {
		Event::WindowEvent { event, .. } => {
			//println!("{:?}", event);
			match event {
				WindowEvent::KeyboardInput {
					input:
						KeyboardInput {
							virtual_keycode: Some(virtual_code),
							state: ElementState::Pressed,
							modifiers: modifiers_state,
							..
						},
					..
				} => {
					match virtual_code {
						VirtualKeyCode::A => {
							input_alphabet('ㅁ', 'ㅁ', 'A', 'a', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						
						VirtualKeyCode::S => {
							input_alphabet('ㄴ', 'ㄴ', 'S', 's', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::D => {
							input_alphabet('ㅇ', 'ㅇ', 'D', 'd', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::F => {
							input_alphabet('ㄹ', 'ㄹ', 'F', 'f', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::G => {
							input_alphabet('ㅎ', 'ㅎ', 'G', 'g', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::H => {
							input_alphabet('ㅗ', 'ㅗ', 'H', 'h', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::J => {
							input_alphabet('ㅓ', 'ㅓ', 'J', 'j', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::K => {
							input_alphabet('ㅏ', 'ㅏ', 'K', 'k', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::L => {
							input_alphabet('ㅣ', 'ㅣ', 'L', 'l', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						
						VirtualKeyCode::Q => {
							input_alphabet('ㅂ', 'ㅃ', 'Q', 'q', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::W => {
							input_alphabet('ㅈ', 'ㅉ', 'W', 'w', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::E => {
							input_alphabet('ㄷ', 'ㄸ', 'E', 'e', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::R => {
							input_alphabet('ㄱ', 'ㄲ', 'R', 'r', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::T => {
							input_alphabet('ㅅ', 'ㅆ', 'T', 't', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::Y => {
							input_alphabet('ㅛ', 'ㅛ', 'Y', 'y', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::U => {
							input_alphabet('ㅕ', 'ㅕ', 'U', 'u', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::I => {
							input_alphabet('ㅑ', 'ㅑ', 'I', 'i', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::O => {
							input_alphabet('ㅐ', 'ㅒ', 'O', 'o', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::P => {
							input_alphabet('ㅔ', 'ㅖ', 'P', 'p', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						
						VirtualKeyCode::Z => {
							input_alphabet('ㅋ', 'ㅋ', 'Z', 'z', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::X => {
							input_alphabet('ㅌ', 'ㅌ', 'X', 'x', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::C => {
							input_alphabet('ㅊ', 'ㅊ', 'C', 'c', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::V => {
							input_alphabet('ㅍ', 'ㅍ', 'V', 'v', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::B => {
							input_alphabet('ㅠ', 'ㅠ', 'B', 'b', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::N => {
							input_alphabet('ㅜ', 'ㅜ', 'N', 'n', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						VirtualKeyCode::M => {
							input_alphabet('ㅡ', 'ㅡ', 'M', 'm', is_typing, is_korean, is_capslock, typed, typing, modifiers_state);
						},
						
						VirtualKeyCode::Kana  => {
							*is_korean = !*is_korean;
						},
						VirtualKeyCode::Capital  => {
							*is_capslock = !*is_capslock;
						},
						VirtualKeyCode::Return  => {
							*is_typing = !*is_typing;
						},
						
						VirtualKeyCode::Right => {
							let chars_with_dc = create_double_consonant(typing.to_owned());
							let chars_with_dv = create_double_vowel(chars_with_dc);
							let composed_string = compose_korean(chars_with_dv);
							*typed += &composed_string;
							*typing = [].to_vec();
						},
						VirtualKeyCode::Back  => {
							if typing.len() > 0 {
								typing.pop();
							} else {
								typed.pop();
							};
						},
						VirtualKeyCode::Space  => {
							let chars_with_dc = create_double_consonant(typing.to_owned());
							let chars_with_dv = create_double_vowel(chars_with_dc);
							let composed_string = compose_korean(chars_with_dv);
							*typed += &composed_string;
							*typing = [].to_vec();
							
							typed.push(' ');
						},
						
						_ => {
							println!("{virtual_code:?}");
						},
					};
					
					if *is_typing {
						let length = typing.len();
						if length > 2 {
							let last_char = typing[length - 1];
							let pre_last_char = typing[length - 2];
							if syllable_check(last_char) == SyllableType::MiddleVowelLetter &&
								syllable_check(pre_last_char) == SyllableType::BothFirstLastConsonant
							{
								let mut chars = typing.to_owned();
								chars.pop();
								chars.pop();
								let chars_with_dc = create_double_consonant(chars);
								let chars_with_dv = create_double_vowel(chars_with_dc);
								let composed_string = compose_korean(chars_with_dv);
								//println!("composed_string1: {}", composed_string);
								*typed += &composed_string;
								*typing = [pre_last_char, last_char].to_vec();
							};
						};
						
						let length = typing.len();
						if length > 1 {
							let last_char = typing[length - 1];
							
							let chars_with_dc = create_double_consonant(typing.to_owned());
							let chars_with_dv = create_double_vowel(chars_with_dc);
							let composed_string = compose_korean(chars_with_dv);
							//println!("composed_string2: {}", composed_string);
							let composed_chars:Vec<char> = composed_string.chars().collect();
							if composed_chars.len() > 1 {
								typed.push(composed_chars[0]);
								*typing = [last_char].to_vec();
							};
							
							//println!("composed_chars: {:?}", composed_chars);
							//println!("composed_chars[0]: {}", composed_chars[0]);
						};
						
						//println!("{}", typed);
						//println!("{:?}", typing);
						
						let chars_with_dc = create_double_consonant(typing.to_owned());
						let chars_with_dv = create_double_vowel(chars_with_dc);
						let composed_string = compose_korean(chars_with_dv);
						
						let printing = typed.to_owned() + &composed_string;
						println!("{}", printing);
						
					};
				},
				_ => (),
			};
		},
		Event::RedrawRequested(_window_id) => {
			pixels.render();
		},
		_ => (),
	};
}

fn create_double_vowel(chars_vec: Vec<char>) -> Vec<char> {
    let mut result = Vec::new();
    let mut skip_next = false;

    let mut iter = chars_vec.iter().peekable();

    while let Some(&ch) = iter.next() {
        if skip_next {
            skip_next = false;
            continue;
        }

        if let Some(&&next_ch) = iter.peek() {
            match (ch, next_ch) {
                ('ㅗ', 'ㅏ') => {
                    result.push('ㅘ');
                    skip_next = true;
                },
                ('ㅗ', 'ㅐ') => {
                    result.push('ㅙ');
                    skip_next = true;
                },
                ('ㅗ', 'ㅣ') => {
                    result.push('ㅚ');
                    skip_next = true;
                },
                ('ㅜ', 'ㅓ') => {
                    result.push('ㅝ');
                    skip_next = true;
                },
                ('ㅜ', 'ㅔ') => {
                    result.push('ㅞ');
                    skip_next = true;
                },
                ('ㅜ', 'ㅣ') => {
                    result.push('ㅟ');
                    skip_next = true;
                },
                ('ㅡ', 'ㅣ') => {
                    result.push('ㅢ');
                    skip_next = true;
                },
                _ => result.push(ch),
            }
        } else {
            result.push(ch);
        }
    }

    result
}