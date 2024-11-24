use rand::distributions::Distribution;

const BASE64_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

struct Base64;
impl Distribution<u8> for Base64 {
	fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> u8 {
		// SAFETY: 64 is a power of 2 so the range is always valid
		unsafe { *BASE64_CHARSET.get_unchecked((rng.next_u32() >> 26) as usize) }
	}
}

const UID_LEN: usize = 16;

#[derive(Eq, Hash, PartialEq, Default, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Hash([u8; UID_LEN]);

impl Hash {
	pub fn new() -> Self {
      let mut pass = [0; UID_LEN];
		Base64.sample_iter(rand::thread_rng())
			.zip(pass.iter_mut())
			.for_each(|(c, b)| *b = c);
		Self(pass)
	}

	#[inline]
	pub const fn as_str(&self) -> &str {
		// SAFETY: The base64 encoding is always valid utf8
		unsafe { std::str::from_utf8_unchecked(&self.0) }
	}

	#[inline]
	pub fn from(s: &str) -> Option<Self> {
		if s.len() != UID_LEN { return None; }

		if !s.chars().all(|c| BASE64_CHARSET.contains(&(c as u8)))
			{ return None; }

		let mut pass = [0; UID_LEN];
		unsafe {
			std::ptr::copy_nonoverlapping(
				s.as_bytes().as_ptr(), pass.as_mut_ptr(), UID_LEN);
		}
		Some(Self(pass))
	}
}

impl std::fmt::Debug for Hash {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Hash({})", self.as_str())
	}
}
