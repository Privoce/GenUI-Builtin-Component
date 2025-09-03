use makepad_widgets::{HeapLiveIdPath, LiveId};

pub const LIVE_ID_SEED: u64 = 0xd6e8_feb8_6659_fd93;

pub trait HeapLiveIdPathExp {
    // body.navigation.application_pages.upload_frame.UniqueId 3.s3_list.UniqueId 3.UniqueId 1.share_wrap
    fn contains(&self, child: &HeapLiveIdPath) -> Result<bool, String>;
    fn contains_id(&self, id: &LiveId) -> bool;
    fn to_live_id(&self) -> Vec<LiveId>;
    fn trim_matches(&self, target: &HeapLiveIdPath) -> Vec<LiveId>;
    fn eq(&self, target: &HeapLiveIdPath) -> bool;
    fn is_empty(&self) -> bool;
    fn to_vec_str(&self) -> Vec<String>;
    fn to_string(&self) -> String;
}

impl HeapLiveIdPathExp for HeapLiveIdPath {
    fn to_vec_str(&self) -> Vec<String> {
        format!("{:?}", self)
            .split(".")
            .map(|x| x.to_string())
            .collect()
    }
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
    fn contains(&self, child: &HeapLiveIdPath) -> Result<bool, String> {
        // do format then split by `.`
        let father = format!("{:?}", self);
        let child = format!("{:?}", child);

        let father = father.split('.').collect::<Vec<&str>>();
        let child = child.split('.').collect::<Vec<&str>>();
        // eat one by one till `UniqueId`

        if father.len() < child.len() {
            return Err("father LiveIdPath length smaller than child".to_string());
        }

        let mut flag = true;
        for (index, c_p) in child.iter().enumerate() {
            // let f_p = if father[index].starts_with("UniqueId") {
            //     father[index].trim_start_matches("UniqueId ")
            // } else {
            //     father[index]
            // };
            // dbg!(c_p, f_p);

            if *c_p != father[index] {
                flag = false;
                break;
            }
        }
        Ok(flag)
    }

    /// not complete!!!
    fn to_live_id(&self) -> Vec<LiveId> {
        let path = format!("{:?}", self);
        path.split('.')
            .map(|x| LiveId(from_str_unchecked(x)))
            .collect()
    }

    fn trim_matches(&self, target: &HeapLiveIdPath) -> Vec<LiveId> {
        format!("{:?}", self)
            .trim_start_matches(&format!("{:?}", target))
            .split('.')
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| LiveId(from_str_unchecked(x.trim_matches('.'))))
            .collect()
    }

    fn eq(&self, target: &HeapLiveIdPath) -> bool {
        format!("{:?}", self) == format!("{:?}", target)
    }

    fn contains_id(&self, id: &LiveId) -> bool {
        format!("{:?}", self).contains(&id.to_string())
    }

    fn is_empty(&self) -> bool {
        format!("{:?}", self).is_empty()
    }
}

pub const fn from_bytes(seed: u64, id_bytes: &[u8], start: usize, end: usize) -> u64 {
    let mut x = seed;
    let mut i = start;
    while i < end {
        x = x.overflowing_add(id_bytes[i] as u64).0;
        x ^= x >> 32;
        x = x.overflowing_mul(0xd6e8_feb8_6659_fd93).0;
        x ^= x >> 32;
        x = x.overflowing_mul(0xd6e8_feb8_6659_fd93).0;
        x ^= x >> 32;
        i += 1;
    }
    // mark high bit as meaning that this is a hash id
    (x & 0x7fff_ffff_ffff_ffff) | 0x8000_0000_0000_0000
}

pub const fn from_str_unchecked(id_str: &str) -> u64 {
    let bytes = id_str.as_bytes();
    from_bytes(LIVE_ID_SEED, bytes, 0, bytes.len())
}

pub fn round_to_two_decimals(value: f64) -> f64 {
    (value * 10000.0).round() / 10000.0
}

pub trait LiveIdGenerate {
    fn to_live_id(self) -> LiveId;
}

impl LiveIdGenerate for usize {
    fn to_live_id(self) -> LiveId {
        LiveId(self as u64)
    }
}


pub trait LiveIdExp {
    fn as_slice(&self) -> &[LiveId];
}

impl LiveIdExp for LiveId {
    fn as_slice(&self) -> &[LiveId] {
        std::slice::from_ref(self)
    }
}
