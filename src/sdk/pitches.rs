/*!
The pitch at which a projectile is launch is not always equal to the player's view pitch.
The pitches are stored as radians where -89 is PI radians and 89 is -PI radians (inverted)
*/

#[derive(Copy, Clone, Debug)]
pub struct Pitch {
    pub view: f32,
    pub launch: f32,
}

pub fn launch2view(pitches: &[Pitch], launch: f32) -> f32 {
    if pitches.len() < 2 { return launch;
    }

    let mut low = 0;
    let mut high = pitches.len() - 1;
    while low + 1 != high { let middle = (low + high) / 2; let entry = match pitches.get(middle) {     Some(e) => e,     None => return launch, }; if launch < entry.launch {     high = middle; } else {     low = middle; }
    }

    let low = match pitches.get(low) { Some(e) => e, None => return launch,
    };
    let high = match pitches.get(high) { Some(e) => e, None => return launch,
    };

    let fraction = (launch - low.launch) / (high.launch - low.launch);
    low.view + fraction * (high.view - low.view)
}

pub fn view2launch(pitches: &[Pitch], view: f32) -> f32 {
    if pitches.len() < 2 { return view;
    }

    let mut low = 0;
    let mut high = pitches.len() - 1;
    while low + 1 != high { let middle = (low + high) / 2; let entry = match pitches.get(middle) {     Some(e) => e,     None => return view, }; if view < entry.view {     high = middle; } else {     low = middle; }
    }

    let low = match pitches.get(low) { Some(e) => e, None => return view,
    };
    let high = match pitches.get(high) { Some(e) => e, None => return view,
    };

    let fraction = (view - low.view) / (high.view - low.view);
    low.launch + fraction * (high.launch - low.launch)
}

// Thermite and Frag Grenades
pub static GRENADE_PITCHES: [Pitch; 49] = [
    Pitch { view: -1.5533, launch: -1.3990, }, // 89
    Pitch { view: -1.4837, launch: -1.3267, }, // 85
    Pitch { view: -1.3962, launch: -1.2433, }, // 80
    Pitch { view: -1.3092, launch: -1.1534, }, // 75
    Pitch { view: -1.2217, launch: -1.0779, }, // 70
    Pitch { view: -1.1347, launch: -0.9783, }, // 65
    Pitch { view: -1.0472, launch: -0.8977, }, // 60
    Pitch { view: -0.9602, launch: -0.8104, }, // 55
    Pitch { view: -0.8727, launch: -0.7268, }, // 50
    Pitch { view: -0.7857, launch: -0.6375, }, // 45
    Pitch { view: -0.6981, launch: -0.5439, }, // 40
    Pitch { view: -0.6112, launch: -0.4688, }, // 35
    Pitch { view: -0.5236, launch: -0.3880, }, // 30
    Pitch { view: -0.3491, launch: -0.2050, }, // 25
    Pitch { view: -0.3491, launch: -0.2050, }, // 20
    Pitch { view: -0.2615, launch: -0.1165, }, // 15
    Pitch { view: -0.1746, launch: -0.0421, }, // 10
    Pitch { view: -0.0870, launch: 0.0644, }, //  5
    Pitch { view: -0.0001, launch: 0.1403, }, //  0
    Pitch { view: 0.0875, launch: 0.2358, }, // -5
    Pitch { view: 0.1745, launch: 0.3061, }, //-10
    Pitch { view: 0.2620, launch: 0.3753, }, //-15
    Pitch { view: 0.3490, launch: 0.4684, }, //-20
    Pitch { view: 0.4365, launch: 0.5343, }, //-25
    Pitch { view: 0.5235, launch: 0.6238, }, //-30
    Pitch { view: 0.6110, launch: 0.6865, }, //-35
    Pitch { view: 0.6979, launch: 0.7756, }, //-40
    Pitch { view: 0.7331, launch: 0.7968, }, //-42
    Pitch { view: 0.7682, launch: 0.8341, }, //-44
    Pitch { view: 0.8027, launch: 0.8771, }, //-46
    Pitch { view: 0.8379, launch: 0.9038, }, //-48
    Pitch { view: 0.8727, launch: 0.9382, }, //-50
    Pitch { view: 0.9079, launch: 0.9620, }, //-52
    Pitch { view: 0.9424, launch: 1.0048, }, //-54
    Pitch { view: 0.9775, launch: 1.0333, }, //-56
    Pitch { view: 1.0121, launch: 1.0561, }, //-58
    Pitch { view: 1.0472, launch: 1.0987, }, //-60
    Pitch { view: 1.0824, launch: 1.1217, }, //-62
    Pitch { view: 1.1175, launch: 1.1628, }, //-64
    Pitch { view: 1.1520, launch: 1.1868, }, //-66
    Pitch { view: 1.1866, launch: 1.2239, }, //-68
    Pitch { view: 1.2217, launch: 1.2555, }, //-70
    Pitch { view: 1.2563, launch: 1.2859, }, //-72
    Pitch { view: 1.2913, launch: 1.3156, }, //-74
    Pitch { view: 1.3264, launch: 1.3470, }, //-76
    Pitch { view: 1.3615, launch: 1.3822, }, //-78
    Pitch { view: 1.3973, launch: 1.4108, }, //-80
    Pitch { view: 1.4837, launch: 1.4919, }, //-85
    Pitch { view: 1.5533, launch: 1.5546, }, //-89
];

// Arc Star
pub static ARC_PITCHES: [Pitch; 19] = [
    Pitch { view: -1.5533, launch: -1.5198, },
    Pitch { view: -1.3967, launch: -1.3672, },
    Pitch { view: -1.2222, launch: -1.1974, },
    Pitch { view: -1.0477, launch: -1.0260, },
    Pitch { view: -0.8731, launch: -0.8550, },
    Pitch { view: -0.6986, launch: -0.6848, },
    Pitch { view: -0.5241, launch: -0.5129, },
    Pitch { view: -0.3496, launch: -0.3416, },
    Pitch { view: -0.1572, launch: -0.1484, },
    Pitch { view: 0.0000, launch: 0.0080, },
    Pitch { view: 0.1751, launch: 0.1800, },
    Pitch { view: 0.3496, launch: 0.3520, },
    Pitch { view: 0.5241, launch: 0.5234, },
    Pitch { view: 0.6992, launch: 0.6978, },
    Pitch { view: 0.8727, launch: 0.8710, },
    Pitch { view: 1.0472, launch: 1.0453, },
    Pitch { view: 1.2218, launch: 1.2201, },
    Pitch { view: 1.3963, launch: 1.3956, },
    Pitch { view: 1.5533, launch: 1.5533, },
];

// Grenadier Thermite and Frag Grenades
pub static GRENADIER_GRENADE_PITCHES: [Pitch; 19] = [
    Pitch { view: -1.5533, launch: -1.3991, },
    Pitch { view: -1.3973, launch: -1.2456, },
    Pitch { view: -1.2227, launch: -1.0736, },
    Pitch { view: -1.0477, launch: -0.9010, },
    Pitch { view: -0.8737, launch: -0.7293, },
    Pitch { view: -0.6992, launch: -0.5562, },
    Pitch { view: -0.5247, launch: -0.3832, },
    Pitch { view: -0.3507, launch: -0.2101, },
    Pitch { view: -0.1762, launch: -0.0358, },
    Pitch { view: 0.0000, launch: 0.1406, },
    Pitch { view: 0.1745, launch: 0.2984, },
    Pitch { view: 0.3496, launch: 0.4565, },
    Pitch { view: 0.5247, launch: 0.6157, },
    Pitch { view: 0.6987, launch: 0.7741, },
    Pitch { view: 0.8732, launch: 0.9331, },
    Pitch { view: 1.0477, launch: 1.0924, },
    Pitch { view: 1.2222, launch: 1.2519, },
    Pitch { view: 1.3973, launch: 1.4120, },
    Pitch { view: 1.5533, launch: 1.5548, },
];

// Grenadier Arc Star
pub static GRENADIER_ARC_PITCHES: [Pitch; 19] = [
    Pitch { view: -1.5533, launch: -1.5193, },
    Pitch { view: -1.3973, launch: -1.3657, },
    Pitch { view: -1.2222, launch: -1.1931, },
    Pitch { view: -1.0477, launch: -1.0210, },
    Pitch { view: -0.8731, launch: -0.8485, },
    Pitch { view: -0.6986, launch: -0.6759, },
    Pitch { view: -0.5241, launch: -0.5034, },
    Pitch { view: -0.3496, launch: -0.3297, },
    Pitch { view: -0.1751, launch: -0.1554, },
    Pitch { view: 0.0000, launch: 0.0190, },
    Pitch { view: 0.1763, launch: 0.1916, },
    Pitch { view: 0.3502, launch: 0.3624, },
    Pitch { view: 0.5241, launch: 0.5339, },
    Pitch { view: 0.6992, launch: 0.7065, },
    Pitch { view: 0.8737, launch: 0.8792, },
    Pitch { view: 1.0480, launch: 1.0518, },
    Pitch { view: 1.2220, launch: 1.2251, },
    Pitch { view: 1.3965, launch: 1.3976, },
    Pitch { view: 1.5533, launch: 1.5534, },
];
